import time

start_time = time.perf_counter()
from pathlib import Path  # noqa: E402
import os  # noqa: E402
import sys  # noqa: E402
from loguru import logger as log  # noqa: E402


log.remove()
log.add(sys.stderr, level="DEBUG")
if not os.path.exists(Path("./logs/")):
    os.mkdir("logs")
log.add("logs/{time}", level="INFO", rotation="1 day")


# Add the src/ directory to sys.path
sys.path.append(str(Path(__file__).resolve().parent.parent))

import_gtk_time = time.perf_counter()

from lumin.gui.gtk_main import MyApp  # noqa

log.info(f"Gui import time: {(time.perf_counter() - import_gtk_time) * 1000:.2f}ms")
del import_gtk_time


log.info(f"Mandatory imports: {(time.perf_counter() - start_time) * 1000:.2f}ms")


def on_search_activate(search_box):
    log.info("Search box activated")


def on_search_text_changed(search_box):
    # Lazy loading is done here to improve startup time.
    # Without it, I end up dropping inputs
    # because I start typing before there is a gui
    search_start_import = time.perf_counter()
    from lumin.modules.app_launcher.main import search as app_search  # noqa
    import threading  # noqa: E402

    log.info(
        f"app_search import time: {(time.perf_counter() - search_start_import) * 1000:.2f}ms"
    )

    log.info(
        f"Search import time: {
            (time.perf_counter() - search_start_import) * 1000:.2f
        }ms"
    )

    log.debug(f"Seach entry text changed. {search_box}.text = {search_box.get_text()}")

    text: str = search_box.get_text()

    if text == "":
        log.info("Search text was empty. Showing empty results")
        app.update_results(None)  # Make results empty
        return

    search = app_search

    # Ideally i'd use a match statement, but the prefix's are not fixed length
    if text[:2] == "!d":
        # dict_start = time.perf_counter()
        from lumin.modules.dictionary.main import search as dictionary_search  # noqa

        search = dictionary_search
        text = text[2:].strip()

    if text[:1] == "/":
        from lumin.modules.calc.main import calc_func

        search = calc_func
        text = text[1:]

    # Create a new thread
    # This thread does the app search,
    # and then Glib updates the results on the main thread

    def run_search():
        result_box = search(text)
        log.debug(f"Result received from search: {result_box}")
        from gi.repository import GLib  # noqa: E402

        GLib.idle_add(update_results, result_box)

    def update_results(result_box):
        app.update_results(result_box)
        # Glib.idle expects a bool to indicate if function should be repeated
        # This makes it run once and terminate
        return False

    search_thread = threading.Thread(target=run_search, daemon=True)
    search_thread.start()


def main():

    global app
    if "install" in sys.argv:
        return 0
    log.info("Gui being initialised")
    app = MyApp(on_search_text_changed, on_search_activate)
    log.info(f"Time to gui: {(time.perf_counter() - start_time) * 1000:.2f}ms")
    app.run()


if __name__ == "__main__":
    main()
