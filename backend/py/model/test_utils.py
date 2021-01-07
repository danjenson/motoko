import pandas as pd


def iris_df(with_news=False):
    """Returns the iris data frame."""
    path = 'tests/data/iris.csv'
    if with_news:
        path = 'tests/data/iris_news.csv'
    return pd.read_csv(path)
