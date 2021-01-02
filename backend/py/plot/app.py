from os import environ as env
from uuid import UUID
import json

from plotnine import (
    aes,
    ggplot,
    ggtitle,
    geom_bar,
    geom_histogram,
    geom_line,
    geom_point,
    geom_smooth,
    xlab,
    ylab,
)
from sqlalchemy import create_engine, text
import boto3
import pandas as pd


class InvalidArguments(Exception):
    pass


def lambda_handler(event, context):
    validate(event)
    plot_uuid = event['uuid']
    data_db, meta_db = dbs()
    s3 = boto3.client('s3')

    def update_status(status):
        sql = text('UPDATE plots SET status = :status WHERE uuid = :uuid')
        res = meta_db.execute(sql, status=status, uuid=plot_uuid)
        return status if res.rowcount == 1 else 'failed'

    res = 'failed'
    try:
        update_status('running')
        args = json.loads(event['args'])
        cols = ', '.join(columns(**args))
        df = pd.read_sql(f"SELECT {cols} FROM {event['view']}", data_db)
        p = plot(df, event['type'], args)
        fname = f'{plot_uuid}.svg'
        tmp_svg = f'/tmp/{fname}'
        p.save(tmp_svg)
        if run_mode() != 'local':
            s3.upload_file(tmp_svg, 'motoko-data', f'plots/{fname}')
        res = update_status('completed')
    except Exception as e:
        res = update_status('failed')
        raise e
    finally:
        meta_db.dispose()
        data_db.dispose()
    return {'statusCode': 200, 'body': res}


def dbs():
    docker_db_url = 'postgres://postgres@172.17.0.1:5432/motoko'
    data_db_url = docker_db_url + '_data'
    meta_db_url = docker_db_url + '_meta'
    if run_mode() != 'local':
        s = boto3.session.Session()
        sm = s.client(service_name='secretsmanager', region_name='us-west-1')
        secrets = json.loads(
            sm.get_secret_value(SecretId='motoko')['SecretString'])
        data_db_url = secrets['data_db_url']
        meta_db_url = secrets['meta_db_url']
    data_db = create_engine(data_db_url)
    meta_db = create_engine(meta_db_url)
    return data_db, meta_db


def run_mode():
    return env.get('RUN_MODE', 'local')


def validate(event):
    required = ['view', 'uuid', 'type', 'args']
    for req in required:
        if req not in event:
            raise InvalidArguments(f'no {req}')
    event['uuid'] = UUID(event['uuid'])


def columns(
    x=None,
    y=None,
    color=None,
    shape=None,
    facet_x=None,
    facet_y=None,
    **kwargs,
):
    return {v for k, v in locals().items() if k != 'kwargs' and v}


def plot(df, _type, kwargs):
    return {
        func_name.split('_', 1)[1].upper(): globals()[func_name]
        for func_name in globals().keys() if func_name.startswith('plot_')
    }[_type](df, **kwargs)


def plot_bar(df, x, color=None, **kwargs):
    a = aesthetics(locals())
    if a.get('color'):
        a['fill'] = a.pop('color')
    p = ggplot(df, aes(**a)) + geom_bar()
    return add_shared(p, **kwargs)


def aesthetics(d):
    return {
        k: v
        for k, v in d.items() if v is not None and k in [
            'x',
            'y',
            'color',
            'shape',
        ]
    }


def add_shared(p, title=None, **kwargs):
    if title:
        p = p + ggtitle(title)
    return p


def plot_histogram(df, x, **kwargs):
    p = ggplot(df, aes(**aesthetics(locals()))) + geom_histogram(bins=30)
    return add_shared(p, **kwargs)


def plot_line(df, x, y, color=None, **kwargs):
    p = ggplot(df, aes(**aesthetics(locals()))) + geom_line()
    return add_shared(p, **kwargs)


def plot_scatter(
    df,
    x,
    y,
    color=None,
    shape=None,
    **kwargs,
):
    p = ggplot(df, aes(**aesthetics(locals()))) + geom_point()
    return add_shared(p, **kwargs)


def plot_smooth(
    df,
    x,
    y,
    color=None,
    shape=None,
    se=True,
    **kwargs,
):
    p = ggplot(df, aes(**aesthetics(locals()))) + geom_point() + geom_smooth(
        se=se, method='glm')
    return add_shared(p, **kwargs)
