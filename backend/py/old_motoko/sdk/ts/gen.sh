PROTOC_GEN_TS_PATH='./node_modules/ts-protoc-gen/bin/protoc-gen-ts'
OUT_DIR="."
mkdir -p $OUT_DIR

# types
protoc \
  --proto_path=protos \
  --plugin="protoc-gen-ts=${PROTOC_GEN_TS_PATH}" \
  --js_out="import_style=commonjs,binary:${OUT_DIR}" \
  --ts_out="${OUT_DIR}" \
  protos/enums.proto \
  protos/types.proto \
  protos/motoko.proto

# service
protoc \
  --proto_path=protos \
  --plugin="protoc-gen-ts=${PROTOC_GEN_TS_PATH}" \
  --ts_out="service=grpc-web:${OUT_DIR}" \
  protos/motoko.proto
