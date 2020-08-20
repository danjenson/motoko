// package: pb
// file: enums.proto

import * as jspb from "google-protobuf";

export interface BehaviorTypeMap {
  CONTINUOUS: 0;
  CATEGORICAL: 1;
  TEXT: 2;
}

export const BehaviorType: BehaviorTypeMap;

export interface DataTypeMap {
  BOOLEAN: 0;
  FLOAT: 1;
  INTEGER: 2;
  STRING: 3;
}

export const DataType: DataTypeMap;

export interface FunctionTypeMap {
  MEAN: 0;
  MEDIAN: 1;
  MODE: 2;
}

export const FunctionType: FunctionTypeMap;

export interface KeyTypeMap {
  LEARN: 0;
  TRANSFORMER: 1;
  ESTIMATOR: 2;
  EVALUATION: 3;
}

export const KeyType: KeyTypeMap;

export interface ServiceTypeMap {
  SERVICE_LEARN: 0;
  SERVICE_PREDICT: 1;
}

export const ServiceType: ServiceTypeMap;

export interface TaskTypeMap {
  CLASSIFY: 0;
  REGRESS: 1;
  CLUSTER: 2;
}

export const TaskType: TaskTypeMap;

