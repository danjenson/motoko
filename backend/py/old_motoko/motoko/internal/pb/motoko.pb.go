// Code generated by protoc-gen-go. DO NOT EDIT.
// source: motoko.proto

package pb

import (
	context "context"
	fmt "fmt"
	proto "github.com/golang/protobuf/proto"
	grpc "google.golang.org/grpc"
	math "math"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion3 // please upgrade the proto package

func init() { proto.RegisterFile("motoko.proto", fileDescriptor_7385652019f7c0bb) }

var fileDescriptor_7385652019f7c0bb = []byte{
	// 158 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xe2, 0xe2, 0xc9, 0xcd, 0x2f, 0xc9,
	0xcf, 0xce, 0xd7, 0x2b, 0x28, 0xca, 0x2f, 0xc9, 0x17, 0x62, 0x2a, 0x48, 0x92, 0xe2, 0x2e, 0xa9,
	0x2c, 0x48, 0x2d, 0x86, 0x08, 0x18, 0x75, 0x31, 0x71, 0xb1, 0xf9, 0x82, 0x55, 0x08, 0xe9, 0x71,
	0xb1, 0x7a, 0xe6, 0xa5, 0xa5, 0x16, 0x09, 0x09, 0xe8, 0x15, 0x24, 0xe9, 0x81, 0x99, 0x41, 0xa9,
	0x85, 0xa5, 0xa9, 0xc5, 0x25, 0x52, 0x82, 0x48, 0x22, 0xc5, 0x05, 0xf9, 0x79, 0xc5, 0xa9, 0x1a,
	0x8c, 0x20, 0xf5, 0x3e, 0xa9, 0x89, 0x45, 0x79, 0x10, 0xf5, 0x60, 0x26, 0x8a, 0x7a, 0xa8, 0x08,
	0x5c, 0xbd, 0x19, 0x17, 0x7b, 0x40, 0x51, 0x6a, 0x4a, 0x66, 0x72, 0x89, 0x90, 0x10, 0x48, 0x1e,
	0xca, 0x81, 0xe9, 0x11, 0x46, 0x11, 0x83, 0xe9, 0x32, 0x60, 0x14, 0x32, 0xe6, 0xe2, 0x08, 0x4f,
	0x4d, 0x22, 0xc5, 0x69, 0x70, 0x4d, 0xa4, 0xb8, 0xcf, 0x80, 0x31, 0x89, 0x0d, 0x1c, 0x26, 0xc6,
	0x80, 0x00, 0x00, 0x00, 0xff, 0xff, 0x14, 0x33, 0x6a, 0x2d, 0x34, 0x01, 0x00, 0x00,
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// MotokoClient is the client API for Motoko service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://godoc.org/google.golang.org/grpc#ClientConn.NewStream.
type MotokoClient interface {
	Infer(ctx context.Context, opts ...grpc.CallOption) (Motoko_InferClient, error)
	Learn(ctx context.Context, opts ...grpc.CallOption) (Motoko_LearnClient, error)
	Predict(ctx context.Context, opts ...grpc.CallOption) (Motoko_PredictClient, error)
	// TODO(danj): remove this once client-stream -> unary response is fixed
	// https://github.com/improbable-eng/grpc-web/issues/551
	WebInfer(ctx context.Context, opts ...grpc.CallOption) (Motoko_WebInferClient, error)
	WebLearn(ctx context.Context, opts ...grpc.CallOption) (Motoko_WebLearnClient, error)
}

type motokoClient struct {
	cc *grpc.ClientConn
}

func NewMotokoClient(cc *grpc.ClientConn) MotokoClient {
	return &motokoClient{cc}
}

func (c *motokoClient) Infer(ctx context.Context, opts ...grpc.CallOption) (Motoko_InferClient, error) {
	stream, err := c.cc.NewStream(ctx, &_Motoko_serviceDesc.Streams[0], "/pb.Motoko/Infer", opts...)
	if err != nil {
		return nil, err
	}
	x := &motokoInferClient{stream}
	return x, nil
}

type Motoko_InferClient interface {
	Send(*InferRequest) error
	CloseAndRecv() (*InferResponse, error)
	grpc.ClientStream
}

type motokoInferClient struct {
	grpc.ClientStream
}

func (x *motokoInferClient) Send(m *InferRequest) error {
	return x.ClientStream.SendMsg(m)
}

func (x *motokoInferClient) CloseAndRecv() (*InferResponse, error) {
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	m := new(InferResponse)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func (c *motokoClient) Learn(ctx context.Context, opts ...grpc.CallOption) (Motoko_LearnClient, error) {
	stream, err := c.cc.NewStream(ctx, &_Motoko_serviceDesc.Streams[1], "/pb.Motoko/Learn", opts...)
	if err != nil {
		return nil, err
	}
	x := &motokoLearnClient{stream}
	return x, nil
}

type Motoko_LearnClient interface {
	Send(*LearnRequest) error
	CloseAndRecv() (*LearnResponse, error)
	grpc.ClientStream
}

type motokoLearnClient struct {
	grpc.ClientStream
}

func (x *motokoLearnClient) Send(m *LearnRequest) error {
	return x.ClientStream.SendMsg(m)
}

func (x *motokoLearnClient) CloseAndRecv() (*LearnResponse, error) {
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	m := new(LearnResponse)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func (c *motokoClient) Predict(ctx context.Context, opts ...grpc.CallOption) (Motoko_PredictClient, error) {
	stream, err := c.cc.NewStream(ctx, &_Motoko_serviceDesc.Streams[2], "/pb.Motoko/Predict", opts...)
	if err != nil {
		return nil, err
	}
	x := &motokoPredictClient{stream}
	return x, nil
}

type Motoko_PredictClient interface {
	Send(*PredictRequest) error
	Recv() (*PredictResponse, error)
	grpc.ClientStream
}

type motokoPredictClient struct {
	grpc.ClientStream
}

func (x *motokoPredictClient) Send(m *PredictRequest) error {
	return x.ClientStream.SendMsg(m)
}

func (x *motokoPredictClient) Recv() (*PredictResponse, error) {
	m := new(PredictResponse)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func (c *motokoClient) WebInfer(ctx context.Context, opts ...grpc.CallOption) (Motoko_WebInferClient, error) {
	stream, err := c.cc.NewStream(ctx, &_Motoko_serviceDesc.Streams[3], "/pb.Motoko/WebInfer", opts...)
	if err != nil {
		return nil, err
	}
	x := &motokoWebInferClient{stream}
	return x, nil
}

type Motoko_WebInferClient interface {
	Send(*InferRequest) error
	Recv() (*InferResponse, error)
	grpc.ClientStream
}

type motokoWebInferClient struct {
	grpc.ClientStream
}

func (x *motokoWebInferClient) Send(m *InferRequest) error {
	return x.ClientStream.SendMsg(m)
}

func (x *motokoWebInferClient) Recv() (*InferResponse, error) {
	m := new(InferResponse)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func (c *motokoClient) WebLearn(ctx context.Context, opts ...grpc.CallOption) (Motoko_WebLearnClient, error) {
	stream, err := c.cc.NewStream(ctx, &_Motoko_serviceDesc.Streams[4], "/pb.Motoko/WebLearn", opts...)
	if err != nil {
		return nil, err
	}
	x := &motokoWebLearnClient{stream}
	return x, nil
}

type Motoko_WebLearnClient interface {
	Send(*LearnRequest) error
	Recv() (*LearnResponse, error)
	grpc.ClientStream
}

type motokoWebLearnClient struct {
	grpc.ClientStream
}

func (x *motokoWebLearnClient) Send(m *LearnRequest) error {
	return x.ClientStream.SendMsg(m)
}

func (x *motokoWebLearnClient) Recv() (*LearnResponse, error) {
	m := new(LearnResponse)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// MotokoServer is the server API for Motoko service.
type MotokoServer interface {
	Infer(Motoko_InferServer) error
	Learn(Motoko_LearnServer) error
	Predict(Motoko_PredictServer) error
	// TODO(danj): remove this once client-stream -> unary response is fixed
	// https://github.com/improbable-eng/grpc-web/issues/551
	WebInfer(Motoko_WebInferServer) error
	WebLearn(Motoko_WebLearnServer) error
}

func RegisterMotokoServer(s *grpc.Server, srv MotokoServer) {
	s.RegisterService(&_Motoko_serviceDesc, srv)
}

func _Motoko_Infer_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(MotokoServer).Infer(&motokoInferServer{stream})
}

type Motoko_InferServer interface {
	SendAndClose(*InferResponse) error
	Recv() (*InferRequest, error)
	grpc.ServerStream
}

type motokoInferServer struct {
	grpc.ServerStream
}

func (x *motokoInferServer) SendAndClose(m *InferResponse) error {
	return x.ServerStream.SendMsg(m)
}

func (x *motokoInferServer) Recv() (*InferRequest, error) {
	m := new(InferRequest)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func _Motoko_Learn_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(MotokoServer).Learn(&motokoLearnServer{stream})
}

type Motoko_LearnServer interface {
	SendAndClose(*LearnResponse) error
	Recv() (*LearnRequest, error)
	grpc.ServerStream
}

type motokoLearnServer struct {
	grpc.ServerStream
}

func (x *motokoLearnServer) SendAndClose(m *LearnResponse) error {
	return x.ServerStream.SendMsg(m)
}

func (x *motokoLearnServer) Recv() (*LearnRequest, error) {
	m := new(LearnRequest)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func _Motoko_Predict_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(MotokoServer).Predict(&motokoPredictServer{stream})
}

type Motoko_PredictServer interface {
	Send(*PredictResponse) error
	Recv() (*PredictRequest, error)
	grpc.ServerStream
}

type motokoPredictServer struct {
	grpc.ServerStream
}

func (x *motokoPredictServer) Send(m *PredictResponse) error {
	return x.ServerStream.SendMsg(m)
}

func (x *motokoPredictServer) Recv() (*PredictRequest, error) {
	m := new(PredictRequest)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func _Motoko_WebInfer_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(MotokoServer).WebInfer(&motokoWebInferServer{stream})
}

type Motoko_WebInferServer interface {
	Send(*InferResponse) error
	Recv() (*InferRequest, error)
	grpc.ServerStream
}

type motokoWebInferServer struct {
	grpc.ServerStream
}

func (x *motokoWebInferServer) Send(m *InferResponse) error {
	return x.ServerStream.SendMsg(m)
}

func (x *motokoWebInferServer) Recv() (*InferRequest, error) {
	m := new(InferRequest)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func _Motoko_WebLearn_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(MotokoServer).WebLearn(&motokoWebLearnServer{stream})
}

type Motoko_WebLearnServer interface {
	Send(*LearnResponse) error
	Recv() (*LearnRequest, error)
	grpc.ServerStream
}

type motokoWebLearnServer struct {
	grpc.ServerStream
}

func (x *motokoWebLearnServer) Send(m *LearnResponse) error {
	return x.ServerStream.SendMsg(m)
}

func (x *motokoWebLearnServer) Recv() (*LearnRequest, error) {
	m := new(LearnRequest)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

var _Motoko_serviceDesc = grpc.ServiceDesc{
	ServiceName: "pb.Motoko",
	HandlerType: (*MotokoServer)(nil),
	Methods:     []grpc.MethodDesc{},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "Infer",
			Handler:       _Motoko_Infer_Handler,
			ClientStreams: true,
		},
		{
			StreamName:    "Learn",
			Handler:       _Motoko_Learn_Handler,
			ClientStreams: true,
		},
		{
			StreamName:    "Predict",
			Handler:       _Motoko_Predict_Handler,
			ServerStreams: true,
			ClientStreams: true,
		},
		{
			StreamName:    "WebInfer",
			Handler:       _Motoko_WebInfer_Handler,
			ServerStreams: true,
			ClientStreams: true,
		},
		{
			StreamName:    "WebLearn",
			Handler:       _Motoko_WebLearn_Handler,
			ServerStreams: true,
			ClientStreams: true,
		},
	},
	Metadata: "motoko.proto",
}