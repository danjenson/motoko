# protobuf enums and types
protoc -I protos protos/enums.proto protos/types.proto --go_out=.

# gRPC services
protoc -I protos protos/motoko.proto protos/tachikoma.proto --go_out=plugins=grpc:.
