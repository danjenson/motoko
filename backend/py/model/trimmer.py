from sklearn.feature_selection import SelectFromModel
from sklearn.linear_model import Lasso
from sklearn.svm import LinearSVC
from sklearn.utils.random import sample_without_replacement

from model_types import TaskType


class Trimmer:
    '''Trims features with low importance.

    NOTE: values must be non-null.
    '''
    def __init__(self, task_type, n_samples=10000):
        self.task_type = task_type
        self.n_samples = n_samples
        self.to_trim = {}

    def fit(self, df, target):
        '''Identifies low importance features.'''
        model = None
        if self.task_type == TaskType.CLASSIFY:
            # NOTE: see https://tinyurl.com/y62aaroa
            kwargs = {'penalty': 'l1', 'dual': False, 'max_iter': 5000}
            if df.shape[1] > df.shape[0]:
                kwargs = {'dual': True}
            model = LinearSVC(**kwargs)
        elif self.task_type == TaskType.REGRESS:
            model = Lasso()
        else:
            # cluster
            return self
        X_cols = df[[col for col in df.columns if col != target]].columns
        X, y = sample_X_y(df, target, self.n_samples)
        # see https://tinyurl.com/y5fd7bqc
        t = '0.5*median'
        mask = SelectFromModel(model, threshold=t).fit(X, y).get_support()
        to_keep = list(X_cols[mask]) + [target]
        self.to_trim = {col for col in df.columns if col not in to_keep}
        return self

    def transform(self, df):
        '''Removes low importance features.'''
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
        '''Runs fit and transform.'''
        return self.fit(df, metadata).transform(df)


def sample_X_y(df, target_name, n_samples):
    '''Sample X and y from df.'''
    if n_samples > df.shape[0]:
        n_samples = df.shape[0]
    idx = sample_without_replacement(df.shape[0], n_samples)
    return (
        df.loc[idx, df.columns != target_name],
        df.loc[idx, [target_name]].values.ravel(),
    )
