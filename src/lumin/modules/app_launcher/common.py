from pathlib import Path
from typing import List
from loguru import logger as log
import os


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
