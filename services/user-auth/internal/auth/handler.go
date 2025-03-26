package auth

import (
	"context"
	"fmt"
	"os"
	"time"

	// execute `protoc --go_out=. --go-grpc_out=. --proto_path=. ./proto/user.proto` to hide red curly lines
	// But it's fine not to do that, docker will generate the files
	"user-auth/internal/db"
	"user-auth/internal/models"
	pb "user-auth/proto/user"

	"github.com/golang-jwt/jwt/v5"
	"go.mongodb.org/mongo-driver/v2/bson"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

var jwtSecret = os.Getenv("JWT_SECRET")
var bJwtSecret = []byte(jwtSecret)

type Server struct {
	JwtSecret string
	BwtSecret []byte
	UserModel models.UserModel
	pb.UnimplementedUserAuthServiceServer
}

func NewServer(ctx context.Context, secret string, client *db.MongoClient) *Server {
	return &Server{
		JwtSecret: secret,
		BwtSecret: []byte(secret),
		UserModel: *models.NewUserModel(ctx, client),
	}
}

func EncodeJWT(payload string) (string, error) {
	if jwtSecret == "" {
		return "", fmt.Errorf("jwt_secret is not found")
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"payload": payload,
		"exp":     jwt.NewNumericDate(time.Now().Add(24 * time.Hour)),
	})

	if tokenString, err := token.SignedString(bJwtSecret); err != nil {
		return "", fmt.Errorf("failed to sign token - %v", err)
	} else {
		return tokenString, nil
	}
}

func DecodeJWT(tokenString string) (*jwt.MapClaims, error) {
	if jwtSecret == "" {
		return nil, fmt.Errorf("jwt_secret is not found")
	}
	token, err := jwt.Parse(tokenString, func(token *jwt.Token) (any, error) {
		return bJwtSecret, nil
	}, jwt.WithValidMethods([]string{jwt.SigningMethodHS256.Alg()}))
	if err != nil {
		return nil, err
	}
	if claims, ok := token.Claims.(jwt.MapClaims); ok {
		return &claims, nil
	}
	return nil, fmt.Errorf("cannot extract claims")
}

func (s *Server) CreateLocalUser(ctx context.Context, req *pb.CreateLocalUserRequest) (*pb.CreateUserResponse, error) {
	user := models.User{
		Username:   req.Username,
		Password:   &req.Password,
		Email:      &req.Email,
		AuthMethod: "local",
		Activated:  false,
	}
	insertedResult, err := s.UserModel.InsertOne(ctx, &user)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to insert user - %v", err)
	}
	tokenStr, err := EncodeJWT(req.Email)
	if err == nil {
		return &pb.CreateUserResponse{
			UserId:          insertedResult.InsertedID.(bson.ObjectID).Hex(),
			ActivationToken: tokenStr,
		}, nil
	}
	return nil, status.Errorf(codes.Internal, "failed to encode token - %v", err)
}

func (s *Server) ActivateUser(ctx context.Context, req *pb.ActivateUserRequest) (*pb.ActivateUserResponse, error) {
	decodedClaims, err := DecodeJWT(req.ActivationToken)
	if decodedClaims == nil || err != nil {
		return nil, status.Errorf(codes.InvalidArgument, "failed to decode token - %v", err)
	}

	email := (*decodedClaims)["payload"].(string)
	exp := (*decodedClaims)["exp"].(float64)
	if !time.Now().Before(time.Unix(int64(exp), 0)) {
		return nil, status.Errorf(codes.DeadlineExceeded, "failed to activate user - token is expired")
	}

	filter := bson.M{"email": email}
	update := bson.M{"$set": bson.M{"activated": true}}
	abst, err := s.UserModel.FindOne(ctx, filter)
	if err != nil {
		return nil, status.Errorf(codes.NotFound, "failed to activate user - %v", err)
	}
	if abst == nil {
		return nil, status.Errorf(codes.InvalidArgument, "failed to activate user - no user found")
	}
	user, ok := abst.(*models.User)
	if !ok {
		return nil, status.Errorf(codes.Internal, "failed activate user - wrong type of model")
	}
	if user.Activated {
		return nil, status.Errorf(codes.AlreadyExists, "failed to activate user - user is already activated")
	}
	if _, err = s.UserModel.UpdateOne(ctx, filter, update); err != nil {
		return nil, status.Errorf(codes.Internal, "failed to activate user - %v", err)
	}
	return &pb.ActivateUserResponse{
		Email: email,
	}, nil
}
