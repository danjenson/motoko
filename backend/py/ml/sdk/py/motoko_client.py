"""Motoko client."""
import json
import os

import grpc

from motoko_pb2_grpc import MotokoStub
import types_pb2 as t
import utils as u


class MotokoClient:
    """Motoko client."""
    def __init__(self, email, api_key):
        auth = AuthMetadataPlugin(email, api_key)
        call_creds = grpc.metadata_call_credentials(auth)
        # TODO(danj): use TLS CA authority and motoko.ai
        with open(os.path.expanduser('~/.keys/server.crt'), 'rb') as f:
            tls_creds = grpc.ssl_channel_credentials(f.read())
        creds = grpc.composite_channel_credentials(tls_creds, call_creds)
        self.channel = grpc.secure_channel('localhost:9000', creds)
        self.motoko = MotokoStub(self.channel)

    def __del__(self):
        # TODO(danj): fix once semantics are agreed upon:
        # https://github.com/grpc/grpc/issues/19235
        self.channel.close()

    def infer(self, df, numeric_error_threshold=0.05, n_max_categories=25):
        """Infer metadata from pandas dataframe.

        Args:
            df: a pandas DataFrame

        Returns:
            metadata: a Metadata object
        """
        data = u.df_to_csv_bytes(df)

        def request_iterator():
            req = t.InferRequest()
            req.parameters.numeric_error_threshold = numeric_error_threshold
            req.parameters.n_max_categories = n_max_categories
            yield req
            for chunk in u.chunks(data):
                yield t.InferRequest(data=chunk)

        return self.motoko.Infer(request_iterator()).metadata

    def learn(self, df, metadata):
        """Learn data.

        Args:
            df: a pandas DataFrame
            metadata: a Metadata object

        Returns:
            learn_key: used to access the trained model
            evaluation: dict with performance metrics
            decisions: list of dicts with decisions made when learning
        """
        # TODO(danj): compression?
        data = u.df_to_csv_bytes(df)

        def request_iterator():
            yield t.LearnRequest(metadata=metadata)
            for chunk in u.chunks(data):
                yield t.LearnRequest(data=chunk)

        res = self.motoko.Learn(request_iterator())
        return (
            res.learn_key,
            json.loads(res.evaluation),
            json.loads(res.decisions),
        )

    def predict(self, df, learn_key):
        """Predict on data using the model associated with `learn_key`.

        Args:
            df: a pandas DataFrame
            learn_key: a key used to access a previously trained model

        Returns:
            predictions: a list of predictions
            decisions: decisions made during prediction, i.e. omissions
        """
        # TODO(danj): compression?
        data = u.df_to_csv_bytes(df)

        def request_iterator():
            yield t.PredictRequest(learn_key=learn_key)
            for chunk in u.chunks(data):
                yield t.PredictRequest(data=chunk)

        predictions = ''
        decisions = ''
        for res in self.motoko.Predict(request_iterator()):
            if res.WhichOneof('value') == 'predictions':
                predictions += res.predictions
            else:
                decisions += res.decisions
        return json.loads(predictions), json.loads(decisions)


class AuthMetadataPlugin:
    """Motoko authorization metadata.

    source: https://github.com/grpc/grpc/tree/master/examples/python/auth
    """
    def __init__(self, email, api_key):
        self.email = email
        self.api_key = api_key

    def __call__(self, context, callback):
        """Implements authentication by passing metadata to a callback.

        Implementations of this method must not block.

        Args:
            context: An AuthMetadataContext providing information on the RPC
                that the plugin is being called to authenticate.
            callback: An AuthMetadataPluginCallback to be invoked either
                synchronously or asynchronously.
        """
        callback((('email', self.email), ('api_key', self.api_key)), None)
