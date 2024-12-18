import sys
from pathlib import Path
import logging as log

# Add the src/ directory to sys.path
sys.path.append(str(Path(__file__).resolve().parent.parent))

from lumin.gui.gtk_main import MyApp
from lumin.types import result
from lumin.types.result import Result

import gi

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk  # noqa: E402

# log.basicConfig(level=log.DEBUG)
log.basicConfig(
    level=log.INFO,
    format="{asctime} - {levelname} - {message}",
    style="{",
    datefmt="%H:%M:%S",
)
log.info("ASKLDJLKSDFJLKSDFJj")


def main():
    global app
    log.info("Gui being initialised")
    app = MyApp(on_search_text_changed, on_search_activate)
    log.info("Running GUI")
    app.run()


def on_search_activate(search_box):
    log.info("Search box activated")


def on_open():
    log.info("Thing opened")


def on_search_text_changed(search_box):
    log.info(f"Seach entry text changed. {search_box}.text = {search_box.get_text()}")

    text = search_box.get_text()

    if text == "":
        log.info("Search text was empty. Showing empty results")
        app.update_results(Gtk.Box())  # Make results empty
        return

    # Create fake results
    result_list = []
    for i in range(10):
        result_list.append(Result(f"Result {text} {i}", None, on_open))

    result_box = result.result_list_to_gtkbox(result_list)
    app.update_results(result_box)


if __name__ == "__main__":
    main()
