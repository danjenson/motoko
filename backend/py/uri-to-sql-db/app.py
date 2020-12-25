from os import environ as env
import json

from sqlalchemy import create_engine, text
import boto3
import pandas as pd


def lambda_handler(event, context):
    if 'uri' not in event:
        return json.dumps({'error': 'no URI'})
    if 'uuid' not in event:
        return json.dumps({'error': 'no uuid for table'})
    uri = event['uri']
    if 'drive.google' in uri:
        file_id = uri.split('/')[-2]
        uri = 'https://drive.google.com/uc?export=download&id=' + file_id
    uuid = event['uuid']
    data_db, meta_db = dbs()

    def update_status(status):
        sql = text('UPDATE datasets SET status = :status WHERE uuid = :uuid')
        res = meta_db.execute(sql, status=status, uuid=uuid)
        return status if res.rowcount == 1 else 'failed'

    res = 'failed'
    try:
        update_status('running')
        df = pd.read_csv(uri)
        table_name = 'dataset_' + uuid.replace('-', '_')
        df.to_sql(name=table_name, con=data_db, index=False)
        res = update_status('completed')
    except Exception:
        res = update_status('failed')
    finally:
        meta_db.dispose()
        data_db.dispose()
    return {'statusCode': 200, 'body': res}


def dbs():
    docker_db_url = 'postgres://postgres@172.17.0.1:5432/motoko'
    data_db_url = docker_db_url + '_data'
    meta_db_url = docker_db_url + '_meta'
    if env.get('RUN_MODE', 'local') != 'local':
        s = boto3.session.Session()
        sm = s.client(service_name='secretsmanager', region_name='us-west-1')
        secrets = json.loads(
            sm.get_secret_value(SecretId='motoko')['SecretString'])
        data_db_url = secrets['data_db_url']
        meta_db_url = secrets['meta_db_url']
    data_db = create_engine(data_db_url)
    meta_db = create_engine(meta_db_url)
    return data_db, meta_db
