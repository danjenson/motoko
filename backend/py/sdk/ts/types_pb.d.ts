// package: pb
// file: types.proto

import * as jspb from "google-protobuf";
import * as enums_pb from "./enums_pb";

export class Attribute extends jspb.Message {
  getName(): string;
  setName(value: string): void;

  getBehaviorType(): enums_pb.BehaviorTypeMap[keyof enums_pb.BehaviorTypeMap];
  setBehaviorType(value: enums_pb.BehaviorTypeMap[keyof enums_pb.BehaviorTypeMap]): void;

  getDataType(): enums_pb.DataTypeMap[keyof enums_pb.DataTypeMap];
  setDataType(value: enums_pb.DataTypeMap[keyof enums_pb.DataTypeMap]): void;

  clearReplacementsList(): void;
  getReplacementsList(): Array<Replacement>;
  setReplacementsList(value: Array<Replacement>): void;
  addReplacements(value?: Replacement, index?: number): Replacement;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Attribute.AsObject;
  static toObject(includeInstance: boolean, msg: Attribute): Attribute.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Attribute, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Attribute;
  static deserializeBinaryFromReader(message: Attribute, reader: jspb.BinaryReader): Attribute;
}

export namespace Attribute {
  export type AsObject = {
    name: string,
    behaviorType: enums_pb.BehaviorTypeMap[keyof enums_pb.BehaviorTypeMap],
    dataType: enums_pb.DataTypeMap[keyof enums_pb.DataTypeMap],
    replacementsList: Array<Replacement.AsObject>,
  }
}

export class InferRequest extends jspb.Message {
  hasParameters(): boolean;
  clearParameters(): void;
  getParameters(): InferRequest.Parameters | undefined;
  setParameters(value?: InferRequest.Parameters): void;

  hasData(): boolean;
  clearData(): void;
  getData(): Uint8Array | string;
  getData_asU8(): Uint8Array;
  getData_asB64(): string;
  setData(value: Uint8Array | string): void;

  getValueCase(): InferRequest.ValueCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InferRequest.AsObject;
  static toObject(includeInstance: boolean, msg: InferRequest): InferRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: InferRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InferRequest;
  static deserializeBinaryFromReader(message: InferRequest, reader: jspb.BinaryReader): InferRequest;
}

export namespace InferRequest {
  export type AsObject = {
    parameters?: InferRequest.Parameters.AsObject,
    data: Uint8Array | string,
  }

  export class Parameters extends jspb.Message {
    getNumericErrorThreshold(): number;
    setNumericErrorThreshold(value: number): void;

    getNMaxCategories(): number;
    setNMaxCategories(value: number): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Parameters.AsObject;
    static toObject(includeInstance: boolean, msg: Parameters): Parameters.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Parameters, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Parameters;
    static deserializeBinaryFromReader(message: Parameters, reader: jspb.BinaryReader): Parameters;
  }

  export namespace Parameters {
    export type AsObject = {
      numericErrorThreshold: number,
      nMaxCategories: number,
    }
  }

  export enum ValueCase {
    VALUE_NOT_SET = 0,
    PARAMETERS = 1,
    DATA = 2,
  }
}

export class InferResponse extends jspb.Message {
  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): Metadata | undefined;
  setMetadata(value?: Metadata): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InferResponse.AsObject;
  static toObject(includeInstance: boolean, msg: InferResponse): InferResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: InferResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InferResponse;
  static deserializeBinaryFromReader(message: InferResponse, reader: jspb.BinaryReader): InferResponse;
}

export namespace InferResponse {
  export type AsObject = {
    metadata?: Metadata.AsObject,
  }
}

export class LearnRequest extends jspb.Message {
  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): Metadata | undefined;
  setMetadata(value?: Metadata): void;

  hasData(): boolean;
  clearData(): void;
  getData(): Uint8Array | string;
  getData_asU8(): Uint8Array;
  getData_asB64(): string;
  setData(value: Uint8Array | string): void;

  getValueCase(): LearnRequest.ValueCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LearnRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LearnRequest): LearnRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LearnRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LearnRequest;
  static deserializeBinaryFromReader(message: LearnRequest, reader: jspb.BinaryReader): LearnRequest;
}

export namespace LearnRequest {
  export type AsObject = {
    metadata?: Metadata.AsObject,
    data: Uint8Array | string,
  }

  export enum ValueCase {
    VALUE_NOT_SET = 0,
    METADATA = 1,
    DATA = 2,
  }
}

export class LearnResponse extends jspb.Message {
  getLearnKey(): string;
  setLearnKey(value: string): void;

  getEvaluation(): string;
  setEvaluation(value: string): void;

  getDecisions(): string;
  setDecisions(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LearnResponse.AsObject;
  static toObject(includeInstance: boolean, msg: LearnResponse): LearnResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LearnResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LearnResponse;
  static deserializeBinaryFromReader(message: LearnResponse, reader: jspb.BinaryReader): LearnResponse;
}

export namespace LearnResponse {
  export type AsObject = {
    learnKey: string,
    evaluation: string,
    decisions: string,
  }
}

export class Metadata extends jspb.Message {
  getHasTarget(): boolean;
  setHasTarget(value: boolean): void;

  getTargetName(): string;
  setTargetName(value: string): void;

  clearAttributesList(): void;
  getAttributesList(): Array<Attribute>;
  setAttributesList(value: Array<Attribute>): void;
  addAttributes(value?: Attribute, index?: number): Attribute;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Metadata.AsObject;
  static toObject(includeInstance: boolean, msg: Metadata): Metadata.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Metadata, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Metadata;
  static deserializeBinaryFromReader(message: Metadata, reader: jspb.BinaryReader): Metadata;
}

export namespace Metadata {
  export type AsObject = {
    hasTarget: boolean,
    targetName: string,
    attributesList: Array<Attribute.AsObject>,
  }
}

export class PredictRequest extends jspb.Message {
  hasLearnKey(): boolean;
  clearLearnKey(): void;
  getLearnKey(): string;
  setLearnKey(value: string): void;

  hasData(): boolean;
  clearData(): void;
  getData(): Uint8Array | string;
  getData_asU8(): Uint8Array;
  getData_asB64(): string;
  setData(value: Uint8Array | string): void;

  getValueCase(): PredictRequest.ValueCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PredictRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PredictRequest): PredictRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: PredictRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PredictRequest;
  static deserializeBinaryFromReader(message: PredictRequest, reader: jspb.BinaryReader): PredictRequest;
}

export namespace PredictRequest {
  export type AsObject = {
    learnKey: string,
    data: Uint8Array | string,
  }

  export enum ValueCase {
    VALUE_NOT_SET = 0,
    LEARN_KEY = 1,
    DATA = 2,
  }
}

export class PredictResponse extends jspb.Message {
  hasPredictions(): boolean;
  clearPredictions(): void;
  getPredictions(): string;
  setPredictions(value: string): void;

  hasDecisions(): boolean;
  clearDecisions(): void;
  getDecisions(): string;
  setDecisions(value: string): void;

  getValueCase(): PredictResponse.ValueCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PredictResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PredictResponse): PredictResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: PredictResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PredictResponse;
  static deserializeBinaryFromReader(message: PredictResponse, reader: jspb.BinaryReader): PredictResponse;
}

export namespace PredictResponse {
  export type AsObject = {
    predictions: string,
    decisions: string,
  }

  export enum ValueCase {
    VALUE_NOT_SET = 0,
    PREDICTIONS = 1,
    DECISIONS = 2,
  }
}

export class Replacement extends jspb.Message {
  hasFrom(): boolean;
  clearFrom(): void;
  getFrom(): Replacement.From | undefined;
  setFrom(value?: Replacement.From): void;

  hasTo(): boolean;
  clearTo(): void;
  getTo(): Replacement.To | undefined;
  setTo(value?: Replacement.To): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Replacement.AsObject;
  static toObject(includeInstance: boolean, msg: Replacement): Replacement.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Replacement, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Replacement;
  static deserializeBinaryFromReader(message: Replacement, reader: jspb.BinaryReader): Replacement;
}

export namespace Replacement {
  export type AsObject = {
    from?: Replacement.From.AsObject,
    to?: Replacement.To.AsObject,
  }

  export class From extends jspb.Message {
    hasMissing(): boolean;
    clearMissing(): void;
    getMissing(): boolean;
    setMissing(value: boolean): void;

    hasDatum(): boolean;
    clearDatum(): void;
    getDatum(): TypedDatum | undefined;
    setDatum(value?: TypedDatum): void;

    getValueCase(): From.ValueCase;
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): From.AsObject;
    static toObject(includeInstance: boolean, msg: From): From.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: From, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): From;
    static deserializeBinaryFromReader(message: From, reader: jspb.BinaryReader): From;
  }

  export namespace From {
    export type AsObject = {
      missing: boolean,
      datum?: TypedDatum.AsObject,
    }

    export enum ValueCase {
      VALUE_NOT_SET = 0,
      MISSING = 1,
      DATUM = 2,
    }
  }

  export class To extends jspb.Message {
    hasFunction(): boolean;
    clearFunction(): void;
    getFunction(): enums_pb.FunctionTypeMap[keyof enums_pb.FunctionTypeMap];
    setFunction(value: enums_pb.FunctionTypeMap[keyof enums_pb.FunctionTypeMap]): void;

    hasDatum(): boolean;
    clearDatum(): void;
    getDatum(): TypedDatum | undefined;
    setDatum(value?: TypedDatum): void;

    getValueCase(): To.ValueCase;
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): To.AsObject;
    static toObject(includeInstance: boolean, msg: To): To.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: To, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): To;
    static deserializeBinaryFromReader(message: To, reader: jspb.BinaryReader): To;
  }

  export namespace To {
    export type AsObject = {
      pb_function: enums_pb.FunctionTypeMap[keyof enums_pb.FunctionTypeMap],
      datum?: TypedDatum.AsObject,
    }

    export enum ValueCase {
      VALUE_NOT_SET = 0,
      FUNCTION = 3,
      DATUM = 4,
    }
  }
}

export class TypedDatum extends jspb.Message {
  getType(): enums_pb.DataTypeMap[keyof enums_pb.DataTypeMap];
  setType(value: enums_pb.DataTypeMap[keyof enums_pb.DataTypeMap]): void;

  hasBoolean(): boolean;
  clearBoolean(): void;
  getBoolean(): boolean;
  setBoolean(value: boolean): void;

  hasFloat(): boolean;
  clearFloat(): void;
  getFloat(): number;
  setFloat(value: number): void;

  hasInteger(): boolean;
  clearInteger(): void;
  getInteger(): number;
  setInteger(value: number): void;

  hasString(): boolean;
  clearString(): void;
  getString(): string;
  setString(value: string): void;

  getValueCase(): TypedDatum.ValueCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TypedDatum.AsObject;
  static toObject(includeInstance: boolean, msg: TypedDatum): TypedDatum.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: TypedDatum, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TypedDatum;
  static deserializeBinaryFromReader(message: TypedDatum, reader: jspb.BinaryReader): TypedDatum;
}

export namespace TypedDatum {
  export type AsObject = {
    type: enums_pb.DataTypeMap[keyof enums_pb.DataTypeMap],
    pb_boolean: boolean,
    pb_float: number,
    integer: number,
    string: string,
  }

  export enum ValueCase {
    VALUE_NOT_SET = 0,
    BOOLEAN = 2,
    FLOAT = 3,
    INTEGER = 4,
    STRING = 5,
  }
}

