#!/bin/bash
# @grpc/grpc-js @grpc/proto-loader google-protobuf grpc-web
PROTO_PATH=./protos/database/v1/database.proto

CLIENT_DIR=./src/gen

# Generate JS and TS code
#protoc -I=./proto ./proto/*.proto \
#  --js_out=import_style=browser,binary:${PROTO_DIR} \
#  --grpc-web_out=import_style=typescript,mode=grpcwebtext:${PROTO_DIR}

#rm -r ./src/services/grpc/generated/*

#protoc -I=./proto \
#  --js_out=import_style=commonjs,binary:${PROTO_DIR} \
#  --grpc-web_out=import_style=typescript,mode=grpcwebtext:${PROTO_DIR} \
#  ./proto/*.proto

npx protoc --ts_out ${CLIENT_DIR} --proto_path protos ${PROTO_PATH}