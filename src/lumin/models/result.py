from typing import Callable, List, Optional
from dataclasses import dataclass, field
import logging as log
import globals as g

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
    listbox.set_sort_func(s, "user data")
    scroll = Gtk.ScrolledWindow()
    scroll.set_vexpand(True)
    scroll.set_child(listbox)
    main_box.append(scroll)

    def invalidate():
        listbox.invalidate_sort()

    return main_box, invalidate


def s(listboxrow1, listboxrow2, user_input) -> int:
    # input_text = "firefox"
    input_text = g.awful_input_global
    input_text = input_text.lower()
    # print(input_text)

    # Maybe cache score of listboxes?

    score1 = 0
    score2 = 0

    name1 = listboxrow1.name
    name2 = listboxrow2.name

    score1 += longestCommonSubstr(name1, input_text)
    score2 += longestCommonSubstr(name2, input_text)
    # score += longestCommonSubstr(search_text, result.generic_name.lower())

    # if search_text[0] == result.display_str.lower()[0]:
    #     score += 2

    return score2 - score1


# Thank you https://www.geeksforgeeks.org/longest-common-substring-dp-29/


def longestCommonSubstr(s1, s2) -> int:
    m = len(s1)
    n = len(s2)

    # Create a 1D array to store the previous row's results
    prev = [0] * (n + 1)

    res = 0
    for i in range(1, m + 1):
        # Create a temporary array to store the current row
        curr = [0] * (n + 1)
        for j in range(1, n + 1):
            if s1[i - 1] == s2[j - 1]:
                curr[j] = prev[j - 1] + 1
                res = max(res, curr[j])
            else:
                curr[j] = 0

        # Move the current row's data to the previous row
        prev = curr

    return res
