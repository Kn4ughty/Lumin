import os
import time
from typing import Callable, List

from lumin.models.result import Result, Run
import lumin.models.result as result_module
from lumin.fastlog import logger as log
import lumin.globals as g

from . import macos_search

import gi

gi.require_version("Gtk", "4.0")
gi.require_version("Gio", "2.0")
from gi.repository import Gtk, Gio  # noqa: E402


# This is done for caching
global apps
apps = []


def get_linux_apps() -> List[Result]:
    output = []
    for app_info in apps:
        display_name = app_info.get_display_name()

        exec = Run(app_info.launch)

        icon = app_info.get_icon()

        if (generic_name := app_info.get_generic_name()) is None:
            generic_name = ""

        output.append(
            Result(
                display_str=display_name,
                icon=icon,
                open_action=exec,
                generic_name=generic_name,
            )
        )
    return output


result_list = []


def search() -> Gtk.Box:
    global result_list, apps

    if len(result_list) == 0:
        if g.PLATFORM_OS == "darwin":
            if len(apps) != 0:
                apps = macos_search.get_app_file_paths()
            result_list = macos_search.get_macos_apps(apps)
        elif g.PLATFORM_OS == "linux":
            # Previously I had done all of the .desktop parsing myself, which was actually quite fun. 
            # Then I found that GTK had a function for that built in. >:|
            # However it seems to be missing some features (like desktop actions) 
            apps = Gio.AppInfo.get_all()
            result_list = get_linux_apps()
        # No else needed as result list is empty

    start_time = time.perf_counter()

    result_box, invalidate = result_module.result_list_to_gtkbox(result_list)
    log.perf("Time to search for app", start_time)
    return result_box, invalidate


if __name__ == "__main__":
    search("insomnia")
