import boto3
import time
import json


def lambda_handler(event, context):
    client = boto3.client('cloudfront')
    invalidation = client.create_invalidation(
        DistributionId=event['distribution_id'],
        InvalidationBatch={
            'Paths': {
                'Quantity': 1,
                'Items': ["/*"]
            },
            'CallerReference': str(time.time())
        })
    return {'status': invalidation['Invalidation']['Status']}
