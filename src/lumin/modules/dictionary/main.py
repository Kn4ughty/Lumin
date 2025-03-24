import nltk
import os
from pathlib import Path
from loguru import logger as log
import csv

from lumin.modules.dictionary.download_dict import crawl
import lumin.globals as g

words_file_path = g.DATA_DIR.joinpath(g.ENGLISH_DICT_NAME)


def search(s: str):
    # nltk.edit_distance()

    with open(words_file_path) as f:
        data = csv.DictReader(f)
        for row in data:
            try:
                print(row[s])
            except KeyError:
                pass
        # print(data[s])

    # USE A BINARY SEARCH HERE.
    # FILE IS PRE SORTED
    # only find the correct word.
    # However if there is no correct word, show a list of nearby words maybe?
    # Alternately show nothign

    # def s(app) -> int:
    # sorted_result = sorted(apps, reverse=True, key=s)
    return []


def download():
    alph = "abcdefghijklmnopqrstuvwxyz"
    if os.environ.get("DEV"):
        alph = "x"
    # print("word,pos,definition")
    s = ""
    log.info("Downloading English dictionary")
    for word, pos, definition in crawl(alph):
        r = f'{word},{pos},"{definition}"'
        print(r)
        s += r

    with open(g.DATA_DIR.joinpath(g.ENGLISH_DICT_NAME), 'w') as f:
        f.write(s)
        f.close()
