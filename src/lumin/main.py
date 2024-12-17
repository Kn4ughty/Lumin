import sys
from pathlib import Path

# Add the src/ directory to sys.path
sys.path.append(str(Path(__file__).resolve().parent.parent))

from lumin.gui.gtk_main import MyApp
from lumin.types import result
from lumin.types.result import Result 


def main():
    global app
    app = MyApp(on_search_text_changed, on_search_activate)
    app.run()


def on_search_activate(search_box):
    print("Search Activated")
    pass

def on_open():
    print("woah i was opened")

def on_search_text_changed(search_box):
    print("Search text changed")
    text = search_box.get_text()
    
    # Create fake results
    result_list = []
    for i in range(10):
        result_list.append(Result(f"Result {text} {i}", None, on_open))
        
    result_box = result.result_list_to_gtkbox(result_list)
    app.update_results(result_box)

    print(result_box.get_first_child)



    print(search_box.get_text())



if __name__ == "__main__":
    main()
