"""Tests Inferer."""
import unittest

from inferer import Inferer
from db import DB
import utils as u


class InfererTest(unittest.TestCase):
    """Tests Inferer."""
    def test_all(self):
        """Tests Inferer."""
        db = DB('motoko_test')
        metadata = u.iris_metadata()
        _ = db.store(metadata, [], {}, {}, {})
        inferer = Inferer(db)
        df = u.iris_df()
        m = inferer.infer(df, 0.05, 25)
        self.assertEqual(m.target_name, metadata.target_name)
        self.assertEqual(
            m.attributes[0].data_type,
            metadata.attributes[0].data_type,
        )
        self.assertEqual(
            m.attributes[2].behavior_type,
            metadata.attributes[2].behavior_type,
        )
        db.clear()
