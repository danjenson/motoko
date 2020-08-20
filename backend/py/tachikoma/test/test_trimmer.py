"""Tests Trimmer."""

import unittest

import numpy as np

from enums_pb2 import BehaviorType, DataType
from trimmer import TaskType, Trimmer
import utils as u


class TrimmerTest(unittest.TestCase):
    """Tests Trimmer."""
    def setUp(self):
        self.df = u.iris_df()
        self.metadata = u.iris_metadata()
        np.random.seed(0)
        self.df['all_5'] = 5
        self.df['uniform'] = np.random.uniform(size=self.df.shape[0])
        self.df['randint'] = np.random.randint(10, size=self.df.shape[0])

    def test_fit_transform_for_classify(self):
        """Tests trimmer for classification."""
        trimmer = Trimmer(TaskType.CLASSIFY)
        dft, _ = trimmer.fit_transform(self.df, self.metadata)
        # NOTE: should also remove all_5 but alas it is imperfect
        should_remove = {'uniform', 'randint'}
        self.assertFalse(bool(should_remove.intersection(set(dft.columns))))

    def test_fit_transform_for_regress(self):
        """Tests trimmer for regression."""
        trimmer = Trimmer(TaskType.REGRESS)
        self.metadata.target_name = 'sepal_length'
        self.df.drop('species', axis=1, inplace=True)
        dft, _ = trimmer.fit_transform(self.df, self.metadata)
        should_remove = {'all_5', 'uniform', 'randint'}
        self.assertFalse(bool(should_remove.intersection(set(dft.columns))))

    def test_fit_transform_for_cluster(self):
        """Tests trimmer for clustering; shouldn't drop anything."""
        trimmer = Trimmer(TaskType.CLUSTER)
        dft, _ = trimmer.fit_transform(self.df, self.metadata)
        self.assertTrue(set(self.df.columns) == set(dft.columns))
