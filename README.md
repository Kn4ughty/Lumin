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
- [ ] Match style from figma
- [ ] Fix lutris not working. (when running manually, it works only from $HOME)
- [x] Configurable css


## For later
- [ ] Support [rofi dmenu](https://github.com/davatorium/rofi/wiki/dmenu_specs)
- [ ] Web searching (i.e !w car -> https://en.wikipedia.org/wiki/car)
- [ ] Custom modules
