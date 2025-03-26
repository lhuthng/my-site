package models

import (
	"user-auth/internal/db"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
	"go.mongodb.org/mongo-driver/v2/mongo/options"
	"golang.org/x/net/context"
)

type User struct {
	Username   string  `bson:"username"`
	Email      *string `bson:"email,omitempty"`
	Password   *string `bson:"password,omitempty"`
	AuthMethod string  `bson:"authMethod"`
	AuthId     *string `bson:"authId,omitempty"`
	Activated  bool    `bson:"activated"`
}

type UserModel struct {
	Model
}

func NewUserModel(ctx context.Context, client *db.MongoClient) *UserModel {
	userModel := UserModel{
		Model: Model{
			Name:       "users",
			Collection: nil,
			IndexModels: []mongo.IndexModel{
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
		},
	}
	userModel.SetIndexes(ctx, client)
	return &userModel
}

func (userModel *UserModel) FindOne(ctx context.Context, filter any) (*User, error) {
	var user User
	if err := userModel.Model.FindOne(ctx, filter, &user); err != nil {
		return nil, err
	} else {
		return &user, nil
	}
}
