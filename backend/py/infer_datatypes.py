# import pandas as pd


def lambda_handler(event, context):
    # _df = pd.read_csv(event['dataset_uri'], nrows=1000)
    return {'a': 'continuous', 'b': 'categorical'}
