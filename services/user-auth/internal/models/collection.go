package models

import (
	"context"
	"fmt"
	"user-auth/internal/db"

	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
	"go.mongodb.org/mongo-driver/v2/mongo/options"
)

type Getter interface {
	GetSchema() any
	GetPreUpdate(*bson.M) *bson.M
	GetPreInsert(any) any
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

func (model *Model) FindOne(ctx context.Context, filter any) (*mongo.SingleResult, error) {
	if err := model.IsDown(); err != nil {
		return nil, err
	}
	// ERROR IS IN THE RESULT!!!
	result := model.Collection.FindOne(ctx, filter)
	return result, result.Err()
}

func (model *Model) FindOneD(ctx context.Context, filter any) (any, error) {
	result, err := model.FindOne(ctx, filter)
	if err != nil {
		return nil, err
	}
	output := model.Getter.GetSchema()
	err = result.Decode(output)
	return output, err
}

func (model *Model) UpdateOne(ctx context.Context, filter *bson.M, update *bson.M, opts ...options.Lister[options.UpdateOneOptions]) (*mongo.UpdateResult, error) {
	if err := model.IsDown(); err != nil {
		return nil, err
	}
	update = model.GetPreUpdate(update)
	if updateResult, err := model.Collection.UpdateOne(ctx, filter, update, opts...); err != nil {
		return nil, fmt.Errorf("failed to update %s / %w", model.Name, err)
	} else {
		return updateResult, nil
	}
}

func (model *Model) InsertOne(ctx context.Context, record any, opts ...options.Lister[options.InsertOneOptions]) (*mongo.InsertOneResult, error) {
	if err := model.IsDown(); err != nil {
		return nil, err
	}
	record = model.Getter.GetPreInsert(record)
	insertResult, err := model.Collection.InsertOne(ctx, record, opts...)
	if err != nil {
		return nil, fmt.Errorf("failed to insert %s / %w", model.Name, err)
	}
	return insertResult, nil
}
