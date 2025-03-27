package models

import (
	"time"
	"user-auth/internal/db"

	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
	"go.mongodb.org/mongo-driver/v2/mongo/options"
	"golang.org/x/net/context"
)

type Session struct {
	Id     bson.ObjectID `bson:"_id,omitempty"`
	UserId bson.ObjectID `bson:"userId"`
	Token  string        `bson:"token"`
	Expr   time.Time     `bson:"expr"`
}

type SessionModel struct {
	Model
}

func NewSessionModel(ctx context.Context, client *db.MongoClient) *SessionModel {
	sessionModel := &SessionModel{
		Model: Model{
			Name:       "sessions",
			Collection: nil,
			IndexModels: []mongo.IndexModel{
				{
					Keys:    bson.M{"userId": 1},
					Options: options.Index().SetUnique(true),
				},
				{
					Keys:    bson.M{"expiresAt": 1},
					Options: options.Index().SetExpireAfterSeconds(0),
				},
			},
		},
	}
	sessionModel.SetIndexes(ctx, client)
	sessionModel.Getter = sessionModel
	return sessionModel
}

func (sessionModel *SessionModel) GetSchema() any {
	return &Session{}
}
