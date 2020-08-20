"""Tests Enforcer class."""

import unittest

import numpy as np

from enforcer import Enforcer
import utils as u


class EnforcerTest(unittest.TestCase):
    """Tests Enforcer class."""
    def test_fit_transform(self):
        """Tests fit_transform."""
        enforcer = Enforcer()
        df, metadata = u.iris_df(), u.iris_metadata()
        idx = 1
        attr = 'sepal_length'
        val = 'banana'
        df.loc[idx, attr] = val
        dft, _ = enforcer.fit_transform(df, metadata)
        self.assertTrue(np.isnan(dft.loc[idx, attr]))
