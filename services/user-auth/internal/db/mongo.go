package db

import (
	"context"
	"fmt"

	"go.mongodb.org/mongo-driver/v2/mongo"
	"go.mongodb.org/mongo-driver/v2/mongo/options"
)

type MongoClient struct {
	Client     *mongo.Client
	Database   *mongo.Database
	Collection *mongo.Collection
}

type MongoCollection struct {
	*mongo.Collection
}

func Connect(uri, database string) (*MongoClient, error) {
	clientOptions := options.Client().ApplyURI(uri)
	client, err := mongo.Connect(clientOptions)
	if err != nil {
		return nil, fmt.Errorf("could not connect to MongoDB: %v", err)
	}

	err = client.Ping(context.Background(), nil)
	if err != nil {
		return nil, fmt.Errorf("could not ping MongoDB: %v", err)
	}

	return &MongoClient{
		Client:   client,
		Database: client.Database(database),
	}, nil
}

func (client *MongoClient) Disconnect() error {
	return client.Client.Disconnect(context.Background())
}

func (client *MongoClient) Use(collectionName string) *MongoCollection {
	return &MongoCollection{
		Collection: client.Database.Collection(collectionName),
	}
}

func (collection *MongoCollection) InsertOne(document interface{}) (*mongo.InsertOneResult, error) {
	return collection.Collection.InsertOne(context.Background(), document)
}

func (collection *MongoCollection) FindOne(filter interface{}) *mongo.SingleResult {
	return collection.Collection.FindOne(context.Background(), filter)
}
