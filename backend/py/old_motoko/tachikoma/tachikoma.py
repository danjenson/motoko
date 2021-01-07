"""Tachikoma."""
import json

from inferer import Inferer
from learner import learn, predict
from tachikoma_pb2_grpc import TachikomaServicer
import types_pb2 as t
import utils as u


class Tachikoma(TachikomaServicer):
    """Tachikoma."""
    def __init__(self, db):
        self.db = db
        self.inferer = Inferer(db)

    def Infer(self, request_iterator, context=None):
        """Infers metadata."""
        numeric_error_threshold = 0.05
        n_max_categories = 25
        data = b''
        for req in request_iterator:
            if req.WhichOneof('value') == 'parameters':
                numeric_error_threshold = req.parameters.numeric_error_threshold
                n_max_categories = req.parameters.n_max_categories
            else:
                data += req.data
        # TODO(danj): introduce TypedBytes to be more flexible
        df = u.csv_bytes_to_df(data)
        return t.InferResponse(metadata=self.inferer.infer(
            df, numeric_error_threshold, n_max_categories))

    def Learn(self, request_iterator, context=None):
        """Learns data."""
        metadata = None
        data = b''
        for req in request_iterator:
            if req.WhichOneof('value') == 'metadata':
                metadata = req.metadata
            else:
                data += req.data
        df = u.csv_bytes_to_df(data)
        _, transformers, estimator, evaluation, decisions = learn(df, metadata)
        learn_key = self.db.store(
            metadata,
            transformers,
            estimator,
            evaluation,
            decisions,
        )
        return t.LearnResponse(
            learn_key=learn_key,
            evaluation=json.dumps(evaluation),
            decisions=json.dumps(decisions),
        )

    def Predict(self, request_iterator, context=None):
        """Predicts using data and learner corresponding to `learn_key`."""
        learn_key = ''
        data = b''
        for req in request_iterator:
            if req.WhichOneof('value') == 'learn_key':
                learn_key = req.learn_key
            else:
                data += req.data
        # TODO(danj): insert_and_shift skipped predictions and failed row loads
        df = u.csv_bytes_to_df(data)
        d = self.db.get(learn_key, ['metadata', 'transformers', 'estimator'])
        predictions, decisions = predict(
            df,
            d['metadata'].target_name,
            d['transformers'],
            d['estimator'],
        )

        def response_iterator():
            for chunk in u.chunks(json.dumps(list(predictions))):
                yield t.PredictResponse(predictions=chunk)
            for chunk in u.chunks(json.dumps(decisions)):
                yield t.PredictResponse(decisions=chunk)

        return response_iterator()
