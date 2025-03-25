import nltk
from nltk.corpus import wordnet as wn
from loguru import logger as log

import lumin.globals as g

import gi

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk  # noqa: E402

# words_file_path = g.DATA_DIR.joinpath(g.ENGLISH_DICT_NAME)

# this dataset has problems.
# Halfway through the R letters, it breaks,
# and the definition becomes 500k characters long

# Look at:
# https://www.gutenberg.org/cache/epub/29765/pg29765.txt
# https://stackoverflow.com/questions/6441975/where-can-i-download-english-dictionary-database-in-a-text-format  # noqa

# https://www.nltk.org/howto/wordnet.html


def search(s: str):
    s = s.lower()
    log.info(f"dict recived text: '{s}'")
    main_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=0)
    try:
        words = wn.synsets(s)
        log.info(f"Words found: {words}")
    except LookupError:
        log.error("Failed to get word. Have you installed the dataset?")

    if len(words) == 0:
        log.info(f"No definition for word: '{s}' found")
        return main_box

    # print(dir(words[0]))
    cleaned_word_list = []
    for word in words:
        name: str = word.name()
        print(name)
        split_name = name.split(".")
        log.info(f"Split name = {split_name}")
        if split_name[0] == s:
            cleaned_word_list.append(word)

    log.info(f"Cleaned words = {cleaned_word_list}")

    # https://docs.gtk.org/Pango/pango_markup.html
    display_str = ""
    for word in cleaned_word_list:
        word_type = word.name().split(".")[1]
        word_names = [str(lemma.name()) for lemma in word.lemmas()]
        definition = word.definition()
        item = f"{word_names}. {word_type}. \n {definition}"
        display_str += item

    log.info(f"display_str: {display_str}")

    label = Gtk.Label(label=display_str)
    main_box.append(label)

    return main_box


def download():
    nltk.download('wordnet')
