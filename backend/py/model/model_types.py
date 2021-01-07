from enum import Enum, auto
import re


class TaskType(Enum):
    CLUSTER = auto()
    CLASSIFY = auto()
    REGRESS = auto()


class BehaviorType(Enum):
    CONTINUOUS = auto()
    CATEGORICAL = auto()
    TEXT = auto()


class DataType(Enum):
    BOOLEAN = auto()
    FLOAT = auto()
    INTEGER = auto()
    STRING = auto()


def behavior_type(col):
    return {
        DataType.BOOLEAN: BehaviorType.CATEGORICAL,
        DataType.FLOAT: BehaviorType.CONTINUOUS,
        DataType.INTEGER: BehaviorType.CONTINUOUS,
        DataType.STRING: BehaviorType.CATEGORICAL,
    }[data_type(col)]


def data_type(col):
    dt = {
        # strings are dtype('O').name = 'object' in pandas
        'object': DataType.STRING,
        'int': DataType.INTEGER,
        'int64': DataType.INTEGER,
        'float': DataType.FLOAT,
        'float64': DataType.FLOAT,
        'bool': DataType.BOOLEAN,
    }[col.dtype.name]
    if dt in [DataType.INTEGER, DataType.FLOAT]:
        bool_regex = re.compile('^is_.*')
        if bool_regex.match(col.name) and col.nunique() == 2:
            dt = DataType.BOOLEAN
    return dt
