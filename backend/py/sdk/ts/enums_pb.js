/**
 * @fileoverview
 * @enhanceable
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

var jspb = require('google-protobuf');
var goog = jspb;
var global = Function('return this')();

goog.exportSymbol('proto.pb.BehaviorType', null, global);
goog.exportSymbol('proto.pb.DataType', null, global);
goog.exportSymbol('proto.pb.FunctionType', null, global);
goog.exportSymbol('proto.pb.KeyType', null, global);
goog.exportSymbol('proto.pb.ServiceType', null, global);
goog.exportSymbol('proto.pb.TaskType', null, global);
/**
 * @enum {number}
 */
proto.pb.BehaviorType = {
  CONTINUOUS: 0,
  CATEGORICAL: 1,
  TEXT: 2
};

/**
 * @enum {number}
 */
proto.pb.DataType = {
  BOOLEAN: 0,
  FLOAT: 1,
  INTEGER: 2,
  STRING: 3
};

/**
 * @enum {number}
 */
proto.pb.FunctionType = {
  MEAN: 0,
  MEDIAN: 1,
  MODE: 2
};

/**
 * @enum {number}
 */
proto.pb.KeyType = {
  LEARN: 0,
  TRANSFORMER: 1,
  ESTIMATOR: 2,
  EVALUATION: 3
};

/**
 * @enum {number}
 */
proto.pb.ServiceType = {
  SERVICE_LEARN: 0,
  SERVICE_PREDICT: 1
};

/**
 * @enum {number}
 */
proto.pb.TaskType = {
  CLASSIFY: 0,
  REGRESS: 1,
  CLUSTER: 2
};

goog.object.extend(exports, proto.pb);
