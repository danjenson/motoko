import json

from sqlalchemy import create_engine, text
import pandas as pd

import utils as u


def lambda_handler(event, context):
    u.validate(event, ['uri', 'uuid'])
    uri, uuid = event['uri'], event['uuid']
    if 'drive.google' in uri:
        file_id = uri.split('/')[-2]
        uri = 'https://drive.google.com/uc?export=download&id=' + file_id
    data_db, meta_db = dbs()

    def update_status(status):
        q = text('UPDATE datasets SET status = :status WHERE uuid = :uuid')
        res = meta_db.execute(q, status=status, uuid=uuid)
        return status if res.rowcount == 1 else 'failed'

    res = 'failed'
    try:
        update_status('running')
        df = pd.read_csv(uri)
        table_name = 'dataset_' + str(uuid).replace('-', '_')
        df.to_sql(name=table_name, con=data_db, index=False)
        res = update_status('completed')
    except Exception as e:
        res = update_status('failed')
        raise e
    finally:
        data_db.dispose()
        meta_db.dispose()
    return {'statusCode': 200, 'body': res}


def dbs():
    data_db_url, meta_db_url = u.db_urls()
    data_db = create_engine(data_db_url)
    meta_db = create_engine(meta_db_url)
    return data_db, meta_db
