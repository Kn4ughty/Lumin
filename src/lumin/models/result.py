from typing import Callable, List, Optional
from dataclasses import dataclass, field
import logging as log

import gi

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk  # noqa: E402

log.getLogger(__name__)

# This is yucky.
# It gets set by the gui startup code to the search entry thing
# I dont have a better way to do this yet
search_entry = None
search_activate_signal = None


@dataclass(frozen=True)
class Result:
    display_str: str
    open_action: Callable
    icon: Optional[Gtk.Image] = None
    keywords: List[str] = field(default_factory=list)
    generic_name: str = ""


def result_list_to_gtkbox(result_list: List[Result]) -> Gtk.Box():
    """
    This method is public because I
    expect to reuse it when doing web searching.
    """
    global search_entry
    global search_activate_signal

    log.info("Turning results list into a gtkbox.")
    log.debug(f"result_list = {result_list}")
    main_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=0)
    listbox = Gtk.ListBox()

    # if search_entry is not None:
    #     log.warning("search entry found")
    #     search_entry.connect("activate", result_list[0].open_action)

    def activate_result(list_box, list_box_row):
        list_box_row.activate_callback()

    # for item in result_list:
    for i in range(len(result_list)):
        if i == 0:
            log.warning("search entry found")
            log.info(result_list[0])
            if search_activate_signal is not None:
                search_entry.disconnect(search_activate_signal)
            search_activate_signal = search_entry.connect(
                "activate", result_list[0].open_action
            )

        item = result_list[i]
        row = Gtk.ListBoxRow()
        box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
        name_label = Gtk.Label(label=item.display_str)
        if item.icon is not None:
            box.append(item.icon)
        box.append(name_label)
        if item.generic_name != "":
            generic_name_label = Gtk.Label(label=f"({item.generic_name})")
            generic_name_label.add_css_class("subtitle")
            box.append(generic_name_label)

        row.set_child(box)

        row.activate_callback = item.open_action

        listbox.append(row)

    listbox.connect("row-activated", activate_result)  # Arrow + Enter handling
    scroll = Gtk.ScrolledWindow()
    scroll.set_vexpand(True)
    scroll.set_child(listbox)
    main_box.append(scroll)

    return main_box
