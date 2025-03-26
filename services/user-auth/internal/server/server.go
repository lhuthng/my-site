package server

import (
	"context"
	"fmt"
	"log"
	"net"
	"os"

	"user-auth/internal/auth"
	"user-auth/internal/db"
	"user-auth/internal/interceptors"
	pb "user-auth/proto/user"

	"google.golang.org/grpc"
)

func StartGRPCServer(ctx context.Context, client *db.MongoClient) {
	listener, err := net.Listen("tcp", ":5000")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer(
		interceptors.ChainUnaryInterceptors(),
	)

	server := auth.NewServer(
		ctx,
		os.Getenv("JWT_SECRET"),
		client,
	)
	pb.RegisterUserAuthServiceServer(grpcServer, server)

	fmt.Println("gRPC server started on port :5000")
	if err := grpcServer.Serve(listener); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
