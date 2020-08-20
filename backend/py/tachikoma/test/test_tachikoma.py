"""Tests Tachikoma."""
import json
import pickle
import unittest

from db import DB
from tachikoma import Tachikoma
import types_pb2 as t
import utils as u


class TestTachikoma(unittest.TestCase):
    """Tests Tachikoma."""
    @classmethod
    def setUpClass(cls):
        cls.db = DB('motoko_test')
        cls.tk = Tachikoma(cls.db)
        cls.metadata = u.iris_metadata()
        cls.db.store(cls.metadata, [], {}, {}, {})
        with open('test/data/iris.csv', 'rb') as f:
            cls.data = f.read()

    @classmethod
    def tearDownClass(cls):
        cls.db.clear()

    def test_infer(self):
        """Tests Infer."""
        def request_iterator():
            yield t.InferRequest(parameters=t.InferRequest.Parameters(
                numeric_error_threshold=0.05,
                n_max_categories=25,
            ))
            for chunk in u.chunks(self.data):
                yield t.InferRequest(data=chunk)

        res = self.tk.Infer(request_iterator())
        self.assertEqual(res.metadata.target_name, self.metadata.target_name)

    def test_learn_and_predict(self):
        """Tests Learn."""
        def learn_request_iterator():
            yield t.LearnRequest(metadata=self.metadata)
            for chunk in u.chunks(self.data):
                yield t.LearnRequest(data=chunk)

        res = self.tk.Learn(learn_request_iterator())
        self.assertGreater(len(res.learn_key), 10)
        self.assertGreater(json.loads(res.evaluation)['accuracy'], 0.9)
        self.assertEqual(len(json.loads(res.decisions)), 5)

        def predict_request_iterator():
            yield t.PredictRequest(learn_key=res.learn_key)
            for chunk in u.chunks(self.data):
                yield t.PredictRequest(data=chunk)

        # TODO(danj): test gaps in predictions (i.e. error lines)
        predictions = ''
        decisions = ''
        for res in self.tk.Predict(predict_request_iterator()):
            if res.WhichOneof('value') == 'predictions':
                predictions += res.predictions
            else:
                decisions += res.decisions
        predictions = json.loads(predictions)
        decisions = json.loads(decisions)
        labels = {'setosa', 'versicolor', 'virginica'}
        self.assertFalse(set(res.predictions) - labels)
