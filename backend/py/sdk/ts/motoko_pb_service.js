// package: pb
// file: motoko.proto

var motoko_pb = require("./motoko_pb");
var types_pb = require("./types_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var Motoko = (function () {
  function Motoko() {}
  Motoko.serviceName = "pb.Motoko";
  return Motoko;
}());

Motoko.Infer = {
  methodName: "Infer",
  service: Motoko,
  requestStream: true,
  responseStream: false,
  requestType: types_pb.InferRequest,
  responseType: types_pb.InferResponse
};

Motoko.Learn = {
  methodName: "Learn",
  service: Motoko,
  requestStream: true,
  responseStream: false,
  requestType: types_pb.LearnRequest,
  responseType: types_pb.LearnResponse
};

Motoko.Predict = {
  methodName: "Predict",
  service: Motoko,
  requestStream: true,
  responseStream: true,
  requestType: types_pb.PredictRequest,
  responseType: types_pb.PredictResponse
};

Motoko.WebInfer = {
  methodName: "WebInfer",
  service: Motoko,
  requestStream: true,
  responseStream: true,
  requestType: types_pb.InferRequest,
  responseType: types_pb.InferResponse
};

Motoko.WebLearn = {
  methodName: "WebLearn",
  service: Motoko,
  requestStream: true,
  responseStream: true,
  requestType: types_pb.LearnRequest,
  responseType: types_pb.LearnResponse
};

exports.Motoko = Motoko;

function MotokoClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

MotokoClient.prototype.infer = function infer(metadata) {
  var listeners = {
    end: [],
    status: []
  };
  var client = grpc.client(Motoko.Infer, {
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport
  });
  client.onEnd(function (status, statusMessage, trailers) {
    listeners.status.forEach(function (handler) {
      handler({ code: status, details: statusMessage, metadata: trailers });
    });
    listeners.end.forEach(function (handler) {
      handler({ code: status, details: statusMessage, metadata: trailers });
    });
    listeners = null;
  });
  return {
    on: function (type, handler) {
      listeners[type].push(handler);
      return this;
    },
    write: function (requestMessage) {
      if (!client.started) {
        client.start(metadata);
      }
      client.send(requestMessage);
      return this;
    },
    end: function () {
      client.finishSend();
    },
    cancel: function () {
      listeners = null;
      client.close();
    }
  };
};

MotokoClient.prototype.learn = function learn(metadata) {
  var listeners = {
    end: [],
    status: []
  };
  var client = grpc.client(Motoko.Learn, {
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport
  });
  client.onEnd(function (status, statusMessage, trailers) {
    listeners.status.forEach(function (handler) {
      handler({ code: status, details: statusMessage, metadata: trailers });
    });
    listeners.end.forEach(function (handler) {
      handler({ code: status, details: statusMessage, metadata: trailers });
    });
    listeners = null;
  });
  return {
    on: function (type, handler) {
      listeners[type].push(handler);
      return this;
    },
    write: function (requestMessage) {
      if (!client.started) {
        client.start(metadata);
      }
      client.send(requestMessage);
      return this;
    },
    end: function () {
      client.finishSend();
    },
    cancel: function () {
      listeners = null;
      client.close();
    }
  };
};

MotokoClient.prototype.predict = function predict(metadata) {
  var listeners = {
    data: [],
    end: [],
    status: []
  };
  var client = grpc.client(Motoko.Predict, {
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport
  });
  client.onEnd(function (status, statusMessage, trailers) {
    listeners.status.forEach(function (handler) {
      handler({ code: status, details: statusMessage, metadata: trailers });
    });
    listeners.end.forEach(function (handler) {
      handler({ code: status, details: statusMessage, metadata: trailers });
    });
    listeners = null;
  });
  client.onMessage(function (message) {
    listeners.data.forEach(function (handler) {
      handler(message);
    })
  });
  client.start(metadata);
  return {
    on: function (type, handler) {
      listeners[type].push(handler);
      return this;
    },
    write: function (requestMessage) {
      client.send(requestMessage);
      return this;
    },
    end: function () {
      client.finishSend();
    },
    cancel: function () {
      listeners = null;
      client.close();
    }
  };
};

MotokoClient.prototype.webInfer = function webInfer(metadata) {
  var listeners = {
    data: [],
    end: [],
    status: []
  };
  var client = grpc.client(Motoko.WebInfer, {
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport
  });
  client.onEnd(function (status, statusMessage, trailers) {
    listeners.status.forEach(function (handler) {
      handler({ code: status, details: statusMessage, metadata: trailers });
    });
    listeners.end.forEach(function (handler) {
      handler({ code: status, details: statusMessage, metadata: trailers });
    });
    listeners = null;
  });
  client.onMessage(function (message) {
    listeners.data.forEach(function (handler) {
      handler(message);
    })
  });
  client.start(metadata);
  return {
    on: function (type, handler) {
      listeners[type].push(handler);
      return this;
    },
    write: function (requestMessage) {
      client.send(requestMessage);
      return this;
    },
    end: function () {
      client.finishSend();
    },
    cancel: function () {
      listeners = null;
      client.close();
    }
  };
};

MotokoClient.prototype.webLearn = function webLearn(metadata) {
  var listeners = {
    data: [],
    end: [],
    status: []
  };
  var client = grpc.client(Motoko.WebLearn, {
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport
  });
  client.onEnd(function (status, statusMessage, trailers) {
    listeners.status.forEach(function (handler) {
      handler({ code: status, details: statusMessage, metadata: trailers });
    });
    listeners.end.forEach(function (handler) {
      handler({ code: status, details: statusMessage, metadata: trailers });
    });
    listeners = null;
  });
  client.onMessage(function (message) {
    listeners.data.forEach(function (handler) {
      handler(message);
    })
  });
  client.start(metadata);
  return {
    on: function (type, handler) {
      listeners[type].push(handler);
      return this;
    },
    write: function (requestMessage) {
      client.send(requestMessage);
      return this;
    },
    end: function () {
      client.finishSend();
    },
    cancel: function () {
      listeners = null;
      client.close();
    }
  };
};

exports.MotokoClient = MotokoClient;

