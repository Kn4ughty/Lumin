from pathlib import Path
import shlex
import subprocess
import tomllib
import os
import sys
from lumin.fastlog import logger as log

# TODO. This needs serious cleanup

APP_NAME = "Lumin"
CONFIG_DIR = Path(f"~/.config/{APP_NAME.lower()}/").expanduser()

MAIN_CONFIG_NAME = "config.toml"
MAIN_CONFIG_PATH = CONFIG_DIR.joinpath(MAIN_CONFIG_NAME)

SEARCH_DATA_LOG_FILE = CONFIG_DIR.joinpath("search_data.csv")

CSS_PATH = CONFIG_DIR.joinpath("index.css")

DATA_DIR = Path(f"~/.local/share/{APP_NAME.lower()}/").expanduser()
if not os.path.exists(DATA_DIR):
    os.mkdir(DATA_DIR)

# Potentially load this from a file. default_config.toml or smth
default_config = {
    "theme_file_location": "~/.config/lumin/index.css",
    "desktop_actions_enabled": False,
    "search_logging_enabled": True,
    'prefixes': {
        'dict': [';d'],
        'calc': [';c', '/']
    }
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
log.info(f"config loaded form file {file_config}")


def str_to_bool(s: str) -> bool:
    match s.lower():
        case "false":
            return False
        case "true":
            return True
        case _:
            return None


joined_config = {}
for key in default_config:
    if file_config.get(key, None) is not None:
        joined_config[key] = file_config.get(key)
    else:
        joined_config[key] = default_config.get(key)


THEME_FILE_LOCATION: Path = Path(joined_config["theme_file_location"]).expanduser()
SHOW_DESKTOP_ACTIONS: bool = joined_config["desktop_actions_enabled"]
DO_SEARCH_FREQUENCY_LOGGING: bool = joined_config["search_logging_enabled"]

PLATFORM_OS = sys.platform
IS_WAYLAND = False
if PLATFORM_OS == "linux":
    # Thanks stackoverflow
    # https://unix.stackexchange.com/questions/202891/how-to-know-whether-wayland-or-x11-is-being-used
    command = "bash -c \"loginctl show-session $(awk '/tty/ {print $1}' <(loginctl)) -p Type | awk -F= '{print $2}'\""

    output = subprocess.check_output(shlex.split(command)).decode("utf-8").strip()
    if output == "wayland":
        IS_WAYLAND = True

log.info("ermmm what")
search_input_global = ""
