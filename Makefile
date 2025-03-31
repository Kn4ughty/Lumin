MAIN_FILE_PATH = src/lumin/main.py

# This is not portable.
# I will fix it later
PYTHONPATH=./build/lib.linux-aarch64-cpython-312/


run:
	python3 $(MAIN_FILE_PATH)

run-debug:
	G_MESSAGES_DEBUG=all GTK_DEBUG=interactive GTK_INSPECTOR_TOOL=all python3 $(MAIN_FILE_PATH)

all: 
	black src/
	npm run css-build
	python setup.py build_ext --inplace

	make run

watch:
	npm run css-watch

test:
	pytest -s

clear-config:
	rm ~/.config/lumin/config.toml

install:
	python -m venv .venv
	source .venv/bin/activate
	pip3 install -r requirements.txt
	
	# Build Calc
	python setup.py build_ext --inplace

	python -m wn download oewn:2024
	# python3 $(MAIN_FILE_PATH) install
