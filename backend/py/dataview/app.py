import json

from psycopg2 import sql

import utils as u


def lambda_handler(event, context):
    u.validate(event, ['parent_view', 'view', 'uuid', 'operation', 'args'])
    dataview_uuid = event['uuid']
    data_db, meta_db = u.dbs()
    data_cur, meta_cur = data_db.cursor(), meta_db.cursor()

    def update_status(status):
        q = 'UPDATE dataviews SET status = (%s) WHERE uuid = (%s)'
        meta_cur.execute(q, (status, str(dataview_uuid)))
        return status

    res = 'failed'
    try:
        update_status('running')
        args = json.loads(event['args'])
        dataview(
            event['operation'],
            data_cur,
            event['parent_view'],
            event['view'],
            args,
        )
        update_status('completed')
    except Exception as e:
        q = '''
            UPDATE dataviews
            SET status = 'failed', error = (%s)
            WHERE uuid = (%s)
        '''
        error = json.dumps({'message': e.args[0]})
        meta_cur.execute(q, (error, str(dataview_uuid)))
        raise e
    finally:
        meta_db.commit()
        data_db.commit()
        meta_cur.close()
        data_cur.close()
        meta_db.close()
        data_db.close()
    return {'statusCode': 200, 'body': res}


def dataview(operation, db, parent_view, view, args):
    return {
        func_name.split('_', 1)[1].upper(): globals()[func_name]
        for func_name in globals().keys() if func_name.startswith('dataview_')
    }[operation](db, parent_view, view, **args)


def dataview_filter(db, parent_view, view, filters, **kwargs):
    clauses = []
    for fltr in filters:
        comparator = fltr['comparator']
        if comparator not in ['=', '>', '>=', '<', '<=']:
            raise InvalidComparator(f'invalid comparator: {comparator}')
        clauses.append(
            sql.SQL(f'{{}} {comparator} {{}}').format(
                sql.Identifier(fltr['column']), sql.Literal(fltr['value'])))
    places = '\nAND'.join(['{}'] * len(clauses))
    q = sql.SQL(f'''
        CREATE VIEW {{}} AS
        SELECT *
        FROM {{}}
        WHERE {places}
    ''').format(sql.Identifier(view), sql.Identifier(parent_view), *clauses)
    db.execute(q)


def dataview_mutate(db, parent_view, view, mutations, **kwargs):
    raise NotImplementedError('coming soon!')


def dataview_select(db, parent_view, view, columns, **kwargs):
    places = ', '.join(['{}'] * len(columns))
    q = sql.SQL(f'''
        CREATE VIEW {{}} AS
        SELECT {places}
        FROM {{}}
    ''').format(
        sql.Identifier(view),
        *[sql.Identifier(col) for col in columns],
        sql.Identifier(parent_view),
    )
    db.execute(q)


def dataview_sort(db, parent_view, view, sorts, **kwargs):
    clauses = []
    for sort in sorts:
        order = sort['order']
        if sort['order'] not in ['ASCENDING', 'DESCENDING']:
            raise InvalidOrder(f'invalid order: {order}')
        order = {'ASCENDING': 'ASC', 'DESCENDING': 'DESC'}[order]
        clauses.append(
            sql.SQL(f'{{}} {order}').format(sql.Identifier(sort['column'])))
    places = ', '.join(['{}'] * len(clauses))
    q = sql.SQL(f'''
        CREATE VIEW {{}} AS
        SELECT *
        FROM {{}}
        ORDER BY {places}
    ''').format(sql.Identifier(view), sql.Identifier(parent_view), *clauses)
    db.execute(q)


def dataview_summarize(
    db,
    parent_view,
    view,
    summaries,
    groupBys=None,
    **kwargs,
):
    clauses = []
    for summary in summaries:
        summarizer = summary['summarizer']
        if summarizer not in [
                'COUNT',
                'MEAN',
                'MEDIAN',
                'MODE',
                'MIN',
                'MAX',
                'SUM',
                'STDDEV',
        ]:
            raise InvalidSummarizer(f'invalid summarizer: {summarizer}')
        col = summary['column']
        summarizer = summary['summarizer'].lower()
        csql = f'{summarizer}({{}}::float8) AS "{col}_{summarizer}"'
        if summarizer == 'mean':
            csql = f'AVG({{}}::float8) AS "{col}_mean"'
        elif summarizer == 'median':
            csql = 'percentile_cont(0.5) WITHIN GROUP' + \
                f' (ORDER BY {{}}::float8) AS "{col}_median"'
        elif summarizer == 'mode':
            csql = 'mode() WITHIN GROUP' + \
                f' (ORDER BY {{}}::float8) AS "{col}_mode"'
        clauses.append(sql.SQL(csql).format(sql.Identifier(col)))
    summary_places = ', '.join(['{}'] * len(clauses))
    q = sql.SQL(f'''
        CREATE VIEW {{}} AS
        SELECT {summary_places}
        FROM {{}}
    ''').format(sql.Identifier(view), *clauses, sql.Identifier(parent_view))
    if groupBys:
        group_by_places = ', '.join(['{}'] * len(groupBys))
        group_bys = [sql.Identifier(gby) for gby in groupBys]
        q = sql.SQL(f'''
            CREATE VIEW {{}} AS
            SELECT {group_by_places}, {summary_places}
            FROM {{}}
            GROUP BY {group_by_places}
        ''').format(
            sql.Identifier(view),
            *group_bys,
            *clauses,
            sql.Identifier(parent_view),
            *group_bys,
        )
    db.execute(q)


class InvalidComparator(Exception):
    pass


class InvalidOrder(Exception):
    pass


class InvalidSummarizer(Exception):
    pass
