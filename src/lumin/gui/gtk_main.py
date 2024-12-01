from pathlib import Path
import gi
gi.require_version("Gtk","3.0")
from gi.repository import Gtk, Gdk  # noqa: E402

class GTKApp:
    def __init__(self):
        # Create a window
        self.window = Gtk.Window(title="Lumin Search")
        self.window.set_default_size(600, 200)
        self.window.set_resizable(False)

        # Load css
        css_provider = Gtk.CssProvider()
        theme_path = Path(__file__).parent / "themes" / "dist" / "default.css"
        css_provider.load_from_path(str(theme_path))

        # Apply the CSS to the window
        Gtk.StyleContext.add_provider_for_screen(
            Gdk.Screen.get_default(),
            css_provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        )

        self.box = Gtk.Box(spacing=6)
        self.window.add(self.box)

        # Add a simple label as a placeholder
        self.label = Gtk.Entry()
        self.label.connect("activate", self.on_search_entry)
        

        self.box.pack_start(self.label, True, True, 0)

        self.button = Gtk.Button(label="button!")
        self.button.connect("clicked", self.on_button_clicked)
        self.box.pack_start(self.button, True, True, 0)
        
        # Connect the close event
        self.window.connect("destroy", Gtk.main_quit)

    def run(self):
        self.window.show_all()  # Show the window
        Gtk.main()

    def on_button_clicked(self, widet):
        print("Buttoned!")

    def on_search_entry(self, thing):
        print("Searched!")
        print(f"Search text: {thing.get_text()}")
        print(thing)
