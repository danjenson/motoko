# protobuf enums and types
python -m grpc_tools.protoc -I protos --python_out=. protos/enums.proto protos/types.proto

# gRPC service
python -m grpc_tools.protoc \
  -I protos \
  -I $GOPATH/src/github.com/grpc-ecosystem/grpc-gateway/third_party/googleapis \
  --grpc_python_out=. protos/motoko.proto
