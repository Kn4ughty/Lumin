from pathlib import Path
from typing import List
import time
import os

import logging as log


# https://specifications.freedesktop.org/icon-theme-spec/latest/
def parse_xdg_file(lines: List[str]) -> dict:
    """example file
    [Icon Theme]
    Name=Adwaita
    Comment=The Only One
    Example=folder
    Inherits=AdwaitaLegacy,hicolor
    Hidden=true

    # Directory list
    Directories=16x16/actions,16x16/apps

    [16x16/actions]
    Context=Actions
    Size=16
    Type=Fixed

    [16x16/apps]
    Context=Applications
    Size=16
    Type=Fixed
    """

    entry = {}

    for line in lines:
        if line[0] == "#":
            log.debug(f"Skipping comment line: {line}")
            continue

        if line == "\n":
            log.debug(f"Skipping empty line: {line}")
            continue

        if line[0] == "[":  # ] my editor doesnt understand the bracket is quoted
            log.debug(f"ignoring unknown group header {line}")
            continue

        data = line.split("=", 1)

        # Remove any trailing whitespace
        key = data[0].strip()
        value = data[1].strip()

        entry[key] = value

    return entry


def get_XDG_DATA_DIRS() -> List[Path]:
    default_dir = "/usr/share/"

    XDG_DATA_DIRS = os.getenv("XDG_DATA_DIRS")

    if XDG_DATA_DIRS is None:
        log.warning(f"No value for $XDG_DATA_DIRS was found. \
                Setting to {default_dir}")

        XDG_DATA_DIRS = default_dir

        if not Path(default_dir).exists():
            log.error(f"The directory {default_dir} does not exist, \
                    and no value for $XDG_DATA_DIRS was found. ")
            raise Exception

    dirs = []


    dirs = [Path(dir) for dir in XDG_DATA_DIRS.split(":")]
    log.debug(f"found XDG_DATA_DIRS: {dirs}")

    return dirs


def get_all_desktop_apps() -> List[dict]:
    start_time = time.time()

    dirs = [dir.joinpath("applications") for dir in get_XDG_DATA_DIRS()]

    entries = []

    for search_dir in dirs:
        files = [f for f in search_dir.glob("*.desktop")]

        for file in files:
            log.debug(f"Reading .desktop file: {file}")

            with open(file, "r") as f:
                lines = f.readlines()

            entries.append(parse_xdg_file(lines))

    log.info(
        f"Time to parse all .desktop files: {
             (time.time() - start_time)*1000:.3f}ms"
    )
    # On m1 mac it takes about 9ms. \pm 1ms

    return entries


result = get_all_desktop_apps()
# print(result)
