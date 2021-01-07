# protobuf enums and types
python -m grpc_tools.protoc -I protos --python_out=. protos/enums.proto protos/types.proto

# gRPC service
python -m grpc_tools.protoc -I protos --grpc_python_out=. protos/tachikoma.proto
