"""Protobuf helpers."""
import json

from google.protobuf.json_format import ParseDict
import numpy as np
import scipy as sp

from enums_pb2 import DataType, FunctionType


def from_function_type(function_type):
    """Converts from a function type to a native python function."""
    return {
        FunctionType.MEAN:
        np.nanmean,
        FunctionType.MEDIAN:
        np.nanmedian,
        # NOTE: if multiple modes, take first
        FunctionType.MODE:
        lambda v: sp.stats.mode(v).mode[0],
    }[function_type]


def from_typed_datum(typed_datum):
    """Converts from a typed datum to a native python type."""
    return getattr(typed_datum, DataType.Name(typed_datum.type))


def load_protobuf_from_json(json_file, protobuf):
    """Loads a protobuf struct from a json file."""
    with open(json_file) as f:
        return ParseDict(json.load(f), protobuf())


def protobuf_to_python_dtype(dtype):
    """Converts protobuf data types to python data types."""
    return {v: k for k, v in python_to_protobuf_dtype_map().items()}[dtype]


def python_to_protobuf_dtype(dtype):
    """Returns the protobuf dtype for a given python dtype."""
    # NOTE: this is for numpy data types
    if hasattr(dtype, 'name'):
        dtype = dtype.name
    return python_to_protobuf_dtype_map()[dtype]


def python_to_protobuf_dtype_map():
    """Returns a type mapping python types to protobuf enum types."""
    return {
        # strings are dtype('O').name = 'object' in pandas
        'object': DataType.STRING,
        'int': DataType.INTEGER,
        'int64': DataType.INTEGER,
        'float': DataType.FLOAT,
        'float64': DataType.FLOAT,
        'bool': DataType.BOOLEAN,
    }


def which(obj, field='type'):
    """Extracts a field name and the value from a protobuf oneof field."""
    field_name = obj.WhichOneof(field)
    return field_name, getattr(obj, field_name)
