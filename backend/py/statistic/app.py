import json

from psycopg2 import sql

import utils as u


def lambda_handler(event, context):
    u.validate(event, ['view', 'uuid', 'type', 'args'])
    statistic_uuid = event['uuid']
    data_db, meta_db = u.dbs()
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
        q = '''
            UPDATE statistics
            SET status = 'failed', error = (%s)
            WHERE uuid = (%s)
        '''
        error = json.dumps({'message': e.args[0]})
        meta_cur.execute(q, (error, str(statistic_uuid)))
        raise e
    finally:
        meta_db.commit()
        data_db.commit()
        meta_cur.close()
        data_cur.close()
        meta_db.close()
        data_db.close()
    return {'statusCode': 200, 'body': res}


def statistic(_type, db, view, args):
    return {
        func_name.split('_', 1)[1].upper(): globals()[func_name]
        for func_name in globals().keys() if func_name.startswith('statistic_')
    }[_type](db, view, **args)


def statistic_correlation(db, view, x, y, **kwargs):
    q = sql.SQL('SELECT corr({x}::float8, {y}::float8) FROM {view}').format(
        x=sql.Identifier(x), y=sql.Identifier(y), view=sql.Identifier(view))
    db.execute(q)
    return {'correlation': float(db.fetchone()[0])}


def statistic_summary(db, view, x, **kwargs):
    q = sql.SQL('''
    SELECT
        avg({x}::float8) AS mean,
        percentile_cont(0.5) WITHIN GROUP (ORDER BY {x}::float8) AS median,
        mode() WITHIN GROUP (ORDER BY {x}::float8) AS mode,
        min({x}::float8),
        max({x}::float8),
        stddev({x}::float8)
    FROM {view}
    ''').format(x=sql.Identifier(x), view=sql.Identifier(view))
    db.execute(q)
    res = [float(x) for x in db.fetchone()]
    return {
        'mean': res[0],
        'median': res[1],
        'mode': res[2],
        'min': res[3],
        'max': res[4],
        'stddev': res[5],
    }
