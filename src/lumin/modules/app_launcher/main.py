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
        i = 0
        for char in search_text:
            if char in app.name:
                i += 1
        return i

    return sorted(apps, reverse=True, key=s)

    # this is probably very slow

    # sorted_apps = []
    #
    # for app in apps:
    #     matching_count = 0
    #
