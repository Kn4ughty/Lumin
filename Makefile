MAIN_FILE_PATH = src/lumin/main.py



run:
	python3 $(MAIN_FILE_PATH)

run-debug:
	G_MESSAGES_DEBUG=all GTK_DEBUG=interactive GTK_INSPECTOR_TOOL=all python3 $(MAIN_FILE_PATH)

all: 
	npm run css-build

watch:
	npm run css-watch
