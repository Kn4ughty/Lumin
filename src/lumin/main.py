import sys
from pathlib import Path

# Add the src/ directory to sys.path
sys.path.append(str(Path(__file__).resolve().parent.parent))

from lumin.gui.gtk_main import MyApp
# from lumin import query


def main():
    app = MyApp(on_search_text_changed, on_search_activate)
    app.run()


def on_search_activate(search_box):
    print("Search Activated")
    pass


def on_search_text_changed(search_box):
    print("Search text changed")
    # query.parse()
    print(search_box.get_text())


if __name__ == "__main__":
    main()
