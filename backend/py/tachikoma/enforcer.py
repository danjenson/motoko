"""Enforces data types."""

import pandas as pd

from enums_pb2 import DataType
import protobuf as p


class Enforcer:
    """Enforces data types."""
    def __init__(self):
        self.to_enforce = {}

    def fit(self, df, metadata):
        """Identifies data types that must be enforced."""
        for attribute in metadata.attributes:
            if attribute.name in df.columns:
                self.to_enforce[attribute.name] = attribute.data_type
        return self

    def transform(self, df):
        """Enforces correct data types."""
        decisions = []
        for name, correct_protobuf_dtype in self.to_enforce.items():
            current_protobuf_dtype = p.python_to_protobuf_dtype(df[name].dtype)
            if current_protobuf_dtype != correct_protobuf_dtype:
                na_before = df[name].isna().sum()
                # NOTE: astype won't convert object -> number
                if (current_protobuf_dtype == DataType.STRING
                        and correct_protobuf_dtype in [
                            DataType.INTEGER, DataType.FLOAT
                        ]):
                    df.loc[:, name] = pd.to_numeric(df[name], errors='coerce')
                df.loc[:, name] = df[name].astype(
                    p.protobuf_to_python_dtype(correct_protobuf_dtype),
                    errors='ignore',
                )
                na_after = df[name].isna().sum()
                if na_after - na_before > 0:
                    decisions.append({
                        'action': f'enforce data type for {name}',
                        'reason': 'specified in metadata',
                        'details': {
                            'from': DataType.Name(current_protobuf_dtype),
                            'to': DataType.Name(correct_protobuf_dtype),
                            'new nulls': int(na_after - na_before),
                        },
                    })
        return df, decisions

    def fit_transform(self, df, metadata):
        """Runs fit and transform."""
        return self.fit(df, metadata).transform(df)
