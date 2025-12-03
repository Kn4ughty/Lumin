# Lumin
> A fast module based launcher for Linux and Macos
<img width="1760" height="804" alt="image" src="https://github.com/user-attachments/assets/9e9f1c89-04ac-4928-9131-5c564a55addc" />

[![Tests](https://github.com/Kn4ughty/Lumin/actions/workflows/rust.yml/badge.svg)](https://github.com/Kn4ughty/Lumin/actions/workflows/rust.yml)

# Features and Usage
- Linux and MacOS app launching with async icon lookup using greenthreads.
    - Loads all icons in **2 frames** from input, competing launcher [rofi](https://github.com/davatorium/rofi) takes **6**
- Calculator module with homemade parser 
    - Use `=` prefix and type your equation
- Async web searching 
    - Type `!w` to quickly search through wikipedia articles and preview results
    - Type `!d` to search and use an online dictionary API.
- Search through file names with `'`
    - Searches through files on seperate thread to keep ui responsive
    - Just activate the item to open it in the default app
- dmenu support with `--dmenu` flag
    - Pass in newline seperated items. Selected item is written to stdout. Useful for scripting
    - If only an EOF is sent and no lines, outputs the input text to stdout.
- Configurable via `~/.config/lumin/config.toml`
    - All options documented via code comments

# Installation

Note for linux users: the `xdg-open` command is required for opening URL's in the default browser. It should probably be installed by default but if nothing is launching, it not existing is the most likley reason why.

First [install rust](https://rust-lang.org/tools/install/), then run: 
```sh
cargo install --git https://github.com/Kn4ughty/Lumin/
```
To do easy updates of binaries installed with cargo, install [cargo update](https://github.com/nabijaczleweli/cargo-update):
```sh
cargo install cargo-update
# And to do the updates: 
cargo install-update -a
```

<!-- Hidden since it is for me and not users -->
<!-- # ToDo -->
<!-- - [ ] Change default macos font -->
<!-- - [ ] Support multiple locales -->
<!-- - [ ] Investigate https://austinmreppert.github.io/iced-reference/chapter_3.html -->
<!-- - [ ] Investigate https://github.com/pop-os/cosmic-time -->
<!-- - [ ] Refactor calculator -->
<!-- - [ ] Reduce dependencies -->
