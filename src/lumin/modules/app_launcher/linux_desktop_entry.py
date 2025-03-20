from pathlib import Path
from typing import List
import time
import os

# from loguru import logger as log
import logging as log


from lumin.modules.app_launcher.models import DesktopApp

# https://specifications.freedesktop.org/icon-theme-spec/latest/


def get_XDG_DATA_DIRS() -> List[Path]:
    default_dir = "/usr/share/"

    XDG_DATA_DIRS = os.getenv("XDG_DATA_DIRS")

    if XDG_DATA_DIRS is None:
        log.warning(
            f"No value for $XDG_DATA_DIRS was found. \
                Setting to {default_dir}"
        )

        XDG_DATA_DIRS = default_dir

        if not Path(default_dir).exists():
            log.error(
                f"The directory {default_dir} does not exist, \
                    and no value for $XDG_DATA_DIRS was found. "
            )
            raise Exception

    dirs = []

    dirs = [Path(dir) for dir in XDG_DATA_DIRS.split(":")]
    log.debug(f"found XDG_DATA_DIRS: {dirs}")

    return dirs


def str_to_bool(s: str) -> bool:
    s = s.lower()

    match s:
        case "false":
            return False

        case "true":
            return True
        case _:
            log.warning(f"Str to bool was given bad data. s: {s}")
            return False


# TODO Expand this
def dict_to_desktop_app(app: dict) -> DesktopApp:
    """ """

    log.debug(f"dict_to_desktop_app recieved: {app}")
    # Using dict.get with default values to handle empty cases

    result = DesktopApp(
        name=app["Name"],
        cmd_to_execute=app["Exec"],
        terminal=str_to_bool(app.get("Terminal", "False")),
    )

    return result


def get_all_desktop_apps() -> List[DesktopApp]:
    start_time = time.time()

    dirs = [dir.joinpath("applications") for dir in get_XDG_DATA_DIRS()]

    entries = []

    for search_dir in dirs:
        files = [f for f in search_dir.glob("*.desktop")]

        for file in files:
            log.debug(f"Reading .desktop file: {file}")

            with open(file, "r") as f:
                lines = f.readlines()

            entry = {}

            for line in lines:
                if line[0] == "#":
                    log.debug(f"Skipping comment line: {line}")
                    continue

                if line == "\n":
                    log.debug(f"Skipping empty line: {line}")
                    continue

                if (
                    line[0] == "["
                ):  # ] my editor doesnt understand the bracket is quoted
                    log.debug(f"ignoring unknown group header {line}")
                    continue

                data = line.split("=", 1)

                # Remove any trailing whitespace
                key = data[0].strip()
                value = data[1].strip()

                entry[key] = value

            if str_to_bool(entry.get("NoDisplay", "False")):
                continue

            entries.append(dict_to_desktop_app(entry))

    log.info(
        f"Time to parse all .desktop files: {(time.time() - start_time) * 1000:.3f}ms"
    )
    # On m1 mac it takes about 9ms. \pm 1ms

    return entries


if __name__ == "__main__":
    result = get_all_desktop_apps()
    print(result[0])
