from sklearn.preprocessing import OneHotEncoder
import pandas as pd

from model_types import behavior_type, data_type, BehaviorType, DataType


class CategoricalEncoder:
    '''Encodes categorical features.'''
    def __init__(self):
        self.to_encode = []
        self.encoder = OneHotEncoder(
            # NOTE: automatically determines categories
            categories='auto',
            # NOTE: returns a full array rather than a sparse array, which
            # is easier to merge into a pandas DataFrame
            sparse=False,
            # NOTE: unseen values are encoded as 0s for each expanded binary
            # column
            handle_unknown='ignore',
        )

    def fit(self, df, target):
        '''Identifies and fits features that need to be encoded.'''
        for col in df:
            dt = data_type(df[col])
            bt = behavior_type(df[col])
            name = df[col].name
            if bt is BehaviorType.CATEGORICAL:
                if name == target or dt is DataType.BOOLEAN:
                    continue
                self.to_encode.append(name)
        if self.to_encode:
            self.encoder.fit(df[self.to_encode])
        return self

    def transform(self, df):
        '''Encodes categorical features.'''
        decisions = []
        if self.to_encode:
            # NOTE: prefix categories with feature name for readability
            column_names = [
                'is_' + feature_name + '_' + str(feature_value)
                for feature_name, feature_values in zip(
                    self.to_encode, self.encoder.categories_)
                for feature_value in feature_values
            ]
            dft = pd.DataFrame(
                self.encoder.transform(df[self.to_encode]),
                columns=column_names,
            )
            decisions.append({
                'action': 'one-hot encoded categorical columns',
                'reason': 'models require numeric features',
                'details': {
                    'originals': self.to_encode,
                    'one-hot encoded': column_names,
                },
            })
            # NOTE: need to drop index so pandas doesn't try to join on
            # indices, in case it has changed
            df = df.drop(self.to_encode, axis=1).reset_index(drop=True)
            df = pd.concat([df, dft], axis=1)
        return df, decisions

    def fit_transform(self, df, target):
        '''Runs fit and transform.'''
        return self.fit(df, target).transform(df)
