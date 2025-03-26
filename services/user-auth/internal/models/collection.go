package models

import (
	"context"
	"fmt"
	"user-auth/internal/db"

	"go.mongodb.org/mongo-driver/v2/mongo"
)

type Model struct {
	Name        string
	Collection  *mongo.Collection
	IndexModels []mongo.IndexModel
}

func (model *Model) IsDown() error {
	if model.Collection == nil {
		return fmt.Errorf("collection %s is not connected", model.Name)
	}
	return nil
}

func (model *Model) SetIndexes(ctx context.Context, client *db.MongoClient) error {
	model.Collection = client.Database.Collection(model.Name)
	if _, err := model.Collection.Indexes().CreateMany(ctx, model.IndexModels); err != nil {
		return fmt.Errorf("failed to create indexes - %w", err)
	}
	return nil
}

func (model *Model) FindOne(ctx context.Context, filter any, output any) error {
	if err := model.IsDown(); err != nil {
		return err
	}
	if err := model.Collection.FindOne(ctx, filter).Decode(output); err != nil {
		return fmt.Errorf("failed to find %s - %w", model.Name, err)
	}
	return nil
}

func (model *Model) UpdateOne(ctx context.Context, filter any, update any) (*mongo.UpdateResult, error) {
	if err := model.IsDown(); err != nil {
		return nil, err
	}
	if updateResult, err := model.Collection.UpdateOne(ctx, filter, update); err != nil {
		return nil, fmt.Errorf("failed to update %s / %w", model.Name, err)
	} else {
		return updateResult, nil
	}
}

func (model *Model) InsertOne(ctx context.Context, record any) (*mongo.InsertOneResult, error) {
	if err := model.IsDown(); err != nil {
		return nil, err
	}
	insertResult, err := model.Collection.InsertOne(ctx, record)
	if err != nil {
		return nil, fmt.Errorf("failed to insert %s / %w", model.Name, err)
	}
	return insertResult, nil
}
