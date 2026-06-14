package grpc

import (
	"context"

	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	pb "github.com/biswasakashdev/nimbus/services/nimbus-core/proto-gen/public"
)

type NimbusGRPCHandler struct {
	pb.UnimplementedNimbusPublicServiceServer
}

func NewNimbusServiceGRPCHandler() *NimbusGRPCHandler {
	return &NimbusGRPCHandler{}
}
func (NimbusGRPCHandler) UpdateAccessType(context.Context, *pb.UpdateAccessTypeRequest) (*pb.UpdateAccessTypeResponse, error) {
	return nil, status.Error(codes.Unimplemented, "method UpdateAccessType not implemented")
}

func (NimbusGRPCHandler) GetDirectoryContent(*pb.GetDirectoryContentRequest, grpc.ServerStreamingServer[pb.GetDirectoryContentResponse]) error {
	return status.Error(codes.Unimplemented, "method GetDirectoryContent not implemented")
}
func (NimbusGRPCHandler) FindObjectByName(context.Context, *pb.FindObjectByNameRequest) (*pb.FindObjectByNameResponse, error) {
	return nil, status.Error(codes.Unimplemented, "method FindObjectByName not implemented")
}
