"""Removes records with null values."""

import pandas as pd


class Remover:
    """Removes records with null values."""
    def fit(self, df, metadata):
        """Fit does nothing here but adheres to the fit-transform API."""
        # NOTE: don't fit anything here because transform could be
        # called on training or testing data, and you don't want to remove
        # the same indices in both cases
        return self

    def transform(self, df):
        """Removes all null features and records where any value is null."""
        decisions = []
        na_indices = pd.isnull(df).any(1).nonzero()[0]
        n = len(na_indices)
        pct = n / df.shape[0] * 100
        if na_indices.size > 0:
            df = df.drop(na_indices).reset_index(drop=True)
            decisions.append({
                'action': f'removed {n} rows ({pct:.0%}) due to missing data',
                'reason': 'NA values will cause the model to fail',
                'details': {
                    # np.int64 is not JSON serializable
                    'removed': list(map(int, na_indices))
                },
            })
        return df, decisions

    def fit_transform(self, df, metadata):
        """Runs fit and transform."""
        return self.fit(df, metadata).transform(df)
