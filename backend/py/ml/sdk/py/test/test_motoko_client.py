"Tests motoko client"
import unittest

from motoko_client import MotokoClient
import utils as u


class TestMotokoClient(unittest.TestCase):
    "Tests motoko client"

    def test_motoko_client(self):
        email = 'motoko.kusanagi@sector9.jp'
        with open('test/keys/test_api_key.txt') as f:
            api_key = f.read().strip()
        motoko = MotokoClient(email, api_key)
        df = u.iris_df()
        metadata = motoko.infer(df)
        metadata.target_name = 'species'
        learn_key, evaluation, decisions = motoko.learn(df, metadata)
        predictions, decisions = motoko.predict(df, learn_key)
        self.assertGreater(evaluation["accuracy"], 0.9)
