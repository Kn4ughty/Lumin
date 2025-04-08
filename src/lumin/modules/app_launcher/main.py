import os
import time
from typing import Callable

from lumin.models.result import Result
import lumin.models.result as result_module
from lumin.fastlog import logger as log

import gi

gi.require_version("Gtk", "4.0")
gi.require_version("Gio", "2.0")
from gi.repository import Gtk, Gio  # noqa: E402


apps = Gio.AppInfo.get_all()


class Run:
    def __init__(self, main: Callable):
        self.main = main

    def __call__(self, *args):
        env = os.environ.copy()
        env.pop("VIRTUAL_ENV", None)
        env["PATH"] = "/usr/bin:" + env["PATH"]
        launch_context = Gio.AppLaunchContext()
        launch_context.setenv("PATH", env["PATH"])
        self.main(context=launch_context)
        exit()


result_list = []


for app_info in apps:
    display_name = app_info.get_display_name()

    exec = Run(app_info.launch)

    icon = app_info.get_icon()

    if (generic_name := app_info.get_generic_name()) is None:
        generic_name = ""

    result_list.append(
        Result(
            display_str=display_name,
            icon=icon,
            open_action=exec,
            generic_name=generic_name,
        )
    )


def search() -> Gtk.Box:
    global apps, result_list
    start_time = time.perf_counter()

    result_box, invalidate = result_module.result_list_to_gtkbox(result_list)
    log.perf("Time to search for app", start_time)
    return result_box, invalidate


if __name__ == "__main__":
    search("insomnia")
