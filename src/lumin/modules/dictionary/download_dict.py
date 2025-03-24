from bs4 import BeautifulSoup
import requests
import logging
import os
from pathlib import Path
import lumin.globals as g

logger = logging.getLogger(__name__)


def crawl(alph):
    for letter in alph:
        # this can run a long time its helpfull to know which letter its on
        logger.info(f"processing letter {letter}...")
        url = (
            "http://www.mso.anu.edu.au/~ralph/OPTED/v003/wb1913_" + letter + ".html"
        )  # there is a page for each letter
        req = requests.get(url)  # grab page
        soup = BeautifulSoup(req.text, "html.parser")  # get parser
        dictionary = soup.find_all("p")  # find all the dictionary entries
        for entries in dictionary:
            word = entries.find("b").getText()  # get the word itself
            pos = entries.find("i").getText()  # get the part of speech
            # calulate how much word and pos take up
            cut = len(word) + len(pos) + 4
            definition = entries.getText()[
                cut:
            ]  # cut that from the total sting to get definition
            yield word, pos, definition


def download():
    alph = "abcdefghijklmnopqrstuvwxyz"
    if os.environ.get("DEV"):
        alph = "x"
    # print("word,pos,definition")
    s = ""
    for word, pos, definition in crawl(alph):
        r = f'{word},{pos},"{definition}"'
        print(r)
        s += r

    with open(g.DATA_DIR.joinpath(Path('dict.csv')), 'w') as f:
        f.write(s)
        f.close()


if __name__ == "__main__":
    alph = "abcdefghijklmnopqrstuvwxyz"
    if os.environ.get("DEV"):
        alph = "x"
    # print("word,pos,definition")
    s = ""
    for word, pos, definition in crawl(alph):
        r = f'{word},{pos},"{definition}"'
        print(r)
        s += r

    with open(g.DATA_DIR.joinpath(Path('dict.csv')), 'w') as f:
        f.write(s)
        f.close()
