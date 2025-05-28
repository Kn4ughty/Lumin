Spotlight search for linux

Written with gtk and python. 


# Installation
Python, (probbaly gtk-devel?)

## Requirements
```
libqalculate
python
cmake
gobject-introspection
gtk4
```
## Additional requirements for wayland users:
```
gtk4-layer-shell
```

You will need libqalculate-devel for the calculator module.
It might be named differently based on your package manager.

Then just run
```sh
make install
```


# TODO
## For beta release

- [x] App icons
- [x] Theming based on file name in config
- [x] Configurable css
- [ ] Configurable settings
    - [ ] Option to limit length of results
    - [ ] Configurable module prefixes
- [ ] Do the ugly workaround for only pressing enter to run
- [ ] Code cleanup (rename things, remove old code comments etc.)


## For later
- [ ] Support [rofi dmenu](https://github.com/davatorium/rofi/wiki/dmenu_specs)
- [ ] Web searching (i.e !w car -> https://en.wikipedia.org/wiki/car)
- [ ] Custom modules
