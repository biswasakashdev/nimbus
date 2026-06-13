package fileservice

import (
	pb "github.com/biswasakashdev/nimbus/services/nimbus-core/proto-gen/client"
)

type FileGRPCService struct {
	pb.UnimplementedNimbusServiceServer
}

func NewFileGRPCService() *FileGRPCService {
	return &FileGRPCService{}
}
