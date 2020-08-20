"""Encodes categorical features."""

from sklearn.preprocessing import OneHotEncoder
import pandas as pd

from enums_pb2 import BehaviorType, DataType


class CategoricalEncoder:
    """Encodes categorical features."""
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

    def fit(self, df, metadata):
        """Identifies and fits features that need to be encoded."""
        for attribute in metadata.attributes:
            if attribute.name in df.columns:
                if attribute.behavior_type == BehaviorType.CATEGORICAL:
                    if attribute.data_type not in [
                            DataType.BOOLEAN,
                            DataType.INTEGER,
                            DataType.STRING,
                    ]:
                        msg = '{t} unsupported categorical data type'.format(
                            t=DataType.Name(attribute.data_type))
                        raise NotImplementedError(msg)
                    if attribute.name == metadata.target_name:
                        continue
                    elif attribute.data_type == DataType.BOOLEAN:
                        continue
                    else:
                        self.to_encode.append(attribute.name)
        if self.to_encode:
            self.encoder.fit(df[self.to_encode])
        return self

    def transform(self, df):
        """Encodes categorical features."""
        decisions = []
        if self.to_encode:
            # NOTE: prefix categories with attribute name for readability
            column_names = [
                'is_' + attribute_name + '_' + str(attribute_value)
                for attribute_name, attribute_values in zip(
                    self.to_encode, self.encoder.categories_)
                for attribute_value in attribute_values
            ]
            dft = pd.DataFrame(
                self.encoder.transform(df[self.to_encode]),
                columns=column_names,
            )
            decisions.append({
                'action': 'binarized categorical columns',
                'reason': 'models require numeric features',
                'details': {
                    'originals': self.to_encode,
                    'binarized': column_names,
                },
            })
            # NOTE: need to drop index so pandas doesn't try to join on
            # indices, in case it has changed
            df = df.drop(self.to_encode, axis=1).reset_index(drop=True)
            df = pd.concat([df, dft], axis=1)
        return df, decisions

    def fit_transform(self, df, metadata):
        """Runs fit and transform."""
        return self.fit(df, metadata).transform(df)
