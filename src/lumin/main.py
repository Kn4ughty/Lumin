import sys
from pathlib import Path

# Add the src/ directory to sys.path
sys.path.append(str(Path(__file__).resolve().parent.parent))

from lumin.gui.gtk_main import GTKApp

def main():
    app = GTKApp(on_search_activate, on_search_text_changed)
    app.run()


def on_search_activate(search_box):
    print("Search Activated")
    print(search_box)
    pass


def on_search_text_changed(search_box):
    print("Search text changed")
    print(search_box)
    pass


if __name__ == "__main__":
    main()
