import pandas as pd


class Remover:
    '''Removes records with null values.'''
    def fit(self, df, metadata):
        '''Fit does nothing here but adheres to the fit-transform API.'''
        # NOTE: don't fit anything here because transform could be
        # called on training or testing data, and you don't want to remove
        # the same indices in both cases
        return self

    def transform(self, df):
        '''Removes unique and >= 50% null features and records with nulls.'''
        decisions = []
        unique_features = list(df.columns[df.nunique() == 1])
        if unique_features:
            df = df.drop(unique_features, axis=1)
            decisions.append({
                'action': 'removed features with only 1 unique value',
                'reason': 'these features will not contribute to performance',
                'details': {
                    'features': unique_features,
                },
            })
        na_rates = df.isnull().sum() / df.shape[0]
        bad_features = list(na_rates.index[na_rates > 0.5])
        if bad_features:
            df = df.drop(bad_features, axis=1)
            decisions.append({
                'action': 'removed features that were more than 50% null',
                'reason': 'NA rows are removed',
                'details': {
                    'features': bad_features
                }
            })
        na_indices = df.isnull().any(1).values.nonzero()[0]
        n = len(na_indices)
        rate = n / df.shape[0]
        if na_indices.size > 0:
            df = df.drop(na_indices).reset_index(drop=True)
            decisions.append({
                'action': f'removed {n} rows ({rate:.0%}) due to missing data',
                'reason': 'NA values will cause the model to fail',
                'details': {
                    # np.int64 is not JSON serializable
                    'removed row indices': list(map(int, na_indices))
                },
            })
        return df, decisions

    def fit_transform(self, df, target):
        '''Runs fit and transform.'''
        return self.fit(df, target).transform(df)
