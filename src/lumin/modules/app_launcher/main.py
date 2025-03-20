import platform
from dataclasses import dataclass, field
from loguru import logger as log

import lumin.modules.app_launcher.linux_desktop_entry as linux_desktop_entry

# import gi
#
# gi.require_version("Gtk", "4.0")
# from gi.repository import Gtk  # noqa: E402
#
OS = platform.system()


def search(search_text: str):
    match OS:
        case "darwin":
            apps = []
            pass
        case "Linux":
            apps = linux_desktop_entry.get_all_desktop_apps()
        case _:
            raise SystemError
    log.debug(apps)

    search_text = search_text.lower()

    def s(app) -> int:
        # Find longest matching substring
        return longestCommonSubstr(search_text, app.name.lower())

    return sorted(apps, reverse=True, key=s)


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
