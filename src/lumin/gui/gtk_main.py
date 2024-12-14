from pathlib import Path
from typing import Callable

import gi

gi.require_version("Gtk","4.0")
from gi.repository import Gtk, Gdk  # noqa: E402


class MyApp(Gtk.Application):
    def __init__(self,  search_changed: Callable, search_activated: Callable):
        super().__init__(application_id="com.example.Gtk4App")
        self.search_changed = search_changed
        self.search_activated = search_activated

    def do_activate(self):
        # Create application window
        window = Gtk.ApplicationWindow(application=self)
        window.set_title("GTK4 Example App")
        window.set_default_size(400, 100)
        window.set_decorated(False)  # Disable window decorations
        window.set_resizable(False)  # Disable resizing

        # Load CSS
        theme_path = Path(__file__).parent / "themes" / "dist" / "default.css"
        self.load_css(theme_path)
        
        # The woke parade said calling things master was bad
        # Contains the sub-elements, search_entry, and results
        lord_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=0)
        lord_box.set_margin_bottom(0)
        window.set_child(lord_box)

        # Box containing text entry and submit button
        search_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=0)
        search_box.set_margin_top(20)
        search_box.set_margin_bottom(0)
        search_box.set_margin_start(20)
        search_box.set_margin_end(20)
        lord_box.append(search_box)

        # Add search entry box
        search_entry = Gtk.Entry()
        search_entry.set_placeholder_text("search")
        search_entry.set_hexpand(True) # Makes it take up correct space
        search_entry.connect("activate", self.search_activated) # enter key
        search_entry.connect("changed", self.search_changed) # any change
        search_box.append(search_entry)

        # TODO Results UI

        # Set the layout into the window
        window.show()

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


if __name__ == "__main__":
    print("This is UI. Running UI by itself doesn't make sense.")
