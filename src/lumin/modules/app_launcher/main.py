import platform
from loguru import logger as log
import time
from typing import List
import subprocess

import lumin.modules.app_launcher.linux_desktop_entry as linux_desktop_entry


from lumin.models.result import Result
import lumin.models.result as result_module

OS = platform.system()


def search(search_text: str) -> List[Result]:
    search_start_time = time.perf_counter()

    search_text = search_text.lower()

    match OS:
        case "darwin":
            apps = []
            pass
        case "Linux":
            apps = linux_desktop_entry.get_all_desktop_apps()
        case _:
            raise SystemError
    log.debug(apps)

    app_get_end_time = time.perf_counter()

    sorting_start_time = time.perf_counter()

    def s(app) -> int:
        score = 0

        score += longestCommonSubstr(search_text, app.name.lower())

        if search_text[0] == app.name.lower()[0]:
            score += 2

        return score

    sorted_result = sorted(apps, reverse=True, key=s)

    log.debug(f"Sorted result: {sorted_result[0:10]}")

    log.info(
        f"App Sorting time: {(
            (time.perf_counter() - sorting_start_time) * 1000):.4f}ms"
    )
    log.info(
        f"App list getting time: {(
            (app_get_end_time - search_start_time) * 1000):.4f}ms"
    )

    class Run:
        def __init__(self, command):
            self.command = command

            self.fn = lambda: subprocess.Popen(command, start_new_session=True)

        def __call__(self):
            log.info(f"Command being run: {self.command}")
            self.fn()
            exit(0)

    apps = []

    for app in sorted_result:
        # I tried using a def here to create the function,
        # but it seemed to get overridden with the last value
        # def a(thing): return print(result.cmd_to_execute, thing)
        # It also didnt work with lambda, even if it was created in the append

        # So instead I need to do this rubbish instead.
        # I wish I was using a language with proper scoping rules

        apps.append(Result(app.name, None, Run(app.cmd_to_execute)))

    result_list = []
    for i in range(min(10, len(apps))):
        result_list.append(apps[i])

    result_element = result_module.result_list_to_gtkbox(result_list)

    log.info(
        f"App total time: {(
            (time.perf_counter() - search_start_time) * 1000):.4f}ms"
    )
    return result_element


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
