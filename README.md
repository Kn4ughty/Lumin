# Lumin
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
- dmenu support with `--dmenu` flag
    - Pass in newline seperated items. Selected item is written to stdout. Useful for scripting
    - If no lines of items are sent, and an EOF is sent, outputs the input text.
- Configurable via `~/.config/lumin/config.toml`
    - All options documented via code comments
    - Resilient to bad config (uses default config in case of error)

# Installation

Note for linux users: the `xdg-open` command is required for opening URL's in the default browser.

```sh
git clone "https://github.com/Kn4ughty/Lumin"
cd Lumin
cargo build --release
```
I reccomend adding to path by symlinking to the release.
```sh
ln -s target/release/lumin ~/bin/
```
Then once it is in your path just run `lumin`


<!-- Hidden since it is for me and not users -->
<!-- # ToDo -->
<!-- - [ ] Change default macos font -->
<!-- - [ ] Investigate https://austinmreppert.github.io/iced-reference/chapter_3.html -->
<!-- - [ ] Investigate https://github.com/pop-os/cosmic-time -->
<!-- - [ ] Refactor calculator -->
<!-- - [ ] Reduce dependencies -->
