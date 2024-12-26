from dataclasses import dataclass, field
from pathlib import Path
from typing import List
import time

import logging as log

# TODO. Work this out
# https://specifications.freedesktop.org/icon-theme-spec/latest/


class AppIcon():
    pass


def get_all_desktop_apps(search_dir: Path) -> List[dict]:

    start_time = time.time()

    # Get all .destop files in search_dir
    files = [f for f in search_dir.glob("*.desktop")]

    entries = []

    for file in files:
        entries.append(parse_desktop_file(file))

    log.info(f"Time to parse all .desktop files: {
             (time.time() - start_time)*1000:.3f}ms")
    return entries


def parse_desktop_file(file: Path) -> dict:
    ''' example file
    [Desktop Entry]
    Version=1.0
    Name=VLC media player
    GenericName=Media player
    Comment=Read, capture, broadcast your multimedia streams
    Name[af]=VLC-mediaspeler
    GenericName[af]=Mediaspeler
    ... 
     More names in different languages
    ...
    Exec=/usr/bin/vlc --started-from-file %U
    TryExec=/usr/bin/vlc
    Icon=vlc
    Terminal=false
    Type=Application
    '''

    entry = {}

    with open(file, 'r') as f:
        lines = f.readlines()

    log.debug(f"Parsing desktop file: {file} with contents {lines}")

    in_desktop_entry_group_header = False
    for line in lines:
        if line[0] == '#':
            log.debug(f"Skipping comment line: {line}")
            continue

        if line == '\n':
            log.debug(f"Skipping empty line: {line}")
            continue

        if line == '[Desktop Entry]\n':
            in_desktop_entry_group_header = True
            log.debug(f"Found desktop entry group header {line}")
            continue

        if line[0] == '[':
            log.debug(f"ignoring unknown group header {line}")
            continue

        if not in_desktop_entry_group_header:
            log.warning(f"Desktop entry group header missing. Line: \'{
                        line}\' \n full file: {lines}")
            log.warning(f"Invalid .deskop file? Path {file}")

        # At this point there should only be key value pairs in the format
        # Key=Value

        # Set max split to 1, to only match first =
        # This is because exec values could include = as part of args
        # i.e Exec=/usr/bin/example --arg1=1 --arg2=2
        data = line.split('=', 1)

        # Remove trailing whitespace
        key = data[0].strip()
        value = data[1].strip()

        entry[key] = value

        match key:
            case "Hidden":
                if value == "true":
                    return {}
                else:
                    entry[key] = value
            case "Path":
                entry[key] = Path(value)

    return entry


result = get_all_desktop_apps(Path("/usr/share/applications"))
