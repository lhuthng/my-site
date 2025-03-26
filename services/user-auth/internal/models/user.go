package models

import (
	"fmt"
	"user-auth/internal/db"

	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
	"go.mongodb.org/mongo-driver/v2/mongo/options"
	"golang.org/x/crypto/bcrypt"
	"golang.org/x/net/context"
)

type User struct {
	Id         bson.ObjectID `bson:"_id,omitempty"`
	Username   string        `bson:"username"`
	Email      *string       `bson:"email,omitempty"`
	Password   *string       `bson:"password,omitempty"`
	AuthMethod string        `bson:"authMethod"`
	AuthId     *string       `bson:"authId,omitempty"`
	Activated  bool          `bson:"activated"`
}

type UserModel struct {
	Model
}

func NewUserModel(ctx context.Context, client *db.MongoClient) *UserModel {
	userModel := &UserModel{
		Model: *NewModel(
			"users",
			nil,
			[]mongo.IndexModel{
				{
					Keys:    bson.M{"username": 1},
					Options: options.Index().SetUnique(true),
				},
				{
					Keys:    bson.M{"email": 1},
					Options: options.Index().SetUnique(true).SetSparse(true),
				},
				{
					Keys:    bson.M{"authId": 1},
					Options: options.Index().SetUnique(true).SetSparse(true),
				},
			},
		),
	}
	userModel.SetIndexes(ctx, client)
	userModel.Getter = userModel
	return userModel
}

func HashPassword(password string) (string, error) {
	hashedPassword, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.DefaultCost)
	if err != nil {
		return "", err
	}
	return string(hashedPassword), nil
}

func (user *User) ComparePassword(password string) (bool, error) {
	if user == nil {
		return false, fmt.Errorf("user is undefined")
	}
	if user.Password == nil {
		return false, fmt.Errorf("user has no password: %s", user.AuthMethod)
	}
	if err := bcrypt.CompareHashAndPassword([]byte(*user.Password), []byte(password)); err != nil {
		return false, err
	}
	return true, nil
}

func (userModel *UserModel) GetSchema() any {
	return &User{}
}

func (userModel *UserModel) GetPreUpdate(update *bson.M) *bson.M {
	if update == nil {
		return nil
	}
	if set, ok := (*update)["$set"].(bson.M); ok {
		if password, exists := set["password"]; exists {
			if hashedPassword, err := HashPassword(password.(string)); err == nil {
				set["password"] = hashedPassword
				(*update)["$set"] = set
			}
		}
	}
	return update
}

func (userModel *UserModel) GetPreInsert(record any) any {
	if user, ok := record.(*User); ok && user.Password != nil {
		if hashedPassword, err := HashPassword(*user.Password); err == nil {
			user.Password = &hashedPassword
		}
		return user
	}
	return record
}
