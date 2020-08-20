"""Generic utils."""
from collections import defaultdict
import datetime as dt
import io

from sklearn import svm
from sklearn.utils.random import sample_without_replacement
import pandas as pd

from types_pb2 import Metadata
import protobuf as p


def chunks(l, n=int(2e6)):
    """Yield successive n-sized chunks from l."""
    for i in range(0, len(l), n):
        yield l[i:i + n]


def class_name(obj):
    """Returns the class name of the given object as a string."""
    return obj.__class__.__name__


def csv_bytes_to_df(csv_bytes):
    """Load bytestring representing a csv to a pandas data frame."""
    # TODO(danj): collect bad lines
    return pd.read_csv(
        io.BytesIO(csv_bytes),
        error_bad_lines=False,
        # it prints poorly to console
        warn_bad_lines=False,
    )


def datetime_prefix():
    """Returns an isoformatted date as a prefix string."""
    return dt.datetime.now().isoformat() + ': '


def df_to_csv_bytes(df):
    """Converts a python object to a csv encoded byte array."""
    if not isinstance(df, pd.DataFrame):
        msg = 'only implemented for pandas DataFrames'
        raise NotImplementedError(msg)
    buf = io.StringIO()
    df.to_csv(buf, index=False)
    return buf.getvalue().encode('utf-8')


def iris_df(with_news=False):
    """Returns the iris data frame."""
    path = 'test/data/iris.csv'
    if with_news:
        path = 'test/data/iris_news.csv'
    return pd.read_csv(path)


def iris_estimator(with_news=False):
    """Returns a model trained on the iris data frame."""
    df = iris_df(with_news)
    features = list(set(df.columns) - set(['species']))
    estimator = svm.SVC(gamma='scale')
    estimator.fit(df[features].values, df.species.values.ravel())
    return estimator


def iris_metadata(with_news=False):
    """Returns the iris metadata."""
    path = 'test/data/iris_metadata.json'
    if with_news:
        path = 'test/data/iris_news_metadata.json'
    return p.load_protobuf_from_json(path, Metadata)


def nested_default_dict(depth, base_type):
    """Creates a nested default dictionary with ``n`` levels."""
    def recurse(depth, base_type):
        if depth == 0:
            return base_type
        return lambda: defaultdict(recurse(depth - 1, base_type))

    return recurse(depth, base_type)()


def sample_X_y(df, target_name, n_samples):
    """Sample X and y from df."""
    if n_samples > df.shape[0]:
        n_samples = df.shape[0]
    idx = sample_without_replacement(df.shape[0], n_samples)
    return (
        df.loc[idx, df.columns != target_name],
        df.loc[idx, [target_name]].values.ravel(),
    )


def type_name(obj):
    """Returns the str type name of a given object."""
    return type(obj).__name__
