package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"time"
	"user-auth/internal/db"
	"user-auth/internal/server"
)

func main() {

	username := os.Getenv("MONGO_USERNAME")
	password := os.Getenv("MONGO_PASSWORD")
	host := os.Getenv("MONGO_HOST")
	port := os.Getenv("MONGO_PORT")
	database := os.Getenv("MONGO_DB")
	uri := fmt.Sprintf("mongodb://%s:%s@%s:%s/%s?authSource=admin", username, password, host, port, database)

	client, err := db.Connect(uri, database)
	if err != nil {
		log.Fatalf("error connecting to MongoDB: %v", err)
	}
	defer client.Disconnect()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	server.StartGRPCServer(ctx, client)
}
