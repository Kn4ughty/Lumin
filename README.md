# Spotlight search for linux

For people who want dynamic searching using prefixes
## Current modules
- App searching
- Calculator (start search with `/`)
- Dictionary (start search with `!d`)

# Installation

## Requirements

Please note that this list is not exhaustive.
You might need more, just pay attention to the output of pip as its installing.

### MacOS

install brew (link me)

```
brew install git python cmake pkgconf cairo gobject-introspection libqalculate npm gtk4
```

You have an additional pip requirement so after following the other installation steps run:
```
pip3 install pyobjc
```
This package provides python bindings to MacOS objectice C libraries.


### Linux
(Non exhaustive, just pay attention to pips output)
```
npm
libqalculate-devel
python
cmake
gobject-introspection
gtk4
```
### Additional requirements for wayland users:
```
gtk4-layer-shell
```

## Final step

Run this while in the project root directory:
```sh
python3 -m venv .venv
source .venv/bin/activate
make install
```


# TODO
## For beta release

- [x] App icons
- [x] Theming based on file name in config
- [x] Configurable css
- [ ] Handle case where .config doesnt exist
- [ ] Configurable settings
    - [ ] Configurable module prefixes
    - [ ] gtk4-layer-shell toggle
- [ ] Do the ugly workaround for only pressing enter to run
- [ ] Code cleanup (rename things, remove old code comments etc.)
- [ ] Copy result of math module to clipbard


## For later
- [ ] Support [rofi dmenu](https://github.com/davatorium/rofi/wiki/dmenu_specs)
- [ ] Web searching (i.e !w car -> https://en.wikipedia.org/wiki/car)
- [ ] Probably do an entire re-write of the program in a better language.
