from pathlib import Path
from loguru import logger as log
import sys
import threading
import os
import sys

# Add the src/ directory to sys.path
sys.path.append(str(Path(__file__).resolve().parent.parent))

from lumin.gui.gtk_main import MyApp  # noqa
from lumin.models import result  # noqa
import lumin.modules.dictionary as dictionary_module  # noqa
from lumin.modules.app_launcher.main import search as app_search  # noqa
from lumin.modules.dictionary.main import search as dictionary_search  # noqa

import gi  # noqa

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk, GLib  # noqa: E402


log.remove()
log.add(sys.stderr, level="INFO")
if not os.path.exists(Path("./logs/")):
    os.mkdir("logs")
log.add("logs/{time}", level="INFO", rotation="1 day")


# # log.basicConfig(level=log.DEBUG)
# log.basicConfig(
#     level=log.INFO,
#     format="{asctime}-{levelname}: {message}",
#     style="{",
#     datefmt="%H:%M:%S",
# )
log.debug("ASKLDJLKSDFJLKSDFJj")


def main():
    global app
    log.info("Gui being initialised")
    app = MyApp(on_search_text_changed, on_search_activate)
    log.info("Running GUI")
    app.run()


def on_search_activate(search_box):
    log.info("Search box activated")


def on_open(thing):
    log.info("REVIUVTED EVENT", thing)


def on_search_text_changed(search_box):
    log.debug(
        f"Seach entry text changed. {
            search_box}.text = {search_box.get_text()}"
    )

    text = search_box.get_text()

    if text == "":
        log.info("Search text was empty. Showing empty results")
        app.update_results(Gtk.Box())  # Make results empty
        return

    search = app_search

    # Ideally a use match statement, but the prefix's are not fixed length
    if text[:2] == "!d":
        search = dictionary_search
        text = text[3:]

    # Create a new thread
    # This thread does the app search,
    # and then Glib updates the results on the main thread

    def run_search():
        result_box = search(text)
        log.debug(f"Result received from search: {result_box}")
        GLib.idle_add(update_results, result_box)

    def update_results(result_box):
        app.update_results(result_box)
        # Glib.idle expects a bool to indicate if function should be repeated
        # This makes it run once and terminate
        return False

    # TODO. Profile the startup cost of creating a new thread

    search_thread = threading.Thread(target=run_search, daemon=True)
    search_thread.start()

    # apps = app_search(text))


if __name__ == "__main__":
    if 'install' in sys.argv:
        dictionary_module.main.download()

    main()
