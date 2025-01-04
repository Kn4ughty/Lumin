import platform
from dataclasses import dataclass
from pathlib import Path

import linux_desktop_entry

import gi
gi.require_version("Gtk", "4.0")
from gi.repository import Gtk  # noqa: E402

OS = platform.system()

@dataclass
class desktop_app():
    name: str
    cmd_to_execute: str
    generic_name: str = ""
    keywords: list[str] = []
    catagoires: list[str] = []
    icon: Gtk.Image = None
    terminal: bool = False



def search(search_text: str):
    match OS:
        case "darwin":
            pass
        case "linux":
            pass
