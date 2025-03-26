package models

import (
	"context"
	"fmt"
	"user-auth/internal/db"

	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
)

type Getter interface {
	GetSchema() any
	GetPreUpdate(update *bson.M) *bson.M
	GetPreInsert(record any) any
}

type Model struct {
	Name        string
	Collection  *mongo.Collection
	IndexModels []mongo.IndexModel
	Getter      Getter
}

func NewModel(Name string, Collection *mongo.Collection, IndexModels []mongo.IndexModel) *Model {
	model := &Model{
		Name: Name, Collection: Collection, IndexModels: IndexModels,
	}
	model.Getter = model
	return model
}

func (model *Model) GetSchema() any {
	return nil
}

func (model *Model) GetPreUpdate(update *bson.M) *bson.M {
	return update
}

func (model *Model) GetPreInsert(record any) any {
	return record
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

func (model *Model) FindOne(ctx context.Context, filter any) (any, error) {
	if err := model.IsDown(); err != nil {
		return nil, err
	}
	fmt.Println(model)
	output := model.Getter.GetSchema()
	if err := model.Collection.FindOne(ctx, filter).Decode(output); err != nil {
		return nil, fmt.Errorf("failed to find %s - %w", model.Name, err)
	}
	return output, nil
}

func (model *Model) UpdateOne(ctx context.Context, filter *bson.M, update *bson.M) (*mongo.UpdateResult, error) {
	if err := model.IsDown(); err != nil {
		return nil, err
	}
	fmt.Println(update)
	update = model.GetPreUpdate(update)
	fmt.Println(update)
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
	fmt.Println(record)
	record = model.Getter.GetPreInsert(record)
	fmt.Println(record)
	insertResult, err := model.Collection.InsertOne(ctx, record)
	if err != nil {
		return nil, fmt.Errorf("failed to insert %s / %w", model.Name, err)
	}
	return insertResult, nil
}
