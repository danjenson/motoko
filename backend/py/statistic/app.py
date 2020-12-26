from os import environ as env
from uuid import UUID
import json

from psycopg2 import sql
import boto3
import psycopg2


class InvalidArguments(Exception):
    pass


def lambda_handler(event, context):
    validate(event)
    statistic_uuid = event['uuid']
    data_db, meta_db = dbs()
    data_cur, meta_cur = data_db.cursor(), meta_db.cursor()

    def update_status(status):
        q = 'UPDATE statistics SET status = (%s) WHERE uuid = (%s)'
        meta_cur.execute(q, (status, str(statistic_uuid)))
        return status

    res = 'failed'
    try:
        update_status('running')
        args = json.loads(event['args'])
        value = statistic(event['type'], data_cur, event['view'], args)
        q = '''
            UPDATE statistics
            SET status = 'completed', value = (%s)
            WHERE uuid = (%s)
        '''
        meta_cur.execute(q, (json.dumps(value), str(statistic_uuid)))
        res = 'completed'
    except Exception as e:
        res = update_status('failed')
        raise e
    finally:
        meta_db.commit()
        data_db.commit()
        meta_cur.close()
        data_cur.close()
        meta_db.close()
        data_db.close()
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
    data_db = psycopg2.connect(data_db_url)
    meta_db = psycopg2.connect(meta_db_url)
    return data_db, meta_db


def run_mode():
    return env.get('RUN_MODE', 'local')


def validate(event):
    required = ['view', 'uuid', 'type', 'args']
    for req in required:
        if req not in event:
            raise InvalidArguments(f'no {req}')
    event['uuid'] = UUID(event['uuid'])


def statistic(_type, db, view, args):
    return {
        func_name.split('_', 1)[1].upper(): globals()[func_name]
        for func_name in globals().keys() if func_name.startswith('statistic_')
    }[_type](db, view, **args)


def statistic_correlation(db, view, x, y, **kwargs):
    q = sql.SQL('SELECT corr({x}, {y}) FROM {view}').format(
        x=sql.Identifier(x), y=sql.Identifier(y), view=sql.Identifier(view))
    db.execute(q)
    return {'correlation': db.fetchone()[0]}


def statistic_summary(db, view, x, **kwargs):
    q = sql.SQL('''
    SELECT
        avg({x}) AS mean,
        percentile_cont(0.5) WITHIN GROUP (ORDER BY {x}) AS median,
        mode() WITHIN GROUP (ORDER BY {x}) AS mode,
        min({x}),
        max({x}),
        stddev({x})
    FROM {view}
    ''').format(x=sql.Identifier(x), view=sql.Identifier(view))
    db.execute(q)
    res = db.fetchone()
    return {
        'mean': res[0],
        'median': res[1],
        'mode': res[2],
        'min': res[3],
        'max': res[4],
        'stddev': res[5],
    }
