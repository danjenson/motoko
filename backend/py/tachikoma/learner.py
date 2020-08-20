"""non sunt multiplicanda entia sine necessitate"""
import multiprocessing as mp

from sklearn.cluster import KMeans
from sklearn.dummy import DummyClassifier, DummyRegressor
from sklearn.ensemble import RandomForestClassifier
from sklearn.linear_model import SGDRegressor
from sklearn.metrics import silhouette_score
from sklearn.model_selection import KFold
from sklearn.svm import SVR
import numpy as np

from categorical_encoder import CategoricalEncoder
from continuous_scaler import ContinuousScaler
from enforcer import Enforcer
from enums_pb2 import BehaviorType, TaskType
from remover import Remover
from replacer import Replacer
from selector import Selector
from text_encoder import TextEncoder
from trimmer import Trimmer


def learn(df, metadata):
    """Learns the data and returns transformers, estimator, decisions, and
    evaluation.

    Args:
        df: pandas data frame
        metadata: protobuf metadata about data frame

    Output:
        transformers: used to transform the data for learning
        estimator: the model trained on the transformed data
        decisions: decisions made a long the way
        evaluation: metrics about model performance
    """
    task_type = TaskType.CLUSTER
    action = 'clustering data'
    reason = 'no target column was specified'
    if metadata.has_target:
        for attribute in metadata.attributes:
            if attribute.name == metadata.target_name:
                if attribute.behavior_type == BehaviorType.CATEGORICAL:
                    task_type = TaskType.CLASSIFY
                    action = 'classifying data'
                    reason = 'target column is categorical'
                elif attribute.behavior_type == BehaviorType.CONTINUOUS:
                    task_type = TaskType.REGRESS
                    action = 'regressing on data'
                    reason = 'target column is continuous'
    df, transformers, t_decisions = _transform(df, metadata, task_type)
    estimator, evaluation, e_decisions = _train(df, metadata, task_type)
    s_decisions = [{'action': action, 'reason': reason}]
    decisions = s_decisions + t_decisions + e_decisions
    return df, transformers, estimator, evaluation, decisions


def _transform(df, metadata, task_type):
    """Runs the transformers over the data."""
    transformers = [Enforcer(), Replacer(), Selector(), Remover()]
    if task_type == TaskType.CLASSIFY:
        transformers.extend([
            TextEncoder(),
            Trimmer(TaskType.CLASSIFY),
        ])
    elif task_type == TaskType.REGRESS:
        transformers.extend([
            CategoricalEncoder(),
            TextEncoder(),
            ContinuousScaler(TaskType.REGRESS),
            Trimmer(TaskType.REGRESS),
        ])
    else:
        # cluster
        transformers.extend([
            CategoricalEncoder(),
            TextEncoder(),
            ContinuousScaler(TaskType.CLUSTER),
        ])
    decisions = []
    for transformer in transformers:
        df, ds = transformer.fit_transform(df, metadata)
        decisions.extend(ds)
    return df, transformers, decisions


def _train(df, metadata, task_type):
    return {
        TaskType.CLASSIFY: _classify,
        TaskType.REGRESS: _regress,
        TaskType.CLUSTER: _cluster,
    }[task_type](df, metadata.target_name)


def _classify(df, target_name):
    """Trains and evaluates the classification algorithm."""
    # TODO(danj): reasonable defaults?
    kwargs = {
        # use all available processors
        'n_jobs': -1,
        'criterion': 'entropy',
        'n_estimators': 200,
        'max_depth': 7,
    }
    estimator = RandomForestClassifier(**kwargs)
    dummy_estimator = DummyClassifier()
    estimator, evaluation, decisions = _evaluate(
        df,
        target_name,
        estimator,
        dummy_estimator,
    )
    decisions = [{
        'action': 'trained random forest classifier',
        'reason': 'solid default classifier',
        'details': {
            'decision criterion': kwargs['criterion'],
            'number of trees': kwargs['n_estimators'],
            'max depth': kwargs['max_depth'],
        },
    }] + decisions
    return estimator, evaluation, decisions


def _evaluate(df, target_name, estimator, dummy_estimator):
    """Returns a trained estimator and its evaluation."""
    def get_X(idx):
        return df.loc[idx, df.columns != target_name]

    def get_y(idx):
        return df.loc[idx, df.columns == target_name].values.ravel()

    tasks = []
    k = 5
    kfold = KFold(n_splits=k, shuffle=True)
    for train_idx, valid_idx in kfold.split(df):
        tasks.append((
            estimator,
            dummy_estimator,
            get_X(train_idx),
            get_y(train_idx),
            get_X(valid_idx),
            get_y(valid_idx),
        ))

    with mp.Pool(mp.cpu_count()) as pool:
        accuracies = pool.starmap(_fit, tasks)

    average_accuracy = np.mean([a[0] for a in accuracies])
    dummy_average_accuracy = np.mean([a[1] for a in accuracies])
    all_idx = range(df.shape[0])
    estimator.fit(get_X(all_idx), get_y(all_idx))
    # TODO(danj): R^2 == accuracy is a slight abuse of syntax
    decisions = [{
        'action':
        f'evaluated model using {k}-fold cross-validation',
        'reason':
        'produces a reasonable estimate of model performance'
    }]
    evaluation = {
        # technically, this is R^2 for regressions
        'accuracy': average_accuracy,
        'improvement': average_accuracy - dummy_average_accuracy
    }
    return estimator, evaluation, decisions


def _fit(
        estimator,
        dummy_estimator,
        X_train,
        y_train,
        X_valid,
        y_valid,
):
    """Fit estimator and dummy estimator."""
    return (
        estimator.fit(X_train, y_train).score(X_valid, y_valid),
        dummy_estimator.fit(X_train, y_train).score(X_valid, y_valid),
    )


def _cluster(df, target_name=None):
    """Trains and evaluates the clustering algorithm."""
    # TODO(danj): reasonable numbers for k?
    test_k = [2, 3, 5, 8]
    # silhouette_score ranges from -1 to 1; -2 will always be beaten
    best_sscore = -2
    for k in test_k:
        estimator = KMeans(k)
        y = estimator.fit_predict(df.values)
        sscore = silhouette_score(df.values, y)
        if sscore > best_sscore:
            best_k = k
            best_sscore = sscore
            best_estimator = estimator
    evaluation = {'accuracy': (best_sscore + 1.0) / 2.0}
    decisions = [{
        'action': f'trained cluster using K-Means (k={best_k})',
        'reason': 'reasonable default clusterer',
    }, {
        'action': 'evaluated using normalized silhouette score',
        'reason': 'most common evaluation metric for clustering',
    }]
    return best_estimator, evaluation, decisions


def _regress(df, target_name):
    """Regresses on the data."""
    # see sklearn flow chart: https://tinyurl.com/y9mzk9tm
    estimator = SVR(gamma='scale')
    action = 'training support vector regression'
    reason = 'solid regressor for datasets with less than 100k records'
    details = {
        'number of rows': df.shape[0],
        'kernel': 'radial basis function',
    }
    if df.shape[0] > 1e5:
        estimator = SGDRegressor()
        action = 'training stochastic gradient descent regressor'
        reason = 'solid regressor for datasets with more than 100k records'
        details.pop('kernel')
    dummy_estimator = DummyRegressor()
    estimator, evaluation, decisions = _evaluate(
        df,
        target_name,
        estimator,
        dummy_estimator,
    )
    decisions = [{'action': action, 'reason': reason, 'details': details}]
    return estimator, evaluation, decisions


def predict(df, target_name, transformers, estimator):
    decisions = []
    for transformer in transformers:
        df, ds = transformer.transform(df)
        decisions.extend(ds)
    features = list(set(df.columns) - set([target_name]))
    return estimator.predict(df[features].values), decisions
