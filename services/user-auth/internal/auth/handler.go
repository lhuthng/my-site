package auth

import (
	"context"
	"fmt"
	"os"
	"time"

	// execute `protoc --go_out=. --go-grpc_out=. --proto_path=. ./proto/user.proto` to hide red curly lines
	// But it's fine not to do that, docker will generate the files
	"user-auth/internal/models"
	pb "user-auth/proto"

	"github.com/golang-jwt/jwt/v5"
	"go.mongodb.org/mongo-driver/v2/bson"
)

var jwtSecret = os.Getenv("JWT_SECRET")
var bJwtSecret = []byte(jwtSecret)

type Server struct {
	pb.UnimplementedUserAuthServiceServer
}

type ActivationClaim struct {
	Email string `json:"email"`
	jwt.RegisteredClaims
}

type SessionClaim struct {
	Session string `json:"session"`
	jwt.RegisteredClaims
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
		return "", err
	} else {
		return tokenString, nil
	}
}

func DecodeJWT(tokenString string, claimType jwt.Claims) (*jwt.MapClaims, error) {
	if jwtSecret == "" {
		return nil, fmt.Errorf("jwt_secret is not found")
	}
	token, err := jwt.Parse(tokenString, func(token *jwt.Token) (interface{}, error) {
		return bJwtSecret, nil
	}, jwt.WithValidMethods([]string{jwt.SigningMethodHS256.Alg()}))
	if err != nil {
		return nil, fmt.Errorf("%s", err)
	}
	fmt.Println(token)
	if claims, ok := token.Claims.(jwt.MapClaims); ok {
		fmt.Println(claims)
		return &claims, nil
	}
	return nil, fmt.Errorf("Error")
}

func (s *Server) CreateLocalUser(ctx context.Context, req *pb.CreateLocalUserRequest) (*pb.CreateUserResponse, error) {
	user := models.User{
		Username:   req.Username,
		Password:   &req.Password,
		Email:      &req.Email,
		AuthMethod: "local",
		Activated:  false,
	}
	_, err := models.InsertUser(user)
	if err != nil {
		return nil, fmt.Errorf("failed to insert user: %w", err)
	}
	tokenStr, err := EncodeJWT(req.Email)
	if err == nil {
		return &pb.CreateUserResponse{
			UserId:          req.Username,
			ActivationToken: tokenStr,
			Message:         "success",
		}, nil
	}
	return nil, fmt.Errorf("failed to encode %w", err)
}

func (s *Server) ActivateUser(ctx context.Context, req *pb.ActivateUserRequest) (*pb.ActivateUserResponse, error) {
	decodedClaims, err := DecodeJWT(req.ActivationToken, &ActivationClaim{})
	if decodedClaims == nil || err != nil {
		return nil, err
	}
	email := (*decodedClaims)["payload"]
	if _, err := models.UpdateOneUser(bson.D{{"email", email}}, bson.D{{"$set", bson.D{{"activated", true}}}}); err == nil {
		return nil, err
	}
	return &pb.ActivateUserResponse{
		Email:   email.(string),
		Message: "success",
	}, nil
}
