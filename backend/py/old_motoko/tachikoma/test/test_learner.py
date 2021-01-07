"""Tests TransformerService."""

import unittest

import numpy as np

from enums_pb2 import BehaviorType, DataType, TaskType
from types_pb2 import Attribute
import learner
import utils as u


class LearnerTest(unittest.TestCase):
    """Tests TransformerService."""
    def test_learn(self):
        """Tests that the data is either standardized or normalized."""
        # perturb the dataset a bit
        df = u.iris_df()
        row_idx = [1, 3, 5]
        col_idx = [2, 4]
        df.iloc[row_idx, col_idx] = np.nan
        df['all_na'] = np.nan
        df['all_a'] = 'a'
        df['all_5'] = 5
        df['unlisted'] = np.random.randint(10, size=df.shape[0])
        # update metadata
        metadata = u.iris_metadata()
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
        ])
        for task_type in TaskType.values():
            if task_type == TaskType.CLASSIFY:
                metadata.has_target = True
                metadata.target_name = 'species'
            elif task_type == TaskType.REGRESS:
                metadata.has_target = True
                metadata.target_name = 'petal_length'
            elif task_type == TaskType.CLUSTER:
                metadata.has_target = False
                metadata.target_name = ''
            dft, _, _, evaluation, _ = learner.learn(df, metadata)
            should_remove = {'all_na', 'all_a', 'all_5', 'unlisted'}
            dft_cols = set(dft.columns)
            required_accuracy = 0.9
            if task_type in [TaskType.CLASSIFY, TaskType.REGRESS]:
                self.assertFalse(bool(should_remove.intersection(dft_cols)))
            if task_type == TaskType.REGRESS:
                # actually hard to predict petal_length
                required_accuracy = 0.45
                should_standardize = []
                for attribute in metadata.attributes:
                    if (attribute.behavior_type == BehaviorType.CONTINUOUS
                            and attribute.name in dft_cols
                            and attribute.name != metadata.target_name):
                        should_standardize.append(attribute.name)
                zeros = np.zeros(len(should_standardize))
                ones = np.ones(len(should_standardize))
                dft_standardized = dft[should_standardize]
                means_0 = np.isclose(dft_standardized.mean(), zeros)
                sds_1 = np.isclose(dft_standardized.std(), ones, atol=0.01)
                is_standardized = np.all(np.logical_and(means_0, sds_1))
                self.assertTrue(is_standardized)
            elif task_type == TaskType.CLUSTER:
                mins_ge_0 = (dft.min() >= 0.0).values
                maxs_le_1 = (dft.max() <= 1.0).values
                is_normalized = np.all(np.logical_and(mins_ge_0, maxs_le_1))
                self.assertTrue(is_normalized)
            self.assertGreaterEqual(evaluation['accuracy'], required_accuracy)
