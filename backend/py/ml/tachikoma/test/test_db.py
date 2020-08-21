"""Tests simple wrapper for db actions"""
import datetime as dt
import unittest

from db import DB
import utils as u


class DBTest(unittest.TestCase):
    """Tests simple wrapper for db actions"""
    def test_all(self):
        """Tests all actions"""

        # connect to db
        db = DB('motoko_test')

        # create test objs
        metadata = u.iris_metadata()
        estimator = u.iris_estimator()
        transformers, evaluation, decisions = [], {}, {}
        learn_key = db.store(
            metadata,
            transformers,
            estimator,
            evaluation,
            decisions,
        )

        # test storage
        m = db.metadata_since(dt.datetime.now() - dt.timedelta(seconds=5))[0]
        self.assertEqual(metadata.target_name, m.target_name)
        res = db.get(learn_key, ['metadata', 'estimator'])
        self.assertEqual(metadata.target_name, res['metadata'].target_name)
        self.assertEqual(
            res['estimator'].get_params()['gamma'],
            estimator.get_params()['gamma'],
        )

        # clean up
        db.clear()
