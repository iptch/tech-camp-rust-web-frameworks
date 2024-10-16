import random
import re
import string
from typing import Dict
from urllib import parse as urlparse

import pandas as pd
from locust import task, FastHttpUser, run_single_user

HEADER = "desc"

def random_string():
    length = random.randint(8,16)
    letters = string.ascii_lowercase
    return ''.join(random.choice(letters) for _ in range(length))


class ERFAUser(FastHttpUser):
    ids: Dict[str, str] = {}

    def on_start(self):
        #l = [string.ascii_lowercase * 1000] * 100
        #self.data = pd.concat([pd.read_csv("/assets/wine_small.csv"), pd.DataFrame(l, columns=['desc'])], ignore_index=True)
        self.data = pd.read_csv("/assets/wine_small.csv")
        for _ in range(10):
            self.post()

    @task(3)
    def post(self):
        text = self.data.sample()[HEADER].values[0]
        text = re.sub(r"[^\x00-\x7f]", "", text)
        with self.rest("POST", "/texts", json={"data": text}) as resp:
            if not resp.js["id"]:
                resp.failure(
                    f"Unexpected value of id in response ({resp.status_code}): {resp.text}"
                )
            else:
                self.ids[resp.js["id"]] = text

    @task(1)
    def delete(self):
        if len(self.ids) < 1:
            return
        uuid = random.choice(list(self.ids.keys()))
        with self.rest("DELETE", f"/texts/{uuid}") as resp:
            if resp.status_code != 204:
                resp.failure(
                    f"Failed to delete element ({resp.status_code}): {resp.text}"
                )
            else:
                del self.ids[uuid]

    @task(10)
    def get(self):
        if len(self.ids) < 1:
            return
        uuid = random.choice(list(self.ids.keys()))
        with self.rest("GET", f"/texts/{uuid}") as resp:
            if resp.js["data"] != self.ids[uuid]:
                resp.failure(
                    f"Data returned not expected from cache ({resp.status_code}): {resp.text}"
                )

    @task(10)
    def search(self):
        if len(self.ids) < 1:
            return
        uuid = random.choice(list(self.ids.keys()))
        word = ""
        while word == "":
            word = random.choice(self.ids[uuid].split(" "))
        exists = True
        if random.random() > 0.5:
            exists = False
            word = random_string()

        with self.rest(
                "GET", f"/texts/{uuid}/search?term={urlparse.quote_plus(word)}"
        ) as resp:
            if resp.js["found"] != exists:
                resp.failure(
                    f"Incorrect response for search term '{word}' in '{self.ids[uuid]}' : {resp.text}"
                )
if __name__ == "__main__":
    run_single_user(ERFAUser)