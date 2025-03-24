from pathlib import Path
import tomllib
import os
from loguru import logger as log

APP_NAME = "Lumin"
CONFIG_DIR = Path(f"~/.config/{APP_NAME.lower()}/").expanduser()
MAIN_CONFIG_NAME = "config.toml"
MAIN_CONFIG_PATH = CONFIG_DIR.joinpath(MAIN_CONFIG_NAME)

# TODO. Potentially load this from a file. default_config.toml or smth
default_config = """
desktop_actions_enabled = false
"""

if not os.path.exists(CONFIG_DIR):
    os.mkdir(CONFIG_DIR)
if not os.path.exists(MAIN_CONFIG_PATH):
    log.warning("Main config doesnt exist.")

    f = open(MAIN_CONFIG_PATH, "w")
    f.write(default_config)
    f.close()


with open(MAIN_CONFIG_PATH, "rb") as f:
    data = tomllib.load(f)

CONFIG_DICT = data
