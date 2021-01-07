"""Tests ContinuousScaler class."""

import unittest

import numpy as np

from continuous_scaler import ContinuousScaler
from enums_pb2 import BehaviorType, TaskType
import utils as u


class ContinuousScalerTest(unittest.TestCase):
    """Tests CategoricalEncoder class."""
    def setUp(self):
        self.df = u.iris_df()
        self.metadata = u.iris_metadata()
        self.continuous_attributes = []
        for attribute in self.metadata.attributes:
            if attribute.behavior_type == BehaviorType.CONTINUOUS:
                self.continuous_attributes.append(attribute.name)

    def test_target_not_scaled(self):
        """Tests that target attribute is not scaled."""
        cs = ContinuousScaler(TaskType.CLASSIFY)
        dft, _ = cs.fit_transform(self.df, self.metadata)
        self.assertTrue(np.all(self.df['species'] == dft['species']))

    def test_task_classify(self):
        """Test that the means are all 0 and standard deviations are 1."""
        self._test_classify_or_regress(TaskType.CLASSIFY)

    def test_task_regress(self):
        """Test that the means are all 0 and standard deviations are 1."""
        self._test_classify_or_regress(TaskType.REGRESS)

    def _test_classify_or_regress(self, task_type):
        cs = ContinuousScaler(task_type)
        dft, _ = cs.fit_transform(self.df, self.metadata)
        dftc = dft[self.continuous_attributes]
        zeros = np.zeros(len(self.continuous_attributes))
        ones = np.ones(len(self.continuous_attributes))
        means_0 = np.all(np.isclose(dftc.mean(), zeros))
        sds_1 = np.all(np.isclose(dftc.std(), ones, atol=0.01))
        self.assertTrue(means_0 and sds_1)

    def test_task_cluster(self):
        """Test that all scaled values are between 0 and 1."""
        cs = ContinuousScaler(TaskType.CLUSTER)
        dft, _ = cs.fit_transform(self.df, self.metadata)
        dftc = dft[self.continuous_attributes]
        mins_ge_0 = np.all(dftc.min() >= 0.0)
        maxs_le_1 = np.all(dftc.max() <= 1.0)
        self.assertTrue(mins_ge_0 and maxs_le_1)
