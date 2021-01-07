from sklearn.feature_extraction.text import TfidfVectorizer
import pandas as pd

from model_types import data_type, DataType


class TextEncoder:
    '''Encodes categorial features.'''
    def __init__(self, n_max_features_each=100):
        self.n_max_features_each = n_max_features_each
        self.to_encode = []
        self.encoders = {}

    def fit(self, df, target):
        '''Identifies and fits features that need to be encoded.'''
        for col in df:
            dt = data_type(df[col])
            name = df[col].name
            if dt is DataType.STRING and name != target:
                self.to_encode.append(name)
        for feature in self.to_encode:
            self.encoders[feature] = TfidfVectorizer(
                analyzer='word',
                stop_words='english',
                # TODO(danj): improve memory management
                max_features=self.n_max_features_each,
            ).fit(df[feature])
        return self

    def transform(self, df):
        '''Encodes text features.'''
        decisions = []
        dfts = []
        for attribute_name in self.to_encode:
            names = self._feature_names(attribute_name)
            sparse_matrix = \
                self.encoders[attribute_name].transform(df[attribute_name])
            dfts.append(pd.DataFrame(sparse_matrix.toarray(), columns=names))
            decisions.append({
                'action': f'encoded text feature {attribute_name}',
                'reason': 'features must be numeric for models',
                'details': {
                    'method': 'tf-idf vectorization',
                    'from': attribute_name,
                    'to': names,
                }
            })
        df.drop(self.to_encode, axis=1, inplace=True)
        return pd.concat([df] + dfts, axis=1), decisions

    def _feature_names(self, feature):
        '''Returns the expanded feature names.'''
        # NOTE: *.vocabulary_ is a mapping from terms to feature indices
        encoder = self.encoders[feature]
        names = [''] * len(encoder.vocabulary_)
        for term, idx in encoder.vocabulary_.items():
            names[idx] = feature + '_' + term
        return names

    def fit_transform(self, df, target):
        '''Runs fit and transform.'''
        return self.fit(df, target).transform(df)
