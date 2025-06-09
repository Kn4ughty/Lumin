import os
import glob
from lumin.models.result import Result, Run
import subprocess

# These imports come from the installation process of pyobjc
# https://developer.apple.com/documentation/appkit/nsworkspace
# from AppKit import NSWorkspace
# from Cocoa import NSURL

import gi

gi.require_version("Gtk", "4.0")
gi.require_version("Gdk", "4.0")
from gi.repository import Gtk, Gdk  # noqa: E402


def get_app_file_paths():
    search_dirs = [
        "/Applications",
        os.path.expanduser("~/Applications"),
        "/System/Applications",
    ]
    app_paths = []

    for directory in search_dirs:
        if os.path.exists(directory):
            apps = glob.glob(os.path.join(directory, "**", "*.app"), recursive=True)
            for app in apps:
                app_paths.append(app)

    return app_paths


def open_app_from_url(file_path, *argv, **kwargs):
    try:
        # Use the 'open' command on macOS to open the app
        subprocess.run(["open", file_path], check=True)
        print(f"App at {file_path} opened successfully.")
    except subprocess.CalledProcessError as e:
        print(f"Error opening the app: {e}")


def get_Result_from_path(app_path: str):
    name = os.path.basename(app_path)[:-4]

    # callable = lambda x: subprocess.run(["open", app_path])
    def callable(*argv, **kwargs):
        subprocess.run(["open", app_path])
        exit()

    return Result(display_str=name, open_action=callable)


# Yk what, this is tooooo hard.
# I need to convert from MacOS bytes to python bytes
# Then to GlibBytes then to a GdkPixbuf then to an image.
# Macos users dont get icons.


# Get and print the list of installed apps
apps = get_app_file_paths()
for app in apps:
    print(app)
