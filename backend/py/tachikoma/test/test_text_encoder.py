"""Tests TextEncoder."""

import unittest

import numpy as np

from enums_pb2 import BehaviorType, DataType
from text_encoder import TextEncoder
import utils as u


class TextEncoderTest(unittest.TestCase):
    """Tests TextEncoder."""
    def setUp(self):
        self.df = u.iris_df(with_news=True)
        # NOTE: created with the following
        # from sklearn.datasets import fetch_20newsgroups
        # df['news'] = fetch_20newsgroups(subset='train')['data'][:df.shape[0]]
        self.metadata = u.iris_metadata(with_news=True)
        self.te = TextEncoder()

    def test_invalid_text(self):
        """Tests that raises an error when not string data type."""
        for attribute in self.metadata.attributes:
            if attribute.name == 'news':
                attribute.behavior_type = BehaviorType.TEXT
                attribute.data_type = DataType.INTEGER
        with self.assertRaises(NotImplementedError):
            self.te.fit(self.df, self.metadata)

    def test_target_not_encoded(self):
        """Tests that target attribute is not encoded into target_{1...}."""
        expected_values = ['species']
        dft, _ = self.te.fit_transform(self.df, self.metadata)
        actual_values = [
            col for col in dft.columns if col.startswith('species')
        ]
        self.assertListEqual(actual_values, expected_values)

    def test_n_max_attributes_each(self):
        """Tests that it keeps max n_max_attributes_each per feature."""
        n_max_attributes_each = 5
        te = TextEncoder(n_max_attributes_each)
        dft, _ = te.fit_transform(self.df, self.metadata)
        news_attributes = [v for v in dft.columns if v.startswith('news')]
        self.assertEqual(len(news_attributes), n_max_attributes_each)

    def test_text_encoding(self):
        """Test that it encoded values to between 0 and 1."""
        prefix = ['news']
        dft, _ = self.te.fit_transform(self.df, self.metadata)
        news_attributes = [v for v in dft.columns if v.startswith('news')]
        means = dft[news_attributes].mean()
        self.assertTrue(np.all(means > 0) and np.all(means < 1))
