import boto3
import time
import json


def lambda_handler(event, context):
    client = boto3.client('cloudfront')
    invalidation = client.create_invalidation(DistributionId='DISTRIBUTION_ID',
                                              InvalidationBatch={
                                                  'Paths': {
                                                      'Quantity': 1,
                                                      'Items': ["/*"]
                                                  },
                                                  'CallerReference':
                                                  str(time.time())
                                              })
    return json.dumps({'event': event, 'invalidation': invalidation})
