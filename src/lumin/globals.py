from pathlib import Path
import shlex
import subprocess
import tomllib
import os
import sys
from lumin.fastlog import logger as log

APP_NAME = "Lumin"
CONFIG_DIR = Path(f"~/.config/{APP_NAME.lower()}/").expanduser()

MAIN_CONFIG_NAME = "config.toml"
MAIN_CONFIG_PATH = CONFIG_DIR.joinpath(MAIN_CONFIG_NAME)

CSS_PATH = CONFIG_DIR.joinpath("index.css")

DATA_DIR = Path(f"~/.local/share/{APP_NAME.lower()}/").expanduser()
if not os.path.exists(DATA_DIR):
    os.mkdir(DATA_DIR)

# Potentially load this from a file. default_config.toml or smth
default_config = {
    "theme_file_location": "~/.config/lumin/index.css",
    "desktop_actions_enabled": False,
}

if not os.path.exists(CONFIG_DIR):
    os.mkdir(CONFIG_DIR)
if not os.path.exists(MAIN_CONFIG_PATH):
    log.warning("Main config doesnt exist.")

    f = open(MAIN_CONFIG_PATH, "w")
    tomllib.dump(default_config, f)
    f.close()

with open(MAIN_CONFIG_PATH, "rb") as f:
    file_config = tomllib.load(f)

joined_config = {}
for key in default_config:
    if file_config.get(key, None) is not None:
        joined_config[key] = file_config.get(key)
    else:
        joined_config[key] = default_config.get(key)


def str_to_bool(s: str) -> bool:
    match s.lower():
        case "false":
            return False
        case "true":
            return True
        case _:
            log.warning(f"Str to bool was given bad data. s: {s}")
            return False


THEME_FILE_LOCATION = Path(joined_config["theme_file_location"]).expanduser()
DESKTOP_ACTIONS_ENABLED = joined_config["desktop_actions_enabled"]

PLATFORM_OS = sys.platform
IS_WAYLAND = False
if PLATFORM_OS == "linux":
    # Thanks stackoverflow
    # https://unix.stackexchange.com/questions/202891/how-to-know-whether-wayland-or-x11-is-being-used
    command = "bash -c \"loginctl show-session $(awk '/tty/ {print $1}' <(loginctl)) -p Type | awk -F= '{print $2}'\""

    output = subprocess.check_output(shlex.split(command)).decode("utf-8").strip()
    if output == "wayland":
        IS_WAYLAND = True


awful_input_global = ""
