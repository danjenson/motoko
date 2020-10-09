import pandas as pd


def lambda_handler(event, context):
    df = pd.read_csv(event['uri'], nrows=1000)
    return df.dtypes.to_json()
