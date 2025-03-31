from pathlib import Path
import tomllib
import os
from loguru import logger as log

APP_NAME = "Lumin"
CONFIG_DIR = Path(f"~/.config/{APP_NAME.lower()}/").expanduser()

MAIN_CONFIG_NAME = "config.toml"
MAIN_CONFIG_PATH = CONFIG_DIR.joinpath(MAIN_CONFIG_NAME)

CSS_PATH = CONFIG_DIR.joinpath("index.css")

DATA_DIR = Path(f"~/.local/share/{APP_NAME.lower()}/").expanduser()
if not os.path.exists(DATA_DIR):
    os.mkdir(DATA_DIR)

# TODO. Potentially load this from a file. default_config.toml or smth
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
    CONFIG = tomllib.load(f)
