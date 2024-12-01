from pathlib import Path
import gi
gi.require_version("Gtk","3.0")
from gi.repository import Gtk, Gdk  # noqa: E402

class GTKApp:
    def __init__(self):
        # Create a window
        self.window = Gtk.Window(title="Lumin Search")
        self.window.set_default_size(600, 400)

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

        # Add a simple label as a placeholder
        label = Gtk.Label(label="Welcome to Lumin")
        self.window.add(label)
        
        # Connect the close event
        self.window.connect("destroy", Gtk.main_quit)

    def run(self):
        self.window.show_all()  # Show the window
        Gtk.main()    
