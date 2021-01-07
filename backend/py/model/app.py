import json
import pickle

from psycopg2 import sql
import boto3
import pandas as pd

from model import model
import utils as u


def lambda_handler(event, context):
    u.validate(event, ['view', 'uuid', 'features'])
    model_uuid = event['uuid']
    data_db, meta_db = u.dbs()
    data_cur, meta_cur = data_db.cursor(), meta_db.cursor()
    s3 = boto3.client('s3')

    def update_status(status):
        q = 'UPDATE models SET status = (%s) WHERE uuid = (%s)'
        meta_cur.execute(q, (status, str(model_uuid)))
        return status

    res = 'failed'
    try:
        update_status('running')
        target, features = event.get('target'), event['features']
        cols = [sql.Identifier(f) for f in features]
        if target:
            cols.append(sql.Identifier(target))
        view = sql.Identifier(event['view'])
        places = ', '.join(['{}'] * len(cols))
        q = sql.SQL(f'SELECT {places} FROM {{}}').format(*cols, view)
        df = pd.read_sql(q, data_db)
        _, m, evaluation, decisions = model(df, target)
        fname = f'{model_uuid}.pkl'
        tmp_pkl = f'/tmp/{fname}'
        with open(tmp_pkl, 'wb') as f:
            pickle.dump(m, f)
        if u.run_mode() != 'local':
            s3.upload_file(tmp_pkl, 'motoko-data', f'models/{fname}')
        q = '''
            UPDATE models
            SET status = 'completed', evaluation = (%s), decisions = (%s)
            WHERE uuid = (%s)
        '''
        args = (json.dumps(evaluation), json.dumps(decisions), str(model_uuid))
        meta_cur.execute(q, args)
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
