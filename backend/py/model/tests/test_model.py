import unittest

import numpy as np

from model_types import behavior_type, BehaviorType, TaskType
from model import model
import test_utils as tu


class ModelTest(unittest.TestCase):
    '''Tests TransformerService.'''
    def test_model(self):
        '''Tests that the data is either standardized or normalized.'''
        np.random.seed(0)
        # perturb the dataset a bit
        df = tu.iris_df()
        row_idx = [1, 3, 5]
        col_idx = [2, 4]
        df.iloc[row_idx, col_idx] = np.nan
        df['all_na'] = np.nan
        df['all_a'] = 'a'
        df['all_5'] = 5
        df['unlisted'] = np.random.randint(10, size=df.shape[0])
        # update target
        for task_type in TaskType:
            if task_type == TaskType.CLUSTER:
                target = None
            elif task_type == TaskType.CLASSIFY:
                target = 'species'
            elif task_type == TaskType.REGRESS:
                target = 'petal_width'
            dft, m, evaluation, decisions = model(df, target)
            should_remove = {'all_na', 'all_a', 'all_5'}
            dft_cols = set(dft.columns)
            required_accuracy = 0.85
            if task_type in [TaskType.CLASSIFY, TaskType.REGRESS]:
                self.assertFalse(bool(should_remove.intersection(dft_cols)))
            if task_type is TaskType.CLUSTER:
                mins_ge_0 = (dft.min() >= 0.0).values
                maxs_le_1 = (dft.max() <= 1.0).values
                is_normalized = np.all(np.logical_and(mins_ge_0, maxs_le_1))
                self.assertTrue(is_normalized)
                self.assertGreaterEqual(
                    evaluation['normalized silhouette score'],
                    required_accuracy)
            elif task_type is TaskType.CLASSIFY:
                self.assertGreaterEqual(evaluation['accuracy'],
                                        required_accuracy)
            elif task_type is TaskType.REGRESS:
                # actually hard to predict petal_length
                required_accuracy = 0.45
                should_standardize = []
                for col in dft:
                    bt = behavior_type(dft[col])
                    name = dft[col].name
                    if (bt is BehaviorType.CONTINUOUS and name != target):
                        should_standardize.append(name)
                zeros = np.zeros(len(should_standardize))
                ones = np.ones(len(should_standardize))
                dft_standardized = dft[should_standardize]
                means_0 = np.isclose(dft_standardized.mean(), zeros)
                sds_1 = np.isclose(dft_standardized.std(), ones, atol=0.01)
                is_standardized = np.all(np.logical_and(means_0, sds_1))
                self.assertTrue(is_standardized)
                self.assertGreaterEqual(evaluation['R^2'], required_accuracy)
