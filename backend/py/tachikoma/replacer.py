"""Replaces values as specified in metadata."""

from collections import defaultdict
from collections import namedtuple

from scipy import stats
import numpy as np

import protobuf as p
import utils as u


class Replacer:
    """Replaces values as specified in metadata."""
    def __init__(self):
        self.replacements = defaultdict(list)

    def fit(self, df, metadata):
        """Identifies values for replacement."""
        for attribute_name, rs in extract_replacements(df, metadata).items():
            # NOTE: to fit values for functions, all steps leading up
            # to the function application must first be applied, i.e.
            # if the client asks to replace 1 with 10 and later replace
            # missing values with the mean, the mean should be calculated
            # after the first value replacement; thus, all replacements
            # must be done up until the last function application to resolve
            # fitted function values. All of these acrobatics are to create
            # concrete replacement values for the transform step.
            last_func_idx = index_of_last_function(rs)
            v = df[attribute_name]
            if last_func_idx > 0:
                # NOTE: only make expensive copy if you're going to edit it
                v = df[attribute_name].copy()
            for idx, (frm, to) in enumerate(rs[:last_func_idx + 1]):
                if u.type_name(to) == 'function':
                    to = to(v)
                self.replacements[attribute_name].append((frm, to))
                if idx != last_func_idx:
                    v = v.replace({frm: to})
        return self

    def transform(self, df):
        """Replaces values according to metadata."""
        decisions = []
        for attribute_name, replacements in self.replacements.items():
            for from_value, to_value in replacements:
                df[attribute_name].replace({from_value: to_value},
                                           inplace=True)
            decisions.append({
                'action': f'replaced values in {attribute_name}',
                'reason': 'specified in metadata',
                'details': {
                    'replacements': replacements
                },
            })
        return df, decisions

    def fit_transform(self, df, metadata):
        """Runs fit and transform."""
        return self.fit(df, metadata).transform(df)


def extract_replacements(df, metadata):
    """Translate the Replacements to native python types and functions."""
    replacements = defaultdict(list)
    for attribute in metadata.attributes:
        if attribute.name in df.columns:
            for replacement in attribute.replacements:
                field_name, value = \
                    p.which(getattr(replacement, 'from'), 'value')
                if field_name == 'datum':
                    from_value = p.from_typed_datum(value)
                if field_name == 'missing':
                    from_value = np.nan
                field_name, value = p.which(replacement.to, 'value')
                if field_name == 'datum':
                    to_value = p.from_typed_datum(value)
                if field_name == 'function':
                    to_value = p.from_function_type(value)
                replacements[attribute.name].append((from_value, to_value))
    return replacements


def index_of_last_function(replacements):
    """Returns the index of the last function in a series of replacements.

    The list of replacements is for a single feature, and each item is a dict
    like {from_value: to_value}.
    """
    last_idx = 0
    idxs = np.where([
        u.type_name(to_value) == 'function' for _, to_value in replacements
    ])[0]
    if len(idxs) > 0:
        last_idx = idxs[-1]
    return last_idx
