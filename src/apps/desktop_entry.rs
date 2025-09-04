// https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html
// 2hr
//
// #![allow(dead_code)]

use ini::Ini;
use std::vec::Vec;

pub struct Action {
    action_identifier: String,
    name: String,
    icon: Option<String>,
    exec: Option<String>,
}

enum EntryType {
    Application,
    Link,
    Directory,
}

// For Vec<String>, maybe just leave as 0 length vector?
pub struct DesktopEntry {
    entry_type: EntryType,
    version: Option<String>,
    name: String,
    generic_name: Option<String>,
    // No display not included since its irrelevant. Should be handled in parsing
    comment: Option<String>,
    icon_string: Option<String>, // https://specifications.freedesktop.org/icon-theme-spec/latest/
    // Handle files with Hidden at parsing level
    only_show_in: Vec<String>,
    not_show_in: Vec<String>,
    // I do not support dbus activation idk what that is
    exec: String, // Techicially optional, nuh uh.
    path: Option<String>,
    terminal: Option<String>,
    action_list: Vec<Action>,
    // mime_types: Option<
    categories: Vec<String>,
    // No impliments
    keywords: Vec<String>,
    url: Option<String>,
    single_main_window: Option<bool>,
}

impl std::default::Default for DesktopEntry {
    fn default() -> DesktopEntry {
        DesktopEntry {
            entry_type: EntryType::Application,
            version: None,
            name: "".to_string(),
            generic_name: None,
            comment: None,
            icon_string: None,
            only_show_in: Vec::new(),
            not_show_in: Vec::new(),
            exec: "".to_string(),
            path: None,
            terminal: None,
            action_list: Vec::new(),
            categories: Vec::new(),
            keywords: Vec::new(),
            url: None,
            single_main_window: None,
        }
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidKey,
    MissingRequiredField,
    BadGroupHeader,
    CouldNotLoadFile,
    DesktopEntryHeaderNotFound,
    UnknownApplicationType,
    NoDisplayTrue,
}

fn parse_from_file(file_path: &std::path::Path) -> Result<DesktopEntry, ParseError> {
    let Ok(entry) = Ini::load_from_file(file_path) else {
        return Err(ParseError::CouldNotLoadFile);
    };
    return parse_from_ini(entry);
}

fn parse_from_ini(input: Ini) -> Result<DesktopEntry, ParseError> {
    // let mut entry = DesktopEntry::default();

    let Some(entry_keys) = input.section(Some("Desktop Entry")) else {
        return Err(ParseError::DesktopEntryHeaderNotFound);
    };

    if matches!(entry_keys.get("NoDisplay"), Some("true"))
        || matches!(entry_keys.get("Hidden"), Some("true"))
    {
        return Err(ParseError::NoDisplayTrue);
    }

    let entry = DesktopEntry {
        // use matches! here.
        entry_type: match entry_keys.get("Type") {
            Some("Application") => EntryType::Application,
            Some("Link") => EntryType::Link,
            Some("Directory") => EntryType::Directory,
            Some(_) => return Err(ParseError::UnknownApplicationType),
            None => return Err(ParseError::MissingRequiredField),
        },
        version: entry_keys.get("Version").map(|s| s.to_string()),
        name: entry_keys
            .get("Name")
            .ok_or(ParseError::MissingRequiredField)?
            .to_string(),
        generic_name: entry_keys.get("GenericName").map(|s| s.to_string()),
        comment: entry_keys.get("Comment").map(|s| s.to_string()),
        icon_string: entry_keys.get("Icon").map(|s| s.to_string()),
        only_show_in: parse_string_list(entry_keys.get("OnlyShowIn")),
        ..Default::default()
    };

    println!("hello");
    println!("{input:#?}");

    Ok(entry)
}

fn parse_string_list(input: Option<&str>) -> Vec<String> {
    // input is like blah;thing2;thing3
    let mut result = Vec::new();
    let mut current = String::new();
    let mut chars = input.unwrap_or("").chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ';' => {
                if current.ends_with("\\") {
                    current.pop();
                    current.push(c);
                } else {
                    result.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    return result;
}

#[test]
fn can_parse_string_list() {
    let input = Some("t1;t2;t\\;3");
    let output = parse_string_list(input);
    println!("{output:#?}");
    debug_assert!(output == vec!["t1".to_string(), "t2".to_string(), "t;3".to_string()])
}

fn parse_bool(s: &str) -> Option<bool> {
    match s {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}


#[test]
fn can_parse_file() {
    let test = r#"
[Desktop Entry]
Type=Application
TryExec=alacritty
Exec=alacritty
Icon=Alacritty
Terminal=false
Categories=System;TerminalEmulator;

Name=Alacritty
GenericName=Terminal
Comment=A fast, cross-platform, OpenGL terminal emulator
StartupNotify=true
StartupWMClass=Alacritty
Actions=New;
X-Desktop-File-Install-Version=0.28

[Desktop Action New]
Name=New Terminal
Exec=alacritty
    "#;
    // a

    parse_from_ini(Ini::load_from_str(test).unwrap()).unwrap();
}

#[test]
fn ignores_no_display_entries() {
    let test = r#"
        [Desktop Entry]
        Type=Application
        TryExec=alacritty
        Exec=alacritty
        Icon=Alacritty
        Terminal=false
        Categories=System;TerminalEmulator;
    "#;
}
