"""Tests CategoricalEncoder"""

import unittest

import numpy as np

from categorical_encoder import CategoricalEncoder
import utils as u

from enums_pb2 import BehaviorType, DataType


class CategoricalEncoderTest(unittest.TestCase):
    """Tests CategoricalEncoder"""
    def setUp(self):
        self.df = u.iris_df()
        self.metadata = u.iris_metadata()
        self.ce = CategoricalEncoder()

    def test_invalid_categorical(self):
        """Should raise error when trying to categorically encode float."""
        for attribute in self.metadata.attributes:
            if attribute.name == 'sepal_length':
                attribute.behavior_type = BehaviorType.CATEGORICAL
                attribute.data_type = DataType.FLOAT
        with self.assertRaises(NotImplementedError):
            self.ce.fit(self.df, self.metadata)

    def test_target_not_encoded(self):
        """Tests that target attribute is not encoded into target_{1...}."""
        expected_values = ['species']
        dft, _ = self.ce.fit_transform(self.df, self.metadata)
        actual_values = [
            col for col in dft.columns if col.startswith('species')
        ]
        self.assertListEqual(actual_values, expected_values)

    def test_string_encoding(self):
        """Tests that features are properly encoded."""
        choices = ['a', 'b', 'c']
        edit_col = 'sepal_length'
        self.df[edit_col] = np.random.choice(choices, self.df.shape[0])
        for attribute in self.metadata.attributes:
            if attribute.name == edit_col:
                attribute.behavior_type = BehaviorType.CATEGORICAL
                attribute.data_type = DataType.STRING
        dft, _ = self.ce.fit_transform(self.df, self.metadata)
        expected_new_cols = {'is_' + edit_col + '_' + c for c in choices}
        diff = expected_new_cols - set(dft.columns)
        # NOTE: checks that the new columns are in the DataFrame
        self.assertSetEqual(diff, set())
        # NOTE: checks that the original column has been removed
        self.assertNotIn(edit_col, dft.columns)

    def test_integer_encoding(self):
        """Tests that integer features are properly encoded."""
        choices = [4.0, 5.0, 6.0]
        edit_col = 'sepal_length'
        self.df[edit_col] = np.random.choice(choices, self.df.shape[0])
        for attribute in self.metadata.attributes:
            if attribute.name == edit_col:
                attribute.behavior_type = BehaviorType.CATEGORICAL
                attribute.data_type = DataType.INTEGER
        dft, _ = self.ce.fit_transform(self.df, self.metadata)
        expected_new_cols = {'is_' + edit_col + '_' + str(c) for c in choices}
        diff = expected_new_cols - set(dft.columns)
        # NOTE: checks that the new columns are in the DataFrame
        self.assertSetEqual(diff, set())
        # NOTE: checks that the original column has been removed
        self.assertNotIn(edit_col, dft.columns)

    def test_unseen_category(self):
        """Test that unseen categories register 0 in every column."""
        train_choices = [4.0, 5.0, 6.0]
        test_choices = [4.0, 5.0, 6.0, 7.0]
        edit_col = 'sepal_length'
        self.df[edit_col] = np.random.choice(train_choices, self.df.shape[0])
        for attribute in self.metadata.attributes:
            if attribute.name == edit_col:
                attribute.behavior_type = BehaviorType.CATEGORICAL
                attribute.data_type = DataType.INTEGER
        self.ce.fit(self.df, self.metadata)
        df2 = self.df.copy()
        df2[edit_col] = np.random.choice(test_choices, self.df.shape[0])
        df2t, _ = self.ce.transform(df2)
        self.assertFalse(df2t.isnull().values.any())
