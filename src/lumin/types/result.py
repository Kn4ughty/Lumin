from typing import Callable, List
from dataclasses import dataclass
import logging as log

import gi

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk, Gdk  # noqa: E402

log.getLogger(__name__)


@dataclass(frozen=True)
class Result:
    display_str: str
    icon: None  # TODO work out this type
    open_action: Callable


def result_list_to_gtkbox(result_list: List[Result]):
    log.info("Turning results list into a gtkbox.")
    log.debug(f"result_list = {result_list}")
    main_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=0)

    for item in result_list:
        log.debug(f"For loop. item = {item}")
        frame = Gtk.Frame()
        frame.set_label_align(0.0)

        text_label = Gtk.Label(label=item.display_str)

        frame.set_child(text_label)
        main_box.append(frame)

    return main_box
