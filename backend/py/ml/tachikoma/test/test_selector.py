"""Tests Selector transformer"""

import json
import unittest

import numpy as np
import pandas as pd

from enums_pb2 import BehaviorType, DataType
from selector import Selector
from types_pb2 import Attribute
import utils as u


class SelectorTest(unittest.TestCase):
    """Tests Selector transformer"""
    def test_fit_transform(self):
        """Tests fit_transform."""
        selector = Selector()
        df, metadata = u.iris_df(), u.iris_metadata()
        df['all_na'] = np.nan
        df['all_a'] = 'a'
        df['all_5'] = 5
        half_n = int(df.shape[0] / 2)
        df['na_and_5'] = [np.nan] * half_n + [5] * half_n
        df['unlisted'] = np.random.randint(10, size=df.shape[0])
        metadata.attributes.extend([
            Attribute(
                name='all_na',
                data_type=DataType.FLOAT,
                behavior_type=BehaviorType.CONTINUOUS,
            ),
            Attribute(
                name='all_a',
                data_type=DataType.STRING,
                behavior_type=BehaviorType.CATEGORICAL,
            ),
            Attribute(
                name='all_5',
                data_type=DataType.INTEGER,
                behavior_type=BehaviorType.CONTINUOUS,
            ),
            Attribute(
                name='na_and_5',
                data_type=DataType.INTEGER,
                behavior_type=BehaviorType.CONTINUOUS,
            ),
        ])
        should_remove = {'all_na', 'all_a', 'all_5', 'unlisted', 'na_and_5'}
        dft, _ = selector.fit_transform(df, metadata)
        self.assertFalse(bool(should_remove.intersection(set(dft.columns))))
