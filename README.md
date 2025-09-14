
# ToDo
- [x] Calculator module
- [ ] Investigate https://github.com/pop-os/cosmic-time
- [ ] Investiage https://austinmreppert.github.io/iced-reference/chapter_3.html
- [ ] Allow for user theming
- [ ] Create a nice default theme
- [ ] Work out how to have async tasks in modules
    - [ ] Leads into web based modules i.e dictionary api

- [ ] Fix bug: with calculator input with single . (period)
```
thread 'main' panicked at src/calculator/mod.rs:130:66:
called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Invalid }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
