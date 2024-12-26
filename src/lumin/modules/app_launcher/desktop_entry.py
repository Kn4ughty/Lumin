from dataclasses import dataclass, field
from pathlib import Path
from typing import List

import logging as log

# TODO. Work this out
# https://specifications.freedesktop.org/icon-theme-spec/latest/
class AppIcon():
    pass

# I have ignored keys that are not applicable
# Will need refactoring for multi language support later
@dataclass
class DesktopEntry():
    exec: str # techinally not mandatory in spec but in practice it is
    name: str
    try_exec: str = ""
    path: Path = Path("") # Working dir for app
    generic_name: str = ""
    comment: str = ""
    icon: AppIcon = AppIcon() # Stupid line
    terminal: bool = False
    keywords: List[str] = field(default_factory=list)


def get_all_desktop_apps(search_dir: Path) -> List[DesktopEntry]:

    # Get all .destop files in search_dir
    files = [f for f in search_dir.glob("*.desktop")]

    for file in files:
        parse_desktop_file(file)

    
    return [DesktopEntry(Path(""), "Name")]


def parse_desktop_file(file: Path) -> DesktopEntry:
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

    entry = DesktopEntry(Path(), "Name")

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
            log.warning(f"Desktop entry group header missing. Line: \'{line}\' \n full file: {lines}")
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

        # match key:
        #     case "Name":
        #         entry.name = value
        #     case "Exec":
        #         entry.exec = value
        #     case "TryExec":
        #         entry.try_exec = value
        #     case "Path":
        #         entry.path = Path(value)
        #     case "GenericName":
        #         entry.generic_name = value
        #     case "Comment":
        #         entry.comment = value








    return DesktopEntry(Path(), "Name")

import time

start_time = time.time()
get_all_desktop_apps(Path("/usr/share/applications"))
print(time.time() - start_time)

# parse_desktop_file(Path("/usr/share/applications/btop.desktop"))
