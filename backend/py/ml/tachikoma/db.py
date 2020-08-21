"""A simple wrapper for the essential db actions."""
import pickle

from bson import ObjectId
from pymongo import MongoClient

from types_pb2 import Metadata


class DB:
    """A simple wrapper for the essential db actions."""
    def __init__(self, database='motoko'):
        self.db = MongoClient()[database].learn

    def store(
            self,
            metadata,
            transformers,
            estimator,
            evaluation,
            decisions,
    ):
        """Stores all data associated with a learning session."""
        return str(
            self.db.insert_one({
                'metadata': metadata.SerializeToString(),
                'transformers': pickle.dumps(transformers),
                'estimator': pickle.dumps(estimator),
                'evaluation': evaluation,
                'decisions': decisions,
            }).inserted_id)

    def metadata_since(self, since_dt):
        """Returns metadata since `since_dt`."""
        results = self.db.find(
            {'_id': {
                '$gt': ObjectId.from_datetime(since_dt)
            }},
            {
                '_id': 0,
                'transformers': 0,
                'estimator': 0,
                'evaluation': 0,
                'decisions': 0,
            },
        )
        ms = []
        for result in results:
            m = Metadata()
            m.ParseFromString(result['metadata'])
            ms.append(m)
        return ms

    def get(self, learn_key, cols):
        fltr = {
            '_id': 0,
            'metadata': 0,
            'transformers': 0,
            'estimator': 0,
            'evaluation': 0,
            'decisions': 0,
        }
        for col in cols:
            del fltr[col]
        res = self.db.find_one({'_id': {'$eq': ObjectId(learn_key)}}, fltr)
        for k, v in res.items():
            if k in ['transformers', 'estimator']:
                res[k] = pickle.loads(v)
            elif k == 'metadata':
                m = Metadata()
                m.ParseFromString(v)
                res[k] = m
        # return single object if only 1 requested
        if len(cols) == 1:
            return res[cols]
        return res

    def clear(self):
        """Clears the learners database."""
        self.db.drop()
