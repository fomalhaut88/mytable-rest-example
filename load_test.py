"""
Run:
    cargo build --release && rm person.tbl && ROCKET_ENV=prod ./target/release/mytable-rest-example
"""

import json
import threading
from time import time
from contextlib import contextmanager

import requests


THREADS = 50
COUNT = 1000
assert COUNT % THREADS == 0


@contextmanager
def measure_time(label):
    t1 = time()
    yield
    t2 = time()
    print("{}: {:.3f} sec".format(label, t2 - t1))


def perform_in_threads(threads):
    def decorator(func):
        def wrapper(*args, **kwargs):
            th_list = [
                threading.Thread(target=func, args=(i,) + args, kwargs=kwargs)
                for i in range(threads)
            ]
            list(map(threading.Thread.start, th_list))
            list(map(threading.Thread.join, th_list))
        return wrapper
    return decorator


@perform_in_threads(THREADS)
def insert_many(i):
    url = "http://localhost:8000/insert"
    data = {
        'name': 'alex',
        'age': 32,
    }
    headers = {
        'Content-Type': 'application/json',
    }
    for _ in range(COUNT // THREADS):
        with requests.post(url, data=json.dumps(data),
                           headers=headers) as response:
            pass


@perform_in_threads(THREADS)
def check_many(i):
    step = COUNT // THREADS
    for id_ in range(i * step + 1, (i + 1) * step + 1):
        url = f"http://localhost:8000/get/{id_}"
        with requests.get(url) as response:
            data = response.json()
            assert data['id'] == id_
            assert data['age'] == 32


@perform_in_threads(THREADS)
def update_many(i):
    step = COUNT // THREADS
    for id_ in range(i * step + 1, (i + 1) * step + 1):
        url = "http://localhost:8000/update"
        data = {
            'id': id_,
            'name': 'alex',
            'age': 33,
        }
        headers = {
            'Content-Type': 'application/json',
        }
        with requests.post(url, data=json.dumps(data),
                           headers=headers) as response:
            pass


if __name__ == "__main__":
    print(f"Count: {COUNT}")
    print(f"Threads: {THREADS}")

    with measure_time("Insert"):
        insert_many()

    with measure_time("Get"):
        check_many()

    with measure_time("Update"):
        update_many()
