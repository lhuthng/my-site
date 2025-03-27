package auth

import (
	"context"
	"crypto/hmac"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"os"
	"strings"
	"time"

	// execute `protoc --go_out=. --go-grpc_out=. --proto_path=. ./proto/user.proto` to hide red curly lines
	// But it's fine not to do that, docker will generate the files
	"user-auth/internal/db"
	"user-auth/internal/models"
	pb "user-auth/proto/user"

	"github.com/golang-jwt/jwt/v5"
	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo/options"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
)

var jwtSecret = os.Getenv("JWT_SECRET")
var bJwtSecret = []byte(jwtSecret)

type Server struct {
	JwtSecret    string
	BwtSecret    []byte
	UserModel    models.UserModel
	SessionModel models.SessionModel
	pb.UnimplementedUserAuthServiceServer
}

func NewServer(ctx context.Context, secret string, client *db.MongoClient) *Server {
	return &Server{
		JwtSecret:    secret,
		BwtSecret:    []byte(secret),
		UserModel:    *models.NewUserModel(ctx, client),
		SessionModel: *models.NewSessionModel(ctx, client),
	}
}

func EncodeJWT(payload string, delay int) (string, time.Time, error) {
	if jwtSecret == "" {
		return "", time.Now(), fmt.Errorf("jwt_secret is not found")
	}
	expr := jwt.NewNumericDate(time.Now().Add(time.Duration(delay) * time.Hour))
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"payload": payload,
		"exp":     expr,
	})

	if tokenString, err := token.SignedString(bJwtSecret); err != nil {
		return "", time.Now(), fmt.Errorf("failed to sign token - %v", err)
	} else {
		return tokenString, time.Now(), nil
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

func GenerateRandomString(n int) (string, error) {
	bytes := make([]byte, n)
	_, err := rand.Read(bytes)
	if err != nil {
		return "", err
	}
	return hex.EncodeToString(bytes), nil
}

func CreateSessionToken(secretKey string, delay int) (string, time.Time, error) {
	sessionID, err := GenerateRandomString(32)
	if err != nil {
		return "", time.Now(), err
	}

	h := hmac.New(sha256.New, []byte(secretKey))
	h.Write([]byte(sessionID))
	hashedToken := hex.EncodeToString(h.Sum(nil))
	expr := jwt.NewNumericDate(time.Now().Add(time.Duration(delay) * time.Hour)).Time

	return hashedToken, expr, nil
}

func CreateError(code codes.Code, mainStr string, subStr string) error {
	return status.Errorf(code, "%s - %s", mainStr, subStr)
}

func (s *Server) CreateLocalUser(ctx context.Context, req *pb.CreateLocalUserRequest) (*pb.CreateUserResponse, error) {
	const errorString = "failed to create user"
	user := models.User{
		Username:   req.Username,
		Password:   &req.Password,
		Email:      &req.Email,
		AuthMethod: "local",
		Activated:  false,
	}
	insertedResult, err := s.UserModel.InsertOne(ctx, &user)
	if err != nil {
		return nil, CreateError(codes.Internal, errorString, err.Error())
	}
	tokenStr, _, err := EncodeJWT(req.Email, 24)
	if err == nil {
		return &pb.CreateUserResponse{
			UserId:          insertedResult.InsertedID.(bson.ObjectID).Hex(),
			ActivationToken: tokenStr,
		}, nil
	}
	return nil, CreateError(codes.Internal, errorString, err.Error())
}

func (server *Server) ActivateUser(ctx context.Context, req *pb.ActivateUserRequest) (*pb.ActivateUserResponse, error) {
	const errorString = "failed to activate user"
	decodedClaims, err := DecodeJWT(req.ActivationToken)
	if decodedClaims == nil || err != nil {
		return nil, CreateError(codes.Internal, errorString, err.Error())
	}

	email := (*decodedClaims)["payload"].(string)
	exp := (*decodedClaims)["exp"].(float64)
	if !time.Now().Before(time.Unix(int64(exp), 0)) {
		return nil, CreateError(codes.DeadlineExceeded, errorString, "token expired")
	}

	filter := bson.M{"email": email}
	update := bson.M{"$set": bson.M{"activated": true}}
	abst, err := server.UserModel.FindOneD(ctx, filter)
	if err != nil {
		return nil, CreateError(codes.NotFound, errorString, err.Error())
	}
	if abst == nil {
		return nil, CreateError(codes.InvalidArgument, errorString, "failed to activate user - no user found")
	}
	fmt.Println(abst)
	if user, ok := abst.(*models.User); !ok {
		return nil, CreateError(codes.Internal, errorString, "failed activate user - wrong type of model")
	} else if user.Activated {
		return nil, CreateError(codes.AlreadyExists, errorString, "failed to activate user - user is already activated")
	}
	if _, err = server.UserModel.UpdateOne(ctx, &filter, &update); err != nil {
		return nil, CreateError(codes.Internal, errorString, err.Error())
	}
	return &pb.ActivateUserResponse{
		Email: email,
	}, nil
}

func (server *Server) LoginLocal(ctx context.Context, req *pb.LoginLocalRequest) (*pb.LoginResponse, error) {
	const errorString = "failed to login"
	filter := bson.M{"email": req.Email}
	abst, err := server.UserModel.FindOneD(ctx, filter)
	if err != nil {
		return nil, CreateError(codes.NotFound, errorString, err.Error())
	}
	if abst == nil {
		return nil, CreateError(codes.InvalidArgument, errorString, "no user found")
	}

	user, ok := abst.(*models.User)
	if !ok {
		return nil, CreateError(codes.Internal, errorString, "wrong type of model")
	}
	if !user.Activated {
		return nil, CreateError(codes.AlreadyExists, errorString, "user is not activated")
	}
	if user.Password == nil {
		return nil, CreateError(codes.Internal, errorString, "local user has no password")
	}

	compareResult, err := user.ComparePassword(req.Password)
	if err != nil {
		return nil, CreateError(codes.Internal, errorString, err.Error())
	}
	if !compareResult {
		return nil, CreateError(codes.Aborted, errorString, "password is incorrect")
	}
	tokenStr, expr, err := CreateSessionToken(user.Id.Hex(), 1)
	if err != nil {
		return nil, CreateError(codes.Internal, errorString, "failed to encode")
	}

	filter = bson.M{
		"userId": user.Id,
	}
	update := bson.M{
		"$set": bson.M{
			"token": tokenStr,
			"expr":  expr,
		},
	}
	allowUpsert := options.UpdateOne().SetUpsert(true)
	server.SessionModel.UpdateOne(ctx, &filter, &update, allowUpsert)
	return &pb.LoginResponse{
		UserId:              user.Id.Hex(),
		AuthenticationToken: tokenStr,
		ExpiresAt:           expr.Unix(),
	}, nil
}

func CollectAuthenticationToken(ctx context.Context) (string, error) {
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		return "", fmt.Errorf("session token not found")
	}
	authHeaders := md.Get("authentication")
	if len(authHeaders) == 0 {
		return "", fmt.Errorf("authentication token not provided")
	}

	return strings.TrimPrefix(authHeaders[0], "Bearer "), nil
}

func (server *Server) RenewTokenRequest(ctx context.Context, req *pb.RenewTokenRequest) (*pb.RenewTokenResponse, error) {
	const errorString = "failed to login"

	token, err := CollectAuthenticationToken(ctx)
	if err != nil {
		return nil, CreateError(codes.Unauthenticated, errorString, err.Error())
	}

	filter := bson.M{
		"token": token,
	}
	abst, err := server.SessionModel.FindOneD(ctx, filter)
	if err != nil {
		return nil, CreateError(codes.Unauthenticated, errorString, "session not found")
	}
	session, ok := abst.(models.Session)
	if !ok {
		return nil, CreateError(codes.Internal, errorString, "wrong type of model")
	}
	if session.UserId.Hex() != req.UserId {
		return nil, CreateError(codes.Unauthenticated, errorString, "invalid user id")
	}
	if session.Token != token {
		return nil, CreateError(codes.Unauthenticated, errorString, "invalid or expired token")
	}
	decodedClaims, err := DecodeJWT(token)
	if decodedClaims == nil || err != nil {
		return nil, CreateError(codes.Internal, errorString, err.Error())
	}

	if (*decodedClaims)["payload"].(string) != "auth-service" {
		return nil, CreateError(codes.Unauthenticated, errorString, "invalid token")
	}
	exp := (*decodedClaims)["exp"].(float64)
	if !time.Now().Before(time.Unix(int64(exp), 0)) {
		return nil, CreateError(codes.Unauthenticated, errorString, "token expired")
	}

	const delay = 1
	tokenStr, expr, err := CreateSessionToken(req.UserId, delay)
	if err != nil {
		return nil, CreateError(codes.Internal, errorString, "failed to encode")
	}
	filter = bson.M{
		"userId": req.UserId,
	}
	update := bson.M{
		"$set": bson.M{
			"token": tokenStr,
			"expr":  expr,
		},
	}
	allowUpsert := options.UpdateOne().SetUpsert(true)
	server.SessionModel.UpdateOne(ctx, &filter, &update, allowUpsert)
	return &pb.RenewTokenResponse{
		AuthenticationToken: tokenStr,
		ExpiresAt:           expr.Unix(),
	}, nil
}

func (server *Server) ValidateToken(ctx context.Context, req *pb.ValidateTokenRequest) (*pb.ValidateTokenResponse, error) {
	const errorString = "failed to validate token"
	token, err := CollectAuthenticationToken(ctx)
	if err != nil {
		return nil, CreateError(codes.Unauthenticated, errorString, err.Error())
	}

	filter := bson.M{
		"token": token,
	}
	if _, err = server.SessionModel.FindOne(ctx, filter); err != nil {
		return nil, CreateError(codes.Unauthenticated, errorString, err.Error())
	}
	return &pb.ValidateTokenResponse{}, nil
}
