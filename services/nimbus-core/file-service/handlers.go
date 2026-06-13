package fileservice

import (
	"context"

	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	pb "github.com/biswasakashdev/nimbus/services/nimbus-core/proto-gen/client"
)

func (FileGRPCService) UpdateAccessType(context.Context, *pb.UpdateAccessTypeRequest) (*pb.UpdateAccessTypeResponse, error) {
	return nil, status.Error(codes.Unimplemented, "method UpdateAccessType not implemented")
}
func (FileGRPCService) DeleteObject(context.Context, *pb.DeleteObjectRequest) (*pb.DeleteObjectResponse, error) {
	return nil, status.Error(codes.Unimplemented, "method DeleteObject not implemented")
}
func (FileGRPCService) GetDirectoryContent(*pb.GetDirectoryContentRequest, grpc.ServerStreamingServer[pb.GetDirectoryContentResponse]) error {
	return status.Error(codes.Unimplemented, "method GetDirectoryContent not implemented")
}
func (FileGRPCService) FindObjectByName(context.Context, *pb.FindObjectByNameRequest) (*pb.FindObjectByNameResponse, error) {
	return nil, status.Error(codes.Unimplemented, "method FindObjectByName not implemented")
}
