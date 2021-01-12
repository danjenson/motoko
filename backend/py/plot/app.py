import json

import boto3
import pandas as pd
from plotnine import (
    aes,
    element_text,
    ggplot,
    ggtitle,
    geom_bar,
    geom_histogram,
    geom_line,
    geom_point,
    geom_smooth,
    theme,
    xlab,
    ylab,
)
from psycopg2 import sql

import utils as u


def lambda_handler(event, context):
    u.validate(event, ['view', 'uuid', 'type', 'args'])
    plot_uuid = event['uuid']
    data_db, meta_db = u.dbs()
    data_cur, meta_cur = data_db.cursor(), meta_db.cursor()
    s3 = boto3.client('s3')

    def update_status(status):
        q = 'UPDATE plots SET status = (%s) WHERE uuid = (%s)'
        meta_cur.execute(q, (status, str(plot_uuid)))
        return status

    res = 'failed'
    try:
        update_status('running')
        args = json.loads(event['args'])
        cols = [sql.Identifier(c) for c in columns(**args)]
        places = ', '.join(['{}'] * len(cols))
        view = sql.Identifier(event['view'])
        q = sql.SQL(f'SELECT {places} FROM {{}}').format(*cols, view)
        df = pd.read_sql(q, data_db)
        p = plot(df, event['type'], args)
        fname = f'{plot_uuid}.svg'
        tmp_svg = f'/tmp/{fname}'
        p.save(tmp_svg)
        if u.run_mode() != 'local':
            s3.upload_file(tmp_svg, 'motoko-data', f'plots/{fname}')
        res = update_status('completed')
    except Exception as e:
        q = '''
            UPDATE plots
            SET status = 'failed', error = :error
            WHERE uuid = :uuid
        '''
        error = json.dumps({'message': e.args[0]})
        meta_cur.execute(q, (error, str(plot_uuid)))
        raise e
    finally:
        meta_db.commit()
        data_db.commit()
        meta_cur.close()
        data_cur.close()
        meta_db.close()
        data_db.close()
    return {'statusCode': 200, 'body': res}


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
    p = ggplot(df, aes(**a)) + geom_bar() + theme(
        axis_text_x=element_text(rotation=90, hjust=1))
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
