<img width="1766" height="766" alt="image" src="https://github.com/user-attachments/assets/83661a1d-7d13-4353-ab56-2f09b49f6ab4" />

# Current features
- Linux and MacOS app launching 
- Calculator module with homemade parser 
    - Use `=` prefix and type your equation
- Async web searching 
    - Type `!w` to quickly search through wikipedia articles and preview results
- dmenu support with `--dmenu` flag
    - Pass in newline seperated items. Selected item is written to stdout. Useful for scripting

# Requirements
`xdg-open` on linux


# ToDo (ordered)
- [ ] Change default macos font
- [ ] Linux App Icon support [look into](https://docs.rs/crate/icon/latest)
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
