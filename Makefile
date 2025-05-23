SHELL = bash
MAIN_FILE_PATH = src/lumin/main.py

ARCH := $(shell uname -m)
PYTHON_VERSION := $(shell python3 -c 'import sys; \
				  print(f"{sys.version_info.major}{sys.version_info.minor}")')
PYTHONPATH := ./build/lib.linux-$(ARCH)-cpython-$(PYTHON_VERSION)/

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
	pip3 install -r requirements.txt
	
	python -m wn download oewn:2024

	make build_cython
	npm install sass
