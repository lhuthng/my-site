package models

import (
	"context"
	"fmt"
	"user-auth/internal/db"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/v2/mongo"
	"go.mongodb.org/mongo-driver/v2/mongo/options"
)

type User struct {
	Id         primitive.ObjectID `bson:"_id,omitempty"`
	Username   string             `bson:"username"`
	Email      *string            `bson:"email,omitempty"`
	Password   *string            `bson:"password,omitempty"`
	AuthMethod string             `bson:"authMethod"`
	AuthId     *string            `bson:"authId,omitempty"`
	Activated  bool               `bson:"activated"`
}

var collection *db.MongoCollection = nil

func SetUsersCollection(ctx context.Context, client db.MongoClient) error {
	fmt.Println("B")
	collection = client.Use("users")
	indexModels := []mongo.IndexModel{
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
	}

	_, err := collection.Indexes().CreateMany(ctx, indexModels)
	if err != nil {
		return fmt.Errorf("failed to create indexes: %w", err)
	}

	fmt.Println("Indexes created successfully.")
	return nil
}

func InsertUser(user User) (*mongo.InsertOneResult, error) {
	if collection == nil {
		return nil, fmt.Errorf("collection is not connected")
	}
	insertResult, err := collection.Collection.InsertOne(context.TODO(), user)
	if err != nil {
		return nil, fmt.Errorf("failed to insert user: %w", err)
	}
	return insertResult, nil
}

func FindOneUser(filter interface{}) (*User, error) {
	if collection == nil {
		return nil, fmt.Errorf("collection is not connected")
	}
	var user *User
	err := collection.Collection.FindOne(context.TODO(), filter).Decode(user)
	return user, err
}

func UpdateOneUser(filter interface{}, update interface{}) (*mongo.UpdateResult, error) {
	if collection == nil {
		return nil, fmt.Errorf("collection is not connected")
	}
	return collection.Collection.UpdateOne(context.TODO(), filter, update)
}
