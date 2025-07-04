from typing import Callable, List, Optional
from dataclasses import dataclass, field
import os

# import logging as log
from fastlog import logger as log
import globals as g

from sort import sort_apps as sort_apps

import gi

gi.require_version("Gtk", "4.0")
gi.require_version("Gio", "2.0")
from gi.repository import Gtk, Gio  # noqa: E402


# This is yucky.
# It gets set by the gui startup code to the search entry box
# I dont have a better way to do this yet
# It is here so that the sorting function can be attached to it.
# Maybe
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
    # log.debug(f"result_list = {result_list}")
    log.info(len(result_list))
    main_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=0)
    listbox = Gtk.ListBox()

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

        if item.icon is not None:
            box.append(Gtk.Image.new_from_gicon(item.icon))

        name_label = Gtk.Label(label=item.display_str)
        box.append(name_label)
        row.name = item.display_str

        if item.generic_name != "":
            generic_name_label = Gtk.Label(label=f"({item.generic_name})")
            generic_name_label.add_css_class("subtitle")
            box.append(generic_name_label)
            row.generic_name = item.generic_name

        row.set_child(box)

        row.activate_callback = item.open_action

        listbox.append(row)

    listbox.connect("row-activated", activate_result)  # Arrow + Enter handling
    listbox.set_sort_func(sort, "user data")
    scroll = Gtk.ScrolledWindow()
    scroll.set_vexpand(True)
    scroll.set_child(listbox)
    main_box.append(scroll)

    def invalidate():
        listbox.invalidate_sort()

    return main_box, invalidate


class Run:
    def __init__(self, main: Callable):
        self.callable = main

    def __call__(self, *args):

        log.info(f"callable: {self.callable}")
        env = os.environ.copy()
        env.pop("VIRTUAL_ENV", None)
        env["PATH"] = "/usr/bin:" + env["PATH"]
        launch_context = Gio.AppLaunchContext()
        launch_context.setenv("PATH", env["PATH"])
        self.callable(context=launch_context)

        # TODO Log Search frequency

        exit()


def sort(listboxrow1, listboxrow2, user_input) -> int:
    input_text = g.search_input_global
    input_text = input_text.lower()

    name1 = listboxrow1.name.lower()
    name2 = listboxrow2.name.lower()

    score = sort_apps(name1, name2, input_text)

    return score
