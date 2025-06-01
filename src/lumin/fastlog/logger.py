from enum import Enum


class LogLevel(Enum):
    DEBUG = "\033[1;34mDEBUG\033[0m"
    INFO = "INFO\033[0m"
    WARNING = "\033[1;33mWARNING\033[0m"
    ERROR = "\033[0mERROR\033[0m"
    CRITICAL = "\033[0m\033[0mCRITICAL\033[0m"


DEFAULT_LOG_LEVEL = LogLevel.INFO


def _print(level: LogLevel, text: str):
    # parent = traceback.format_stack(limit=4)
    parent = ""
    print(f"{parent}{level.value:<10} | {text}")


def debug(text):
    _print(LogLevel.DEBUG, text)


def info(text):
    _print(LogLevel.INFO, text)


def warning(text):
    _print(LogLevel.WARNING, text)


def error(text):
    _print(LogLevel.ERROR, text)


def critical(text):
    _print(LogLevel.CRITICAL, text)


def perf(description: str, start_time: float):
    import time

    info(f"{description}: {(time.perf_counter() - start_time) * 1000:.3f}ms")


def how_did_i_get_here():
    import traceback

    summary = traceback.StackSummary.extract(traceback.walk_stack(None))
    print("".join(summary.format()))
