from enum import Enum


class log_level(Enum):
    DEBUG = "\033[1;34mDEBUG\033[0m"
    INFO = "INFO\033[0m"
    WARNING = "\033[1;33mWARNING\033[0m"
    ERROR = "\033[0mERROR\033[0m"
    CRITICAL = "\033[0m\033[0mCRITICAL\033[0m"


DEFAULT_LOG_LEVEL = log_level.INFO


def _print(level: log_level, text: str):
    # parent = traceback.format_stack(limit=4)
    parent = ""
    print(f"{parent}{level.value:<10} | {text}")


def debug(text):
    _print(log_level.DEBUG, text)


def info(text):
    _print(log_level.INFO, text)


def warning(text):
    _print(log_level.WARNING, text)


def error(text):
    _print(log_level.ERROR, text)


def critical(text):
    _print(log_level.CRITICAL, text)


def catch(func):
    def wrapper():
        try:
            func()
        except error as e:  # noqa: E772
            print(f"Woah, error: {e}")

    return wrapper()


def perf(description: str, start_time: float):
    import time

    info(f"{description}: {(time.perf_counter() - start_time) * 1000:.2f}ms")
