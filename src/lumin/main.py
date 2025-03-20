from pathlib import Path
from loguru import logger as log
import sys
import threading

# Add the src/ directory to sys.path
sys.path.append(str(Path(__file__).resolve().parent.parent))

from lumin.gui.gtk_main import MyApp  # noqa
from lumin.models import result  # noqa
from lumin.models.result import Result  # noqa
from lumin.modules.app_launcher.main import search as app_search  # noqa

import gi  # noqa

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk, GObject, GLib  # noqa: E402


log.remove()
log.add(sys.stderr, level="INFO")

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
    print(thing)


def on_search_text_changed(search_box):
    log.info(f"Seach entry text changed. {search_box}.text = {search_box.get_text()}")

    text = search_box.get_text()

    if text == "":
        log.info("Search text was empty. Showing empty results")
        app.update_results(Gtk.Box())  # Make results empty
        return

    result_list = []

    # This will need to be abstracted when there are more search modes
    # Will need a generic way to thread results

    # The result fetching is done via threads so the UI can update while processing

    def run_search():
        output = []
        apps = app_search(text)
        GLib.idle_add(update_results, apps)

    def update_results(apps):
        for i in range(min(10, len(apps))):
            desktop_app = apps[i]
            result_list.append(Result(desktop_app.name, None, on_open))

        result_box = result.result_list_to_gtkbox(result_list)
        app.update_results(result_box)
        # Glib.idle expects a bool to indicate if the function should be repeated
        # This makes it run once and terminate
        return False

    # TODO. Profile the startup cost of creating a new thread

    search_thread = threading.Thread(target=run_search, daemon=True)
    search_thread.start()

    # apps = app_search(text))


if __name__ == "__main__":
    main()
