# Lumin
<img width="1766" height="766" alt="image" src="https://github.com/user-attachments/assets/83661a1d-7d13-4353-ab56-2f09b49f6ab4" />

[![Tests](https://github.com/Kn4ughty/Lumin/actions/workflows/rust.yml/badge.svg)](https://github.com/Kn4ughty/Lumin/actions/workflows/rust.yml)

# Features and Useage
- Linux and MacOS app launching with async icon lookup
- Calculator module with homemade parser 
    - Use `=` prefix and type your equation
- Async web searching 
    - Type `!w` to quickly search through wikipedia articles and preview results
- dmenu support with `--dmenu` flag
    - Pass in newline seperated items. Selected item is written to stdout. Useful for scripting

# Installation

Note for linux users: the `xdg-open` command is required for opening URL's in the default browser.

```sh
git clone "https://github.com/Kn4ughty/Lumin"
cd Lumin
cargo build --release
```
I reccomend adding to path by symlinking to the release.
```sh
ls -s target/release/lumin ~/bin/
```
Then once it is in your path just run `lumin`


# ToDo
- [ ] Change default macos font
- [ ] Allow for user theming
- [ ] Create a nice default theme
- [ ] Templates for UI elements 
    - [ ] Investiage https://austinmreppert.github.io/iced-reference/chapter_3.html
    - [x] ListBox and List elements
    - [ ] Investigate https://github.com/pop-os/cosmic-time
- [ ] Dictionary module
- [ ] Default screen gives small usage guide
- [ ] Refactor calculator
- [ ] Reduce dependencies
