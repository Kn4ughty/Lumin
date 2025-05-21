# This startup code is soooooo ugly
import time

start_time = time.perf_counter()
from pathlib import Path  # noqa: E402
import sys  # noqa: E402

fast_log_start = time.perf_counter()
from fastlog import logger as log  # noqa: E402

print(f"fastlogtime: {(time.perf_counter() - fast_log_start) * 1000:.2f}ms")

# Add the src/ directory to sys.path
sys.path.append(str(Path(__file__).resolve().parent.parent))
import globals as g  # noqa: E402

import_gtk_time = time.perf_counter()

# Use the wayland gtk Layer shell to make window overlay if on wayland
# see here for reference:
# https://github.com/wmww/gtk4-layer-shell/blob/main/examples/simple-example.py

if g.IS_WAYLAND:
    from ctypes import CDLL

    CDLL("libgtk4-layer-shell.so")

from lumin.gui.gtk_main import MyApp  # noqa

log.perf("Gui import time", import_gtk_time)
del import_gtk_time

from gi.repository import GLib  # noqa: E402

log.perf("Mandatory imports", start_time)


def on_search_activate(search_box):
    log.info("Search box activated")


in_app_search = False
invalidate_callback = None


def on_search_text_changed(search_box):
    global in_app_search, invalidate_callback
    # Lazy loading is done here to improve startup time.
    # Without it, I end up dropping inputs
    # because I start typing before there is a gui
    search_start_import = time.perf_counter()
    from lumin.modules.app_launcher.main import search as app_search  # noqa

    log.perf("app_search import time", search_start_import)

    text: str = search_box.get_text()
    log.debug(f"Seach entry text changed. {search_box}.text = {text}")

    search = app_search

    if text[:2] == "!d":
        from lumin.modules.dictionary.main import search as dictionary_search  # noqa

        search = dictionary_search
        text = text[2:].strip()
        in_app_search = False

    if text[:1] == "/":
        from lumin.modules.calc.main import calc_func

        search = calc_func
        text = text[1:]
        in_app_search = False

    def run_search():
        global in_app_search, invalidate_callback
        search_start_time = time.perf_counter()

        if search == app_search:
            # To prevent memory leaks, dont redo the search.
            # Otherwise creating all the new icons and such every update creates crashes
            g.awful_input_global = text
            if not in_app_search:
                # was not in app search last time.
                # Need to get new results
                result_box, invalidate_callback = app_search()
                app.update_results(result_box)
                in_app_search = True
            else:
                invalidate_callback()
            log.perf("App_search time", search_start_time)
            return False

        search_func_time = time.perf_counter()
        result_box = search(text)

        app.update_results(result_box)
        log.perf("search funciton time", search_func_time)
        # Glib.idle expects a bool to indicate if function should be repeated
        return False

    GLib.idle_add(run_search)


def main():
    global app
    log.info("Gui being initialised")
    app = MyApp(on_search_text_changed, on_search_activate)
    log.info(f"Time to gui: {(time.perf_counter() - start_time) * 1000:.2f}ms")
    app.run()


if __name__ == "__main__":
    main()
