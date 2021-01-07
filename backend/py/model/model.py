from collections import namedtuple
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
from model_types import data_type, BehaviorType, DataType, TaskType
from remover import Remover
from text_encoder import TextEncoder
from trimmer import Trimmer

Model = namedtuple('Model', ['transformers', 'estimator'])


def model(df, target):
    task_type = TaskType.CLUSTER
    action = 'clustering data'
    reason = 'no target column specified'
    if target:
        task_type = TaskType.REGRESS
        action = 'regressing on data'
        reason = 'target column is continuous'
        if data_type(df[target]) in [DataType.STRING, DataType.BOOLEAN]:
            task_type = TaskType.CLASSIFY
            action = 'classifying data'
            reason = 'target column is categorical'
    df, transformers, transform_decisions = transform(task_type, df, target)
    estimator, evaluation, train_decisions = train(task_type, df, target)
    decisions = [{
        'action': action,
        'reason': reason
    }] + transform_decisions + train_decisions
    return df, Model(transformers, estimator), evaluation, decisions


def transform(task_type, df, target):
    transformers = [Remover()]
    if task_type is TaskType.CLUSTER:
        transformers.extend([
            CategoricalEncoder(),
            TextEncoder(),
            ContinuousScaler(TaskType.CLUSTER),
        ])
    elif task_type is TaskType.CLASSIFY:
        transformers.extend([
            TextEncoder(),
            Trimmer(TaskType.CLASSIFY),
        ])
    elif task_type is TaskType.REGRESS:
        transformers.extend([
            CategoricalEncoder(),
            TextEncoder(),
            ContinuousScaler(TaskType.REGRESS),
            Trimmer(TaskType.REGRESS),
        ])
    else:
        raise NotImplementedError('task type not supported!')
    decisions = []
    for transformer in transformers:
        df, ds = transformer.fit_transform(df, target)
        decisions.extend(ds)
    return df, transformers, decisions


def train(task_type, df, target):
    return {
        TaskType.CLUSTER: cluster,
        TaskType.CLASSIFY: classify,
        TaskType.REGRESS: regress,
    }[task_type](df, target)


def cluster(df, target_name=None):
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
    evaluation = {'normalized silhouette score': (best_sscore + 1.0) / 2.0}
    decisions = [{
        'action': f'trained cluster using K-Means (k={best_k})',
        'reason': 'reasonable default clusterer',
    }, {
        'action': 'evaluated using normalized silhouette score',
        'reason': 'most common evaluation metric for clustering',
    }]
    return best_estimator, evaluation, decisions


def classify(df, target):
    # TODO(danj): reasonable defaults?
    kwargs = {
        # use all available processors
        'n_jobs': -1,
        'criterion': 'entropy',
        'n_estimators': 200,
        'max_depth': 7,
    }
    estimator = RandomForestClassifier(**kwargs)
    dummy_estimator = DummyClassifier(strategy='prior')
    estimator, evaluation, decisions = evaluate(
        TaskType.CLASSIFY,
        df,
        target,
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


def evaluate(task_type, df, target, estimator, dummy_estimator):
    def get_X(idx):
        return df.loc[idx, df.columns != target]

    def get_y(idx):
        return df.loc[idx, df.columns == target].values.ravel()

    # can't use multiprocessing.{Pool,Queue} on lambda since it requires /dev/shm
    # https://aws.amazon.com/blogs/compute/parallel-processing-in-python-with-aws-lambda/
    tasks = []
    scores = mp.Manager().dict()
    k = 5
    kfold = KFold(n_splits=k, shuffle=True)
    for task_id, (train_idx, valid_idx) in enumerate(kfold.split(df)):
        task = mp.Process(target=fit,
                          args=(
                              task_id,
                              scores,
                              estimator,
                              dummy_estimator,
                              get_X(train_idx),
                              get_y(train_idx),
                              get_X(valid_idx),
                              get_y(valid_idx),
                          ))
        task.start()
        tasks.append(task)

    for task in tasks:
        task.join()

    average_score = np.mean([s[0] for s in scores.values()])
    dummy_average_score = np.mean([s[1] for s in scores.values()])
    all_idx = range(df.shape[0])
    estimator.fit(get_X(all_idx), get_y(all_idx))
    decisions = [{
        'action':
        f'evaluated model using {k}-fold cross-validation',
        'reason':
        'produces a reasonable estimate of model performance'
    }]
    key = {TaskType.REGRESS: 'R^2', TaskType.CLASSIFY: 'accuracy'}[task_type]
    evaluation = {
        key: average_score,
        'improvement': average_score - dummy_average_score
    }
    return estimator, evaluation, decisions


def fit(
    task_id,
    scores,
    estimator,
    dummy_estimator,
    X_train,
    y_train,
    X_valid,
    y_valid,
):
    scores[task_id] = (
        estimator.fit(X_train, y_train).score(X_valid, y_valid),
        dummy_estimator.fit(X_train, y_train).score(X_valid, y_valid),
    )


def regress(df, target):
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
    estimator, evaluation, decisions = evaluate(
        TaskType.REGRESS,
        df,
        target,
        estimator,
        dummy_estimator,
    )
    decisions = [{'action': action, 'reason': reason, 'details': details}]
    return estimator, evaluation, decisions
