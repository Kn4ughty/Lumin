from typing import Callable


class result:
    display_str: str
    open: Callable

    def __init__(self, display_str: str, open: Callable):
        self.display_str = display_str
        self.open = open

"""
Main search function
Takes in search text, and outputs a list of results
"""
def search():
    # Takes in text
    # Returns list of "results"

    print("parsing")

def example_open():
    print("OPENED THING")

def find_thing():
    pass
