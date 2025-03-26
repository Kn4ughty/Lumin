from loguru import logger as log

# import lumin.globals as g
import wn

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


def search(s: str) -> Gtk.Box:
    s = s.lower()
    log.info(f"dict recived text: '{s}'")
    main_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=0)
    main_box.add_css_class("dict")
    try:
        words = wn.synsets(s)
        log.info(f"Words found: {words}")
    except Exception as e:
        log.error(
            f"Failed to get word. Have you installed the dataset? \n \
            Error:'{e}'\n\
            Hint, try `make install`"
        )
        return

    if len(words) == 0:
        log.info(f"No definition for word: '{s}' found")
        return main_box

    heading = Gtk.Label(label=f"{s}")
    heading.add_css_class("title")
    main_box.append(heading)

    # https://docs.gtk.org/Pango/pango_markup.html
    display_str = ""
    for i in range(len(words)):
        example_str = ""
        if words[i].examples() != []:
            alph = "abcdefghijklmnopqrstuvwxyzz"
            for j in range(len(words[i].examples())):
                example_str += f"    {alph[j]}. {words[i].examples()[j]}\n"

        display_str += f"{i+1}. {words[i].definition()}\n{example_str}"

    log.info(f"display_str: {display_str}")

    label = Gtk.Label(label=display_str)
    main_box.append(label)

    return main_box


# def download():
#     nltk.download('wordnet')
