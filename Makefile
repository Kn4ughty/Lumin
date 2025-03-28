MAIN_FILE_PATH = src/lumin/main.py



run:
	python3 $(MAIN_FILE_PATH)

run-debug:
	black src/
	G_MESSAGES_DEBUG=all GTK_DEBUG=interactive GTK_INSPECTOR_TOOL=all python3 $(MAIN_FILE_PATH)

all: 
	black src/
	npm run css-build
	make run

watch:
	npm run css-watch

test:
	pytest -s

clear-config:
	rm ~/.config/lumin/config.toml

install:
	python -m wn download oewn:2024
	python3 $(MAIN_FILE_PATH) install
