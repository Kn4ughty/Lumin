from typing import List
import time
import functools
import shlex

from loguru import logger as log


from .models import DesktopApp
import lumin.globals as G
from .common import get_XDG_DATA_DIRS, str_to_bool

# https://specifications.freedesktop.org/icon-theme-spec/latest/


# TODO Expand this
def dict_to_desktop_app(app: dict) -> DesktopApp:
    """ """

    # Using dict.get with default values to handle empty cases

    try:
        result = DesktopApp(
            name=app["Name"],
            cmd_to_execute=parse_exec_string(app["Exec"]),
            # cmd_to_execute="test",
            terminal=str_to_bool(app.get("Terminal", "False")),
        )
    except KeyError as e:
        log.warning(f"bad dict given. e: {e}, dict: {app}")
        result = DesktopApp(name="BAD KEY", cmd_to_execute="false")

    return result


# https://specifications.freedesktop.org/desktop-entry-spec/latest/exec-variables.html
def parse_exec_string(s: str) -> List[str]:
    # shlex makes this easy, all i need to do is handle percent symbols

    output = ""

    i = 0
    while i < len(s):
        char = s[i]
        match char:
            case "%":
                if s[i + 1] == "%i":
                    # This is supposed to be the icon of the entry?
                    # Seems strange and i dont have icons yet so warning it is
                    log.warning(f"FOUND STRANGE %i. s: {s}")

                if s[i + 1] == "%":
                    output += "%"
                    i += 2
                    continue
                i += 2
                continue
            case _:
                output += char
        i += 1

    return shlex.split(output)


def parse_entry_contents(lines: List[str]) -> dict:
    # Parse a single thing, from [askdd] to end

    entry = {}
    for line in lines:
        if line[0] == "#":
            continue

        if line == "\n":
            continue

        if line[0] == "[":
            continue

        data = line.split("=", 1)

        # Remove any trailing whitespace
        key = data[0].strip()
        value = data[1].strip()

        entry[key] = value
    return entry


def parse_desktop_file_contents(lines: List[str]) -> List[dict]:
    # Split for each entry header.
    # If its an action, and actions are enabled,
    # Add it

    entries = []

    i = 0
    length = len(lines)
    while i < length:
        line = lines[i]
        if line[0] == "[":
            if line.startswith("[Desktop Action"):
                if not G.CONFIG["desktop_actions_enabled"]:
                    i += 1
                    while i < length and not lines[i].startswith("["):
                        i += 1
                    continue
            # Search forward to next action or end of file.
            # Then select that range of the array and parse it in.
            j = i + 1

            while j < length and not lines[j].startswith("["):
                j += 1
            entry = parse_entry_contents(lines[i:j])
            if str_to_bool(entry.get("NoDisplay", "False")):
                i = j + 1
                continue
                # return

            entries.append(entry)

            i = j + 1
        i += 1

    return entries


@functools.cache
def get_all_desktop_apps() -> List[DesktopApp]:
    start_time = time.time()

    dirs = [dir.joinpath("applications") for dir in get_XDG_DATA_DIRS()]

    entries = []

    for search_dir in dirs:
        files = [f for f in search_dir.glob("*.desktop")]

        for file in files:

            with open(file, "r") as f:
                lines = f.readlines()

            ds = parse_desktop_file_contents(lines)
            if ds is not None:
                for d in ds:
                    entries.append(dict_to_desktop_app(d))

    log.info(
        f"Time to parse all .desktop files: {
            (time.time() - start_time) * 1000:.3f}ms"
    )
    # On m1 mac it takes about 9ms. $\pm$ 1ms

    return entries


if __name__ == "__main__":
    result = get_all_desktop_apps()
    print(result[0])
