from os import environ as env
from uuid import UUID
import json

import boto3
import psycopg2


def dbs():
    data_db_url, meta_db_url = db_urls()
    data_db = psycopg2.connect(data_db_url)
    meta_db = psycopg2.connect(meta_db_url)
    return data_db, meta_db


def db_urls():
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
    return data_db_url, meta_db_url


def run_mode():
    return env.get('RUN_MODE', 'local')


def validate(event, required):
    for req in required:
        if req not in event:
            raise InvalidArguments(f'no {req}')
    if 'uuid' in required:
        event['uuid'] = UUID(event['uuid'])


class InvalidArguments(Exception):
    pass
