from pathlib import Path
from typing import Callable

import gi

gi.require_version("Gtk","4.0")
from gi.repository import Gtk, Gdk  # noqa: E402


class GTKApp:
    def __init__(self, on_search_activate: Callable, on_search_text_changed: Callable):
        # Create a window
        self.window = Gtk.Window(title="")
        self.window.set_default_size(600, 200)
        self.window.set_resizable(False)
        self.window.set_decorated(False)
        self.window.set_titlebar()

        # Load css
        css_provider = Gtk.CssProvider()
        theme_path = Path(__file__).parent / "themes" / "dist" / "default.css"
        css_provider.load_from_path(str(theme_path))

        # Apply the CSS to the window
        Gtk.StyleContext.add_provider_for_display(
            Gdk.Display.get_default(),
            css_provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        )

        self.box = Gtk.Box(spacing=6)
        self.window.set_child(self.box)

        # Add a simple label as a placeholder
        self.label = Gtk.Entry()
        self.label.connect("activate", on_search_activate)
        
        self.label.connect("changed", on_search_text_changed)

        self.box.append(self.label)

        self.button = Gtk.Button(label="button!")
        self.button.connect("clicked", self.on_button_clicked)
        self.box.append(self.button)
        
        # Connect the close event
        # self.window.connect("destroy", Gtk.main_quit)

    def run(self):
        self.window.present()  # Show the window
        # Gtk.main()
        pass

    def on_button_clicked(self, widet):
        print("Buttoned!")
