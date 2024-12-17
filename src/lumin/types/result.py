from typing import Callable, List
from dataclasses import dataclass

import gi

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk, Gdk  # noqa: E402


@dataclass
class Result:
    display_str: str
    icon: None  # TODO work out this type
    open_action: Callable


def result_list_to_gtkbox(result_list: List[Result]):
    main_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=0)

    for item in result_list:
        entry = Gtk.Box()

        text_label = Gtk.Label(label=item.display_str)

        entry.append(text_label)
        main_box.append(entry)

    return main_box
