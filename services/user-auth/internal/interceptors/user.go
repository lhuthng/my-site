package interceptors

import (
	"context"
	"fmt"
	"regexp"
	"unicode"

	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func validateUsername(username string) error {
	if len(username) < 3 || len(username) > 20 {
		return fmt.Errorf("username must be between 3 and 20 characters")
	}
	return nil
}

func validatePassword(password string) error {
	if len(password) < 8 {
		return fmt.Errorf("password must be at least 8 characters long")
	}

	hasNumber := false
	hasSpecial := false

	for _, char := range password {
		switch {
		case unicode.IsDigit(char):
			hasNumber = true
		case unicode.IsPunct(char) || unicode.IsSymbol(char):
			hasSpecial = true
		}
	}

	if !hasNumber {
		return fmt.Errorf("password must contain at least one number")
	}
	if !hasSpecial {
		return fmt.Errorf("password must contain at least one special character")
	}

	return nil
}

func validateEmail(email string) error {
	emailRegex := `^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$`
	re := regexp.MustCompile(emailRegex)
	if !re.MatchString(email) {
		return fmt.Errorf("invalid email format")
	}
	return nil
}

type LocalUser interface {
	GetUsername() string
	GetEmail() string
	GetPassword() string
}

type OAuthUser interface {
	GetAuthId() string
	GetAuthMethod() string
}

var userDetailMethods = map[string]bool{
	"/proto.UserAuthService/createLocalUser": true,
	"/proto.UserAuthService/createOAuthUser": true,
}

func UserDetailInterceptors(
	ctx context.Context,
	req any,
	info *grpc.UnaryServerInfo,
	handler grpc.UnaryHandler,
) (any, error) {
	if _, needValidateUser := userDetailMethods[info.FullMethod]; !needValidateUser {
		return handler(ctx, req)
	}

	switch user := req.(type) {
	case LocalUser:
		if err := validateUsername(user.GetUsername()); err != nil {
			return nil, status.Errorf(codes.InvalidArgument, "invalid username, %v", err)
		}
		if err := validatePassword(user.GetPassword()); err != nil {
			return nil, status.Errorf(codes.InvalidArgument, "invalid password, %v", err)
		}
		if err := validateEmail(user.GetEmail()); err != nil {
			return nil, status.Errorf(codes.InvalidArgument, "invalid email, %v", err)
		}
		return handler(ctx, req)
	case OAuthUser:
		// TODO: Implement OAuthUser
		return nil, status.Errorf(codes.Internal, "not implemented")
	default:
		return nil, status.Errorf(codes.InvalidArgument, "invalid request")
	}
}
