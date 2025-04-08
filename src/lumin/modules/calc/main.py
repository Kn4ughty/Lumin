from qalc import PyCalculator  # noqa: E402

import gi  # noqa: E402

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk  # noqa: E402


calc = PyCalculator()
calc.load_exchange_rates()
calc.load_global_definitions()
calc.load_local_definitions()


def calc_func(s: str):
    global calc
    label = Gtk.Label(label=calc.calculate(s, 2000))
    label.add_css_class("title")
    return label
