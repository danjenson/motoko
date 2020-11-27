#!/usr/bin/env python3
from os import environ as env
import json

from sqlalchemy import create_engine, text
from sqlalchemy_utils import create_database, database_exists
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
    default_url = 'postgres://postgres@localhost/motoko'
    default_data_url = default_url + '_data'
    db = create_engine(env.get('DATABASE_URL', default_url))
    data_db = create_engine(env.get('DATA_DATABASE_URL', default_data_url))

    def update_status(status):
        sql = text('UPDATE datasets SET status = :status WHERE uuid = :uuid')
        res = db.execute(sql, status=status, uuid=uuid)
        return status if res.rowcount == 1 else 'failed'

    update_status('running')
    if not database_exists(data_db.url):
        create_database(data_db.url)
    df = pd.read_csv(uri)
    table_name = 'dataset_' + uuid.replace('-', '_')
    df.to_sql(name=table_name, con=data_db, index=False)
    return {'statusCode': 200, 'body': update_status('completed')}


if __name__ == '__main__':
    import uuid
    event = {
        'uri':
        'https://drive.google.com/file/d/12q0KWJAUaVba9RZrVY8QEXThK1x5GoF8/view?usp=sharing',
        'uuid': str(uuid.uuid4()),
    }
    print(lambda_handler(event, {}))
