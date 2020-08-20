// package: pb
// file: motoko.proto

import * as motoko_pb from "./motoko_pb";
import * as types_pb from "./types_pb";
import {grpc} from "@improbable-eng/grpc-web";

type MotokoInfer = {
  readonly methodName: string;
  readonly service: typeof Motoko;
  readonly requestStream: true;
  readonly responseStream: false;
  readonly requestType: typeof types_pb.InferRequest;
  readonly responseType: typeof types_pb.InferResponse;
};

type MotokoLearn = {
  readonly methodName: string;
  readonly service: typeof Motoko;
  readonly requestStream: true;
  readonly responseStream: false;
  readonly requestType: typeof types_pb.LearnRequest;
  readonly responseType: typeof types_pb.LearnResponse;
};

type MotokoPredict = {
  readonly methodName: string;
  readonly service: typeof Motoko;
  readonly requestStream: true;
  readonly responseStream: true;
  readonly requestType: typeof types_pb.PredictRequest;
  readonly responseType: typeof types_pb.PredictResponse;
};

type MotokoWebInfer = {
  readonly methodName: string;
  readonly service: typeof Motoko;
  readonly requestStream: true;
  readonly responseStream: true;
  readonly requestType: typeof types_pb.InferRequest;
  readonly responseType: typeof types_pb.InferResponse;
};

type MotokoWebLearn = {
  readonly methodName: string;
  readonly service: typeof Motoko;
  readonly requestStream: true;
  readonly responseStream: true;
  readonly requestType: typeof types_pb.LearnRequest;
  readonly responseType: typeof types_pb.LearnResponse;
};

export class Motoko {
  static readonly serviceName: string;
  static readonly Infer: MotokoInfer;
  static readonly Learn: MotokoLearn;
  static readonly Predict: MotokoPredict;
  static readonly WebInfer: MotokoWebInfer;
  static readonly WebLearn: MotokoWebLearn;
}

export type ServiceError = { message: string, code: number; metadata: grpc.Metadata }
export type Status = { details: string, code: number; metadata: grpc.Metadata }

interface UnaryResponse {
  cancel(): void;
}
interface ResponseStream<T> {
  cancel(): void;
  on(type: 'data', handler: (message: T) => void): ResponseStream<T>;
  on(type: 'end', handler: (status?: Status) => void): ResponseStream<T>;
  on(type: 'status', handler: (status: Status) => void): ResponseStream<T>;
}
interface RequestStream<T> {
  write(message: T): RequestStream<T>;
  end(): void;
  cancel(): void;
  on(type: 'end', handler: (status?: Status) => void): RequestStream<T>;
  on(type: 'status', handler: (status: Status) => void): RequestStream<T>;
}
interface BidirectionalStream<ReqT, ResT> {
  write(message: ReqT): BidirectionalStream<ReqT, ResT>;
  end(): void;
  cancel(): void;
  on(type: 'data', handler: (message: ResT) => void): BidirectionalStream<ReqT, ResT>;
  on(type: 'end', handler: (status?: Status) => void): BidirectionalStream<ReqT, ResT>;
  on(type: 'status', handler: (status: Status) => void): BidirectionalStream<ReqT, ResT>;
}

export class MotokoClient {
  readonly serviceHost: string;

  constructor(serviceHost: string, options?: grpc.RpcOptions);
  infer(metadata?: grpc.Metadata): RequestStream<types_pb.InferRequest>;
  learn(metadata?: grpc.Metadata): RequestStream<types_pb.LearnRequest>;
  predict(metadata?: grpc.Metadata): BidirectionalStream<types_pb.PredictRequest, types_pb.PredictResponse>;
  webInfer(metadata?: grpc.Metadata): BidirectionalStream<types_pb.InferRequest, types_pb.InferResponse>;
  webLearn(metadata?: grpc.Metadata): BidirectionalStream<types_pb.LearnRequest, types_pb.LearnResponse>;
}

