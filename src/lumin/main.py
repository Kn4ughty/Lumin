import sys
from pathlib import Path

# Add the src/ directory to sys.path
sys.path.append(str(Path(__file__).resolve().parent.parent))

from lumin.gui.gtk_main import GTKApp  # Import the GTK application class

def main():
    app = GTKApp()  # Instantiate the GTK application
    app.run()       # Start the GTK main loop

if __name__ == "__main__":
    main()
