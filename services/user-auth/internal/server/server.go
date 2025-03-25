package server

import (
	"fmt"
	"log"
	"net"

	"user-auth/internal/auth"
	"user-auth/internal/interceptors"
	pb "user-auth/proto"

	"google.golang.org/grpc"
)

func StartGRPCServer() {
	listener, err := net.Listen("tcp", ":5000")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer(
		interceptors.ChainUnaryInterceptors(),
	)
	pb.RegisterUserAuthServiceServer(grpcServer, &auth.Server{})

	fmt.Println("gRPC server started on port :5000")
	if err := grpcServer.Serve(listener); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
