package models

import (
	"time"
	"user-auth/internal/db"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/v2/mongo"
	"go.mongodb.org/mongo-driver/v2/mongo/options"
	"golang.org/x/net/context"
)

type Session struct {
	UserId primitive.ObjectID `bson:"userId"`
	Token  string             `bson:"token"`
	Expr   time.Time          `bson:"expr"`
}

type SessionModel struct {
	Model
}

func NewSessionModel(ctx context.Context, client *db.MongoClient) *SessionModel {
	sessionModel := SessionModel{
		Model: Model{
			Name:       "sessions",
			Collection: nil,
			IndexModels: []mongo.IndexModel{
				{
					Keys:    bson.M{"token": 1},
					Options: options.Index().SetUnique(true),
				},
			},
		},
	}
	sessionModel.SetIndexes(ctx, client)
	return &sessionModel
}
