MAIN_FILE_PATH = src/lumin/main.py



run:
	black src/
	python3 $(MAIN_FILE_PATH)

run-debug:
	black src/
	G_MESSAGES_DEBUG=all GTK_DEBUG=interactive GTK_INSPECTOR_TOOL=all python3 $(MAIN_FILE_PATH)

all: 
	black src/
	npm run css-build

watch:
	npm run css-watch
