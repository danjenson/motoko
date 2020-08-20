"""An implementation of a continuous scaler."""
import re

from sklearn.preprocessing import Normalizer, StandardScaler
import numpy as np
import pandas as pd

from enums_pb2 import TaskType


class ContinuousScaler:
    """Scales continuous features."""
    def __init__(self, task_type):
        self.task_type = task_type
        self.to_scale = []
        self.scaler = StandardScaler()
        if self.task_type == TaskType.CLUSTER:
            self.scaler = Normalizer()

    def fit(self, df, metadata):
        """Identifies and fits features that need to be scaled."""
        target_name = metadata.target_name
        bool_regex = re.compile('^is_.*')
        for a_name, dtype in df.dtypes.iteritems():
            if bool_regex.match(a_name) and df[a_name].nunique() == 2:
                # don't standarize boolean columns
                continue
            if a_name != target_name and np.issubdtype(dtype, np.number):
                self.to_scale.append(a_name)
        if self.to_scale:
            self.scaler.fit(df[self.to_scale])
        return self

    def transform(self, df):
        """Scales continuous features."""
        decisions = []
        if not self.to_scale:
            return df, decisions
        dft = pd.DataFrame(
            self.scaler.transform(df[self.to_scale]),
            columns=self.to_scale,
        )
        df.drop(self.to_scale, axis=1, inplace=True)
        action = 'standardized non-binary continuous features'
        reason = 'improves performance for classification/regression tasks'
        if self.task_type == TaskType.CLUSTER:
            action = 'normalized non-binary continuous features'
            reason = 'clustering is extremely sensitive to large values'
        decisions.append({
            'action': action,
            'reason': reason,
            'details': {
                'scaled': self.to_scale,
            },
        })
        return pd.concat([df, dft], axis=1), decisions

    def fit_transform(self, df, metadata):
        """Runs fit and transform."""
        return self.fit(df, metadata).transform(df)
