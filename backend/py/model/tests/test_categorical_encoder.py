'''Tests CategoricalEncoder'''

import unittest

import numpy as np

from categorical_encoder import CategoricalEncoder
from model_types import BehaviorType, DataType
import test_utils as tu


class CategoricalEncoderTest(unittest.TestCase):
    '''Tests CategoricalEncoder'''
    def setUp(self):
        self.df = tu.iris_df()
        self.target = 'species'
        self.ce = CategoricalEncoder()

    def test_target_not_encoded(self):
        '''Tests that target attribute is not encoded into target_{1...}.'''
        expected_values = [self.target]
        dft, _ = self.ce.fit_transform(self.df, self.target)
        actual_values = [
            col for col in dft.columns if col.startswith(self.target)
        ]
        self.assertListEqual(actual_values, expected_values)

    def test_string_encoding(self):
        '''Tests that features are properly encoded.'''
        choices = ['a', 'b', 'c']
        edit_col = 'sepal_length'
        self.df[edit_col] = np.random.choice(choices, self.df.shape[0])
        dft, _ = self.ce.fit_transform(self.df, self.target)
        expected_new_cols = {'is_' + edit_col + '_' + c for c in choices}
        diff = expected_new_cols - set(dft.columns)
        # NOTE: checks that the new columns are in the DataFrame
        self.assertSetEqual(diff, set())
        # NOTE: checks that the original column has been removed
        self.assertNotIn(edit_col, dft.columns)

    @unittest.skip('not yet supported')
    def test_integer_encoding(self):
        '''Tests that integer features are properly encoded.'''
        choices = [4.0, 5.0, 6.0]
        edit_col = 'sepal_length'
        self.df[edit_col] = np.random.choice(choices, self.df.shape[0])
        dft, _ = self.ce.fit_transform(self.df, self.target)
        expected_new_cols = {'is_' + edit_col + '_' + str(c) for c in choices}
        diff = expected_new_cols - set(dft.columns)
        # NOTE: checks that the new columns are in the DataFrame
        self.assertSetEqual(diff, set())
        # NOTE: checks that the original column has been removed
        self.assertNotIn(edit_col, dft.columns)

    def test_unseen_category(self):
        '''Test that unseen categories register 0 in every column.'''
        train_choices = ['a', 'b', 'c']
        test_choices = ['a', 'b', 'c', 'd']
        edit_col = 'sepal_length'
        self.df[edit_col] = np.random.choice(train_choices, self.df.shape[0])
        self.ce.fit(self.df, self.target)
        df2 = self.df.copy()
        df2[edit_col] = np.random.choice(test_choices, self.df.shape[0])
        df2t, _ = self.ce.transform(df2)
        self.assertFalse(df2t.isnull().values.any())
