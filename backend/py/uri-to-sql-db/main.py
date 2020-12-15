from os import environ as env
import json

from sqlalchemy import create_engine, text
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
    docker_db_url = 'postgres://postgres@172.17.0.1:5432/motoko'
    db = create_engine(env.get('DATABASE_URL', docker_db_url))
    data_db = create_engine(
        env.get('DATA_DATABASE_URL', docker_db_url + '_data'))

    def update_status(status):
        sql = text('UPDATE datasets SET status = :status WHERE uuid = :uuid')
        res = db.execute(sql, status=status, uuid=uuid)
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
        db.dispose()
        data_db.dispose()
    return {'statusCode': 200, 'body': res}
