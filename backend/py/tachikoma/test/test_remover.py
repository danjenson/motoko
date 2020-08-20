"""Tests Remover class."""

import numpy as np
import unittest

from remover import Remover
import utils as u


class RemoverTest(unittest.TestCase):
    """Tests Enforcer class."""
    def setUp(self):
        self.remover = Remover()
        self.df = u.iris_df()
        self.metadata = u.iris_metadata()

    def test_fit(self):
        """Tests fit."""
        self.remover.fit(self.df, self.metadata)
        # NOTE: fit shouldn't save anything, since different rows
        # may be removed in train vs. test datasets
        self.assertDictEqual(self.remover.__dict__, {})

    def test_fit_transform(self):
        """Sets some values in rows to null values and checks for removal."""
        n_before = self.df.shape[0]
        row_idx = [1, 3, 5]
        col_idx = [2, 4]
        self.df.iloc[row_idx, col_idx] = np.nan
        remover = Remover()
        dfr, _ = remover.fit_transform(self.df, self.metadata)
        n_after = dfr.shape[0]
        self.assertEqual(n_before - n_after, len(row_idx))
