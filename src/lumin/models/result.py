from typing import Callable, List
from dataclasses import dataclass
import logging as log

import gi

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk  # noqa: E402

log.getLogger(__name__)


@dataclass(frozen=True)
class Result:
    display_str: str
    icon: None  # TODO work out this type
    open_action: Callable


def result_list_to_gtkbox(result_list: List[Result]):
    """
    This method is public because I
    expect to reuse it when doing web searching.
    """
    log.info("Turning results list into a gtkbox.")
    log.debug(f"result_list = {result_list}")
    main_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=0)
    listbox = Gtk.ListBox()

    def activate_result(list_box, list_box_row):
        list_box_row.activate_callback()

    # for item in result_list:
    for i in range(len(result_list)):
        item = result_list[i]
        row = Gtk.ListBoxRow()
        box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
        label = Gtk.Label(label=item.display_str)
        box.append(label)
        row.set_child(box)

        row.activate_callback = item.open_action

        listbox.append(row)

    listbox.connect("row-activated", activate_result)  # Arrow + Enter handling
    main_box.append(listbox)

    return main_box
