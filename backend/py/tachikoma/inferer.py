"""A data inference service."""
import datetime as dt
import pickle

import numpy as np
import pandas as pd

from types_pb2 import Attribute, Metadata
from enums_pb2 import BehaviorType, DataType
import utils as u
import protobuf as p


class Inferer:
    """Infers attribute characteristics and target attribute.

    Args:
        db: a db with `metadata_since` function

    Each attribute has a name, behavior type, data type, and replacements (for
    null values, etc). The target column is inferred from historical data and
    heuristics.
    """
    def __init__(self, db):
        self.db = db
        self.target_counts = u.nested_default_dict(1, int)
        self.behavior_counts = u.nested_default_dict(3, int)
        self.last_updated = dt.datetime(2019, 1, 1, 0, 0, 0)
        self._update()

    def _update(self):
        """Updates metadata used by `infer`"""
        update_time = dt.datetime.now()
        prev = update_time - dt.timedelta(minutes=5)
        # for metadata in self.db.metadata_since(self.last_updated):
        for metadata in self.db.metadata_since(prev):
            self._update_target_counts(metadata)
            self._update_behavior_counts(metadata)
        self.last_updated = update_time

    def _update_target_counts(self, metadata):
        """Updates the count indexed by feature name

        if the attribute is a feature, subtract 1
        if the attribute is a target, add 1
        ------------------------------------------
        if the attribute count > 0 => likely a target
        if the attribute count < 0 => likely a feature
        if the attribute count = 0 => unknown
        """
        target_name = metadata.target_name
        for attribute in metadata.attributes:
            self.target_counts[attribute.name] -= 1
            # the target attribute is included in the attributes
            # list, so +1 to correct for subtraction above and +1 for being
            # a target attribute
            if attribute.name == target_name:
                self.target_counts[attribute.name] += 2

    def _update_behavior_counts(self, metadata):
        """Updates behavior counts using metadata."""
        # assumes metadata only contains valid schemas
        for attribute in metadata.attributes:
            counts = self.behavior_counts[attribute.name]
            counts[attribute.data_type][attribute.behavior_type] += 1

    def infer(self, df, numeric_error_threshold, n_max_categories):
        """Infers attribute characteristics and target attribute.

        Attribute characteristics include data type, behavior type, and
        replacement values.
        """
        self._update()
        attributes = []
        for name in df.columns:
            data_type = _infer_data_type(
                df[name],
                numeric_error_threshold,
            )
            behavior_type = _infer_behavior_type(
                df[name],
                data_type,
                n_max_categories,
                self.behavior_counts,
            )
            replacements = _infer_replacements(
                df[name],
                behavior_type,
                data_type,
            )
            # use dict because replacements is list of dict too
            attributes.append(
                Attribute(
                    **{
                        'name': name,
                        'behavior_type': behavior_type,
                        'data_type': data_type,
                        'replacements': replacements,
                    }))

        target_name = _infer_target_from_historical_metadata(
            attributes,
            self.target_counts,
        )
        if not target_name:
            target_name, attributes = \
                _infer_target_using_heuristics(df, attributes)

        return Metadata(
            has_target=True,
            target_name=target_name,
            attributes=attributes,
        )


def _infer_data_type(ser, numeric_error_threshold):
    """Infers the data type of a series."""
    if _is_likely_numeric(ser, numeric_error_threshold):
        if _is_integer(ser):
            return DataType.INTEGER
        return DataType.FLOAT
    return p.python_to_protobuf_dtype(ser.dtype)


def _is_likely_numeric(ser, error_threshold):
    """Tests whether a series is numeric.

    If forcing the column to a numeric data type results in a smaller error
    rate than the `error_threshold`, it is considered numeric.
    """
    na_before = ser.isna().sum()
    tmp = pd.to_numeric(ser, errors='coerce')
    na_after = tmp.isna().sum()
    return (na_after - na_before) / ser.size <= error_threshold


def _is_integer(ser):
    """Tests whether the series is all integers."""
    tmp = pd.to_numeric(ser, errors='coerce')
    # will fail if NAs and not ignore
    tmp_int = tmp.astype(int, errors='ignore')
    vals_eq = tmp == tmp_int
    nans_eq = np.logical_and(np.isnan(tmp), np.isnan(tmp_int))
    return np.all(np.logical_or(vals_eq, nans_eq))


def _infer_behavior_type(
        ser,
        data_type,
        n_max_categories,
        behavior_counts,
):
    """Infers a behavior type using historical metadata and heuristics."""
    behavior_type = _infer_behavior_type_from_historical_metadata(
        ser.name,
        data_type,
        behavior_counts,
    )
    if behavior_type is None:
        # first time this feature <name, data_type> has been seen
        behavior_type = _infer_behavior_type_using_heuristics(
            ser,
            data_type,
            n_max_categories,
        )
    return behavior_type


def _infer_behavior_type_from_historical_metadata(
        name,
        data_type,
        behavior_counts,
):
    """Infers a behavior type using historical metadata."""
    max_count = 0
    behavior_type = None
    for b_type, count in behavior_counts[name][data_type].items():
        if count > max_count:
            max_count = count
            behavior_type = b_type
    return behavior_type


def _infer_behavior_type_using_heuristics(ser, data_type, n_max_categories):
    """Infers a behavior type using heuristics."""
    if data_type == DataType.STRING and ser.nunique() > n_max_categories:
        return BehaviorType.TEXT
    if data_type in [DataType.STRING, DataType.BOOLEAN]:
        return BehaviorType.CATEGORICAL
    # assume integers are continuous
    return BehaviorType.CONTINUOUS


def _infer_replacements(ser, behavior_type, data_type):
    """Infers replacement values for a given attribute."""
    # TODO(danj): do something intelligent, smee
    return []


def _infer_target_from_historical_metadata(attributes, target_counts):
    """Infers the target attribute using historical metadata.

    Creates tuples of (<count>, <attribute_name>) sorted in descending
    order by count (and then attribute_name); the attribute that has most
    often been a target in the historical metadata is selected; if none
    have been seen before, returns None.
    """
    desc_target_names = sorted(
        [
            (target_counts[a.name], a.name) for a in attributes
            # text targets not supported
            if a.behavior_type != BehaviorType.TEXT
        ],
        reverse=True,
    )
    if desc_target_names:
        count, target_name = desc_target_names[0]
        if count:
            return target_name
    # if desc_target_names is empty or count is 0, return None, since a
    # decision cannot be made based on historical data (if the count is 0,
    # either the attribute has never been seen or it has been a target and
    # feature equally often)
    return None


def _infer_target_using_heuristics(df, attributes):
    """Infers the target attribute using heuristics.

    If any attribute name contains one of the strings target, label,
    result, or outcome, return the first non-text data type attribute to
    match. If only a text attribute matches, make it the target and change
    it's behavior to categorical. If all target matches are text, select
    the one with the lowest cardinality and change it's behavior to
    categorical. If there are no matches, select the first non-text
    attribute. If all the attributes are text, select the one with the
    lowest cardinality and change it's behavior to categorical.
    """
    def best_guess(candidate_names):
        behavior_types = {a.name: a.behavior_type for a in attributes}
        # avoids text targets if possible; otherwise, selects the text
        # target with the lowest cardinality and sets it's corresponding
        # behavior type to categorical
        possible_targets = list(
            filter(
                lambda name: behavior_types[name] != BehaviorType.TEXT,
                candidate_names,
            ))
        # if there are any non-text, return the first arbitrarily
        if possible_targets:
            return possible_targets[0], attributes
        # if not, return the one with lowest cardinality
        asc_target_names = sorted([(df[t].nunique(), t)
                                   for t in possible_targets])
        _, target_name = asc_target_names[0]
        return target_name, update_target_behavior(target_name)

    # if the behavior type is text, update it to categorical because
    # text is an unsupported target behavior type
    def update_target_behavior(target_name):
        def f(attribute):
            if attribute.name == target_name:
                if attribute.behavior_type == BehaviorType.TEXT:
                    attribute.behavior_type = BehaviorType.CATEGORICAL
            return attribute

        return list(map(f, attributes))

    targets = ['target', 'label', 'result', 'outcome']
    eligible_targets = np.array([a.name for a in attributes])
    for target in targets:
        idxs = np.where([target in et for et in eligible_targets])[0]
        if idxs.size == 1:
            target_name = eligible_targets[idxs[0]]
            return target_name, update_target_behavior(target_name)
        if idxs.size > 1:
            return best_guess(eligible_targets[idxs])

    return best_guess(eligible_targets)
