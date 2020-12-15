#!/usr/bin/env python3
from collections import defaultdict
from enum import Enum
import pandas as pd
import numpy as np
import json


class DataType(Enum):
    BOOLEAN = 1
    FLOAT = 2
    INTEGER = 3
    STRING = 4


class BehaviorType(Enum):
    CONTINUOUS = 1
    CATEGORICAL = 2
    TEXT = 3


class EnumEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, Enum):
            return obj.name
        return json.JSONEncoder.default(self, obj)


def lambda_handler(event, context):
    if 'uri' not in event:
        return json.dumps({'error': 'no URI'})
    uri = event['uri']
    if 'drive.google' in uri:
        file_id = uri.split('/')[-2]
        uri = 'https://drive.google.com/uc?export=download&id=' + file_id
    df = pd.read_csv(uri, nrows=1000)
    numeric_error_threshold = event.get('numeric_error_threshold', 0.01)
    n_max_categories = event.get('n_max_categories', 20)
    d = {}
    # TODO(danj): connect this from db
    behavior_counts = nested_default_dict(3, int)
    for name in df.columns:
        data_type = _infer_data_type(
            df[name],
            numeric_error_threshold,
        )
        behavior_type = _infer_behavior_type(
            df[name],
            data_type,
            n_max_categories,
            behavior_counts,
        )
        d[name] = {'data_type': data_type, 'behavior_type': behavior_type}
    return json.dumps(d, cls=EnumEncoder)


def nested_default_dict(depth, base_type):
    '''Creates a nested default dictionary with ``n`` levels.'''
    def recurse(depth, base_type):
        if depth == 0:
            return base_type
        return lambda: defaultdict(recurse(depth - 1, base_type))

    return recurse(depth, base_type)()


def _infer_data_type(ser, numeric_error_threshold):
    '''Infers the data type of a series.'''
    if _is_likely_numeric(ser, numeric_error_threshold):
        if _is_integer(ser):
            return DataType.INTEGER
        return DataType.FLOAT
    return python_to_motoko_dtype(ser.dtype)


def _is_likely_numeric(ser, error_threshold):
    '''Tests whether a series is numeric.

    If forcing the column to a numeric data type results in a smaller error
    rate than the `error_threshold`, it is considered numeric.
    '''
    na_before = ser.isna().sum()
    tmp = pd.to_numeric(ser, errors='coerce')
    na_after = tmp.isna().sum()
    return (na_after - na_before) / ser.size <= error_threshold


def _is_integer(ser):
    '''Tests whether the series is all integers.'''
    tmp = pd.to_numeric(ser, errors='coerce')
    # will fail if NAs and not ignore
    tmp_int = tmp.astype(int, errors='ignore')
    vals_eq = tmp == tmp_int
    nans_eq = np.logical_and(np.isnan(tmp), np.isnan(tmp_int))
    return np.all(np.logical_or(vals_eq, nans_eq))


def python_to_motoko_dtype(dtype):
    '''Returns the motoko dtype for a given python dtype.'''
    # NOTE: this is for numpy data types
    if hasattr(dtype, 'name'):
        dtype = dtype.name
    return python_to_motoko_dtype_map()[dtype]


def python_to_motoko_dtype_map():
    '''Returns a type mapping python types to motoko enum types.'''
    return {
        # strings are dtype('O').name = 'object' in pandas
        'object': DataType.STRING,
        'int': DataType.INTEGER,
        'int64': DataType.INTEGER,
        'float': DataType.FLOAT,
        'float64': DataType.FLOAT,
        'bool': DataType.BOOLEAN,
    }


def _infer_behavior_type(
    ser,
    data_type,
    n_max_categories,
    behavior_counts,
):
    '''Infers a behavior type using historical metadata and heuristics.'''
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
    '''Infers a behavior type using historical metadata.'''
    max_count = 0
    behavior_type = None
    for b_type, count in behavior_counts[name][data_type].items():
        if count > max_count:
            max_count = count
            behavior_type = b_type
    return behavior_type


def _infer_behavior_type_using_heuristics(ser, data_type, n_max_categories):
    '''Infers a behavior type using heuristics.'''
    if data_type == DataType.STRING and ser.nunique() > n_max_categories:
        return BehaviorType.TEXT
    if data_type in [DataType.STRING, DataType.BOOLEAN]:
        return BehaviorType.CATEGORICAL
    # assume integers are continuous
    return BehaviorType.CONTINUOUS


if __name__ == '__main__':
    event = {
        'uri':
        'https://drive.google.com/file/d/12q0KWJAUaVba9RZrVY8QEXThK1x5GoF8/view?usp=sharing'
    }
    print(lambda_handler(event, {}))
