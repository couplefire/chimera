import lancedb
uri = "data/"
db = lancedb.connect(uri)

tbl = db.create_table("my_table",
                data=[{"vector": [3.1, 4.1], "item": "foo", "price": 10.0},
                      {"vector": [5.9, 26.5], "item": "bar", "price": 20.0}])