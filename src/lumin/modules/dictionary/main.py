from fastlog import logger as log
import time

# import lumin.globals as g
import wn

wn.config.allow_multithreading = True

import gi  # noqa: E402

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk  # noqa: E402


def search(s: str) -> Gtk.Box:
    start_time = time.perf_counter()
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
        return main_box

    if len(words) == 0:
        log.info(f"No definition for word: '{s}' found")
        return main_box

    heading = Gtk.Label(label=f"{s[0].upper() + s[1:]}")
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

    log.debug(f"display_str: {display_str}")

    scroll = Gtk.ScrolledWindow()
    scroll.set_vexpand(True)  # Allow vertical expansion

    label = Gtk.Label(label=display_str)
    label.set_wrap(True)
    label.add_css_class("dict-body")
    scroll.set_child(label)
    main_box.append(scroll)

    log.info(f"Dictionary time: {(time.perf_counter() - start_time) * 1000:.3f}ms")

    return main_box


# def download():
#     nltk.download('wordnet')
