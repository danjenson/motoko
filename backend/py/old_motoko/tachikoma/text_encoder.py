"""An implementation of a text encoder."""

from sklearn.feature_extraction.text import TfidfVectorizer
import pandas as pd

from enums_pb2 import BehaviorType, DataType


class TextEncoder:
    """Encodes categorial features."""
    def __init__(self, n_max_features_each=100):
        self.n_max_features_each = n_max_features_each
        self.to_encode = []
        self.encoders = {}

    def fit(self, df, metadata):
        """Identifies and fits features that need to be encoded."""
        for attribute in metadata.attributes:
            if attribute.name in df.columns:
                if attribute.behavior_type == BehaviorType.TEXT:
                    if attribute.data_type != DataType.STRING:
                        msg = '{t} unsupported text data type'.format(
                            t=DataType.Name(attribute.data_type))
                        raise NotImplementedError(msg)
                    self.to_encode.append(attribute.name)
        for attribute_name in self.to_encode:
            self.encoders[attribute_name] = TfidfVectorizer(
                analyzer='word',
                stop_words='english',
                # TODO(danj): improve memory management
                max_features=self.n_max_features_each,
            ).fit(df[attribute_name])
        return self

    def transform(self, df):
        """Encodes text features."""
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

    def _feature_names(self, attribute_name):
        """Returns the expanded feature names prefixed by the attribute_name."""
        # NOTE: *.vocabulary_ is a mapping from terms to feature indices
        encoder = self.encoders[attribute_name]
        names = [''] * len(encoder.vocabulary_)
        for term, idx in encoder.vocabulary_.items():
            names[idx] = attribute_name + '_' + term
        return names

    def fit_transform(self, df, metadata):
        """Runs fit and transform."""
        return self.fit(df, metadata).transform(df)
