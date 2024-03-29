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
