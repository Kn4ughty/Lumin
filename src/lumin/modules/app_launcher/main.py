import os
import time
from typing import Callable
import subprocess

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

    def __call__(self):
        env = os.environ.copy()
        env.pop("VIRTUAL_ENV", None)
        env["PATH"] = "/usr/bin:" + env["PATH"]
        launch_context = Gio.AppLaunchContext()
        launch_context.setenv("PATH", env["PATH"])
        self.main(context=launch_context)


def search(search_text: str) -> Gtk.Box:
    global apps
    start_time = time.perf_counter()

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

    def s(result: Result) -> int:
        score = 0

        score += longestCommonSubstr(search_text, result.display_str.lower())
        score += longestCommonSubstr(search_text, result.generic_name.lower())

        if search_text[0] == result.display_str.lower()[0]:
            score += 2

        return score

    sorted_result = sorted(result_list, reverse=True, key=s)

    result_box = result_module.result_list_to_gtkbox(sorted_result)
    log.perf("Time to search for app", start_time)
    return result_box


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


if __name__ == "__main__":
    search("insomnia")
