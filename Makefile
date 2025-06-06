SHELL = bash
MAIN_FILE_PATH = src/lumin/main.py

# I dont really like this way of finding the pythonpath, but it works
ARCH := $(shell uname -m)
PYTHON_VERSION := $(shell python3 -c 'import sys; \
				  print(f"{sys.version_info.major}{sys.version_info.minor}")')

ifeq ($(shell uname -s),Darwin)
	# Convert macos version (XX.YY.ZZ) -> (XX.0)
	PLATFORM := macosx-$(shell sw_vers --productVersion | sed "s/\([0-9]*\)\..*/\1.0/")
else
	PLATFORM := linux
endif

PYTHONPATH := ./build/lib.$(PLATFORM)-$(ARCH)-cpython-$(PYTHON_VERSION)/

export PYTHONPATH

all: lint build_css build_cython run

run:
	python3 $(MAIN_FILE_PATH)

run-debug:
	G_MESSAGES_DEBUG=all GTK_DEBUG=interactive GTK_INSPECTOR_TOOL=all python3 $(MAIN_FILE_PATH)

build_css:
	npm run css-build

build_cython:
	python setup.py build_ext --inplace

lint:
	black src/

test:
	pytest -s

clear-config:
	rm ~/.config/lumin/config.toml

install:
	python3 -m venv .venv
	source .venv/bin/activate
	pip3 install -r requirements.txt
	
	python -m wn download oewn:2024

	make build_cython
	npm install sass

	make build_css
	make build_cython
