

all: 
	npm run css-build
	python3 src/lumin/main.py

watch:
	npm run css-watch
