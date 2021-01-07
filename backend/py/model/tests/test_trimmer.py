import unittest

import numpy as np

from model_types import BehaviorType, DataType
from trimmer import TaskType, Trimmer
import test_utils as tu


class TrimmerTest(unittest.TestCase):
    '''Tests Trimmer.'''
    def setUp(self):
        np.random.seed(0)
        df = tu.iris_df()
        df['all_5'] = 5
        df['uniform'] = np.random.uniform(size=df.shape[0])
        df['randint'] = np.random.randint(10, size=df.shape[0])
        self.df = df
        self.target = 'species'

    def test_fit_transform_for_cluster(self):
        '''Tests trimmer for clustering; shouldn't drop anything.'''
        trimmer = Trimmer(TaskType.CLUSTER)
        dft, _ = trimmer.fit_transform(self.df, self.target)
        self.assertTrue(set(self.df.columns) == set(dft.columns))

    @unittest.skip('imperfect test')
    def test_fit_transform_for_classify(self):
        '''Tests trimmer for classification.'''
        trimmer = Trimmer(TaskType.CLASSIFY)
        dft, _ = trimmer.fit_transform(self.df, self.target)
        should_remove = {'all_5', 'uniform', 'randint'}
        self.assertFalse(bool(should_remove.intersection(set(dft.columns))))

    @unittest.skip('imperfect test')
    def test_fit_transform_for_regress(self):
        '''Tests trimmer for regression.'''
        trimmer = Trimmer(TaskType.REGRESS)
        self.df.drop('species', axis=1, inplace=True)
        dft, _ = trimmer.fit_transform(self.df, 'sepal_length')
        should_remove = {'all_5', 'uniform', 'randint'}
        self.assertFalse(bool(should_remove.intersection(set(dft.columns))))
