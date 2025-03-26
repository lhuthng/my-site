package interceptors

import (
	"context"
	"fmt"
	"regexp"
	"unicode"

	"google.golang.org/grpc"
)

var protectedMethods = map[string]bool{
	"/proto.UserAuthService/createLocalUser": true,
	"/proto.UserAuthService/createOAuthUser": true,
}

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

func UserDetailInterceptors(
	ctx context.Context,
	req any,
	info *grpc.UnaryServerInfo,
	handler grpc.UnaryHandler,
) (any, error) {
	if _, needValidateUser := protectedMethods[info.FullMethod]; !needValidateUser {
		return handler(ctx, req)
	}
	user, ok := req.(LocalUser)
	if ok {
		if err := validateUsername(user.GetUsername()); err != nil {
			return nil, fmt.Errorf("invalid username, %w", err)
		}
		if err := validatePassword(user.GetPassword()); err != nil {
			return nil, fmt.Errorf("invalid password, %w", err)
		}
		if err := validateEmail(user.GetEmail()); err != nil {
			return nil, fmt.Errorf("invalid email, %w", err)
		}
	} else if _, ok := req.(OAuthUser); ok {
		// TODO:
		return nil, fmt.Errorf("not implemented")
	} else {
		return nil, fmt.Errorf("invalid request")
	}
	return handler(ctx, req)
}
