"""Tests Replacer class."""

import unittest

import numpy as np

from replacer import Replacer
import utils as u


class ReplacerTest(unittest.TestCase):
    """Tests Replacer class."""
    @classmethod
    def setUpClass(cls):
        cls.metadata = u.iris_metadata()

    def setUp(self):
        self.replacer = Replacer()
        self.df = u.iris_df()

    def test_fit_with_function_after_datum_replacement(self):
        """Tests fit after using a function after a replacement."""
        row_idx = [1, 2]
        col_idx = ['sepal_length']
        self.df.loc[row_idx, col_idx] = np.nan
        self.replacer.fit(self.df, self.metadata)
        golden_value = 5.9925675675675665
        _, to_value = self.replacer.replacements['sepal_length'][1]
        self.assertAlmostEqual(to_value, golden_value)

    def test_fit_transform_with_function_after_datum_replacement(self):
        """Tests fit_transform after using a function after a replacement."""
        row_idx = [1, 2]
        col_idx = ['sepal_length']
        self.df.loc[row_idx, col_idx] = np.nan
        df, _ = self.replacer.fit_transform(self.df, self.metadata)
        # NOTE: the mean should stay the same
        golden_value = 5.9925675675675665
        actual_value = df.sepal_length.mean()
        self.assertAlmostEqual(actual_value, golden_value)
