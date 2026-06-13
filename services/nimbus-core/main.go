package main

import (
	"log"
	"net"

	fileservice "github.com/biswasakashdev/nimbus/services/nimbus-core/file-service"
	"github.com/biswasakashdev/nimbus/services/nimbus-core/proto-gen/client"
	"google.golang.org/grpc"
)

func main() {

	// 1. Listen on a TCP port
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	// 2. Create the gRPC server

	// 3. Initialize your LoadBalancer implementation
	// This struct will hold the logic for Register and WatchBackends

	grpcServer := grpc.NewServer()
	fileGrpcServer := fileservice.NewFileGRPCService()

	// 4. Register the service with the gRPC server
	client.RegisterNimbusServiceServer(grpcServer, fileGrpcServer)

	log.Printf("Nimbus Core started on %v", lis.Addr())

	// 5. Serve requests
	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}

}
