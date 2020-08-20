"""Implements a feature trimmer as part of a fit-transform pipeline."""
import numpy as np
from sklearn.feature_selection import SelectFromModel
from sklearn.linear_model import Lasso
from sklearn.svm import LinearSVC

from enums_pb2 import TaskType
import utils as u


class Trimmer:
    """Trims features with low importance.

    NOTE: values must be non-null.
    """
    def __init__(self, task_type, n_samples=10000):
        self.task_type = task_type
        self.n_samples = n_samples
        self.to_trim = {}

    def fit(self, df, metadata):
        """Identifies low importance features."""
        model = None
        if self.task_type == TaskType.CLASSIFY:
            # NOTE: see https://tinyurl.com/y62aaroa
            kwargs = {'penalty': 'l1', 'dual': False}
            if df.shape[1] > df.shape[0]:
                kwargs = {'dual': True}
            model = LinearSVC(**kwargs)
        elif self.task_type == TaskType.REGRESS:
            model = Lasso()
        else:
            # cluster
            return self
        X_cols = df[[col for col in df.columns
                     if col != metadata.target_name]].columns
        X, y = u.sample_X_y(df, metadata.target_name, self.n_samples)
        # NOTE: select all features with importance >= 0.5*mean
        # see https://tinyurl.com/y5fd7bqc
        mask = SelectFromModel(model, "0.5*mean").fit(X, y).get_support()
        to_keep = list(X_cols[mask]) + [metadata.target_name]
        self.to_trim = {col for col in df.columns if col not in to_keep}
        return self

    def transform(self, df):
        """Removes low importance features."""
        decisions = []
        if self.to_trim:
            df.drop(self.to_trim, axis=1, inplace=True)
            decisions.append({
                'action': 'removed low importance features',
                'reason': 'improves speed and accuracy of models',
                'details': {
                    'kept': list(df.columns),
                    'removed': list(self.to_trim),
                },
            })
        return df, decisions

    def fit_transform(self, df, metadata):
        """Runs fit and transform."""
        return self.fit(df, metadata).transform(df)
