"""Implements a selector transformer."""


class Selector:
    """Implements a selector transformer."""
    def __init__(self):
        self.one_unique = set()
        self.unlisted = set()
        self.to_remove = set()

    def fit(self, df, metadata):
        """Identifies columns to remove."""
        # all NAs are counted as 0 unique values
        self.one_unique = set(df.columns[df.nunique() <= 1])
        listed_names = {a.name for a in metadata.attributes}
        self.unlisted = {col for col in df.columns if col not in listed_names}
        self.to_remove = (self.one_unique | self.unlisted) \
            - set([metadata.target_name])
        return self

    def transform(self, df):
        """Selects only those columns marked to keep."""
        decisions = []
        if self.to_remove:
            df.drop(self.to_remove, axis=1, inplace=True)
            decisions.append({
                'action': 'removed unlisted and single-valued features',
                'reason': 'unlisted and/or useless features',
                'details': {
                    'kept': list(df.columns),
                    'removed': list(self.to_remove),
                    'single value features': list(self.one_unique),
                },
            })
        return df, decisions

    def fit_transform(self, df, metadata):
        """Runs fit and transform."""
        return self.fit(df, metadata).transform(df)
