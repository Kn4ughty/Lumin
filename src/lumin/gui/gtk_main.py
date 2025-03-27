from pathlib import Path
from typing import Callable

import gi

gi.require_version("Gtk", "4.0")
gi.require_version("Gdk", "4.0")
from gi.repository import Gtk, Gdk  # noqa: E402


class MyApp(Gtk.Application):
    def __init__(self, search_changed: Callable, search_activated: Callable):
        super().__init__(application_id="com.example.Gtk4App")
        self.search_changed = search_changed
        self.search_activated = search_activated

    def do_activate(self):
        # Create application window
        self.window = Gtk.ApplicationWindow(application=self)
        self.window.set_title("GTK4 Example App")
        self.window.set_default_size(800, 400)
        self.window.set_decorated(False)  # Disable window decorations
        self.window.set_resizable(False)  # Disable resizing

        """ Overview of the widget heirarchy
        ________________________lord box_______________________
        | ______________________search_box___________________ |
        | | ----------search entry------------------------  | |
        | |_________________________________________________| |
        | _______________________results box_________________ |
        | |      This box is controled by the search module | |
        | |                                                 | |
        | |_________________________________________________| |
        |_____________________________________________________|
        """

        # Load CSS
        theme_path = Path(__file__).parent / "themes" / "dist" / "default.css"
        self.load_css(theme_path)

        # The woke parade said calling things master was bad
        # Contains the sub-elements, search_entry, and results
        self.lord_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=0)
        self.lord_box.set_margin_bottom(0)
        self.window.set_child(self.lord_box)

        # Box containing text entry and submit button
        self.search_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=0)
        self.search_box.set_margin_top(20)
        self.search_box.set_margin_bottom(0)
        self.search_box.set_margin_start(20)
        self.search_box.set_margin_end(20)
        self.lord_box.append(self.search_box)

        # Add search entry box
        self.search_entry = Gtk.Entry()
        self.search_entry.set_placeholder_text("search")
        self.search_entry.set_hexpand(True)  # Makes it take up correct space
        self.search_entry.connect("activate", self.search_activated)  # enter key
        self.search_entry.connect("changed", self.search_changed)  # any change

        self.search_box.append(self.search_entry)

        self.results_box = Gtk.Box()
        self.lord_box.append(self.results_box)

        # Set the layout into the window
        self.window.show()

    def load_css(self, css_path):
        if not css_path.exists():
            print(f"CSS file not found: {css_path}")
            return

        css_provider = Gtk.CssProvider()
        with open(css_path, "rb") as css_file:
            css_provider.load_from_data(css_file.read())
        Gtk.StyleContext.add_provider_for_display(
            Gdk.Display.get_default(),
            css_provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION,
        )

    def update_results(self, new_results: Gtk.Box):

        self.lord_box.remove(self.results_box)
        self.results_box = new_results
        self.lord_box.append(self.results_box)


if __name__ == "__main__":
    print("This is UI. Running UI by itself doesn't make sense.")
