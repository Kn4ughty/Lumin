// https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html
// 6hr

use ini::Ini;
use std::vec::Vec;
use walkdir::WalkDir;

use log;

#[derive(Debug, PartialEq)]
pub struct Action {
    pub name: String,
    pub icon_path: Option<String>,
    pub exec: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum EntryType {
    Application,
    Link,
    Directory,
}

#[derive(Debug)]
pub struct DesktopEntry {
    pub entry_type: EntryType,
    version: Option<String>,
    pub name: String,
    pub generic_name: Option<String>,
    // No display not included since its irrelevant. Should be handled in parsing
    pub comment: Option<String>,
    // pub icon_path: Option<std::path::PathBuf>, // https://specifications.freedesktop.org/icon-theme-spec/latest/
    pub icon_path: Option<String>, // https://specifications.freedesktop.org/icon-theme-spec/latest/
    // Handle files with Hidden at parsing level
    only_show_in: Vec<String>,
    not_show_in: Vec<String>,
    // I do not support dbus activation idk what that is
    try_exec: Option<String>,
    pub exec: String, // Techicially optional, nuh uh.
    pub working_dir: Option<String>,
    pub terminal: bool,
    pub action_list: Vec<Action>,
    // mime_types: Option<
    categories: Vec<String>,
    // No impliments
    pub keywords: Vec<String>,
    url: Option<String>,
}

impl std::default::Default for DesktopEntry {
    fn default() -> DesktopEntry {
        DesktopEntry {
            entry_type: EntryType::Application,
            version: None,
            name: "".to_string(),
            generic_name: None,
            comment: None,
            icon_path: None,
            only_show_in: Vec::new(),
            not_show_in: Vec::new(),
            try_exec: None,
            exec: "".to_string(),
            working_dir: None,
            terminal: false,
            action_list: Vec::new(),
            categories: Vec::new(),
            keywords: Vec::new(),
            url: None,
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    MissingRequiredField,
    BadGroupHeader,
    CouldNotLoadFile,
    DesktopEntryHeaderNotFound,
    UnknownApplicationType,
    NoDisplayTrue,
    ActionMissingName,
    MissingDataDirsEnvVar,
    FailedToReadDataDirs,
}

pub fn load_desktop_entries() -> Result<Vec<DesktopEntry>, ParseError> {
    let mut entries = Vec::new();
    let Ok(raw_data_dirs) = std::env::var("XDG_DATA_DIRS") else {
        return Err(ParseError::MissingDataDirsEnvVar);
    };
    log::trace!("raw data dirs = {raw_data_dirs}");
    for dir in raw_data_dirs.split(":") {
        for entry in WalkDir::new(dir.to_owned() + "/applications/")
            .into_iter()
            .filter_map(|e| e.ok())
        {
            log::trace!("{}", entry.path().display());
            entries.push(parse_from_file(entry.path()).map_err(|e| {
                log::trace!("error parsing file {:#?} with error: {:?}", entry.path(), e)
            }));
        }
    }

    // panic!();
    Ok(entries.into_iter().filter_map(|a| a.ok()).collect())
}

#[cfg(unix)]
#[test]
fn can_load_system_desktop_entries() {
    let r = load_desktop_entries();
    assert!(r.is_ok());
    let r = r.unwrap();
    println!("{r:#?}");
    assert_ne!(r.len(), 0); // this might break if i use github CI
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

    let entry_keys: &ini::Properties = entry_keys;

    let entry_type = match entry_keys.get("Type") {
        Some("Application") => EntryType::Application,
        Some("Link") => EntryType::Link,
        Some("Directory") => EntryType::Directory,
        Some(_) => return Err(ParseError::UnknownApplicationType),
        None => return Err(ParseError::MissingRequiredField),
    };

    let entry = DesktopEntry {
        entry_type,
        version: entry_keys.get("Version").map(|s| s.to_string()),
        name: entry_keys // TODO. handle different languages
            .get("Name")
            .ok_or(ParseError::MissingRequiredField)?
            .to_string(),
        try_exec: entry_keys.get("TryExec").map(|s| s.to_string()),
        exec: parse_exec_key(
            entry_keys
                .get("Exec")
                .ok_or(ParseError::MissingRequiredField)?,
            entry_keys.get("Icon"),
            entry_keys.get("Name"),
        ),
        generic_name: entry_keys.get("GenericName").map(|s| s.to_string()),
        comment: entry_keys.get("Comment").map(|s| s.to_string()),
        icon_path: entry_keys.get("Icon").map(|s| s.to_string()),
        only_show_in: parse_string_list(entry_keys.get("OnlyShowIn")),
        not_show_in: parse_string_list(entry_keys.get("NotShowIn")),
        working_dir: entry_keys.get("Path").map(|s| s.to_string()),
        terminal: entry_keys.get("Terminal").map_or(false, |b| b == "true"),
        categories: parse_string_list(entry_keys.get("Categories")),
        keywords: parse_string_list(entry_keys.get("Keywords")),
        url: match entry_keys.get("URL") {
            Some(s) => Some(s.to_string()),
            _ if entry_type == EntryType::Link => return Err(ParseError::MissingRequiredField),
            _ => None,
        },
        action_list: parse_string_list(entry_keys.get("Actions")) // i dont like this whole thing
            .into_iter()
            .map(|name: String| {
                let section: &ini::Properties = input
                    .section(Some(format!("Desktop Action {}", name)))
                    .ok_or(ParseError::BadGroupHeader)?;
                return Ok::<Action, ParseError>(Action {
                    name: section
                        .get("Name")
                        .ok_or(ParseError::ActionMissingName)?
                        .to_string(),
                    exec: section.get("Exec").map(|s| s.to_string()),
                    icon_path: section.get("Icon").map(|s| s.to_string()),
                });
            })
            .filter_map(|a| {
                if a.is_ok() {
                    a.ok()
                } else {
                    log::warn!("Action was invalid {:#?}", a.err());
                    None
                }
            })
            .collect(),
    };

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
    result
}

#[test]
fn can_parse_string_list() {
    let input = Some("t1;t2;t\\;3;");
    let output = parse_string_list(input);
    println!("{output:#?}");
    debug_assert!(output == vec!["t1".to_string(), "t2".to_string(), "t;3".to_string()])
}

fn parse_exec_key(input: &str, icon: Option<&str>, name: Option<&str>) -> String {
    // https://specifications.freedesktop.org/desktop-entry-spec/latest/exec-variables.html
    // maybe send to ai
    let mut escaped_result = "".to_string();
    let mut chars = input.chars().peekable();

    // Lots of nesting here..
    while let Some(c) = chars.next() {
        match c {
            // literal \
            '\\' => {
                let Some(n) = chars.next() else {
                    log::warn!("No character after backslash in escape sequence {}", input);
                    continue;
                };

                match n {
                    // Quoting must be done by enclosing the argument between double quotes
                    // and escaping the double quote character , ("`"), ("$"), ("\") by preceding it with an additional backslash character
                    '\\' | '`' | '"' | '$' | '%' => escaped_result.push(n),

                    s => log::info!("unknown escape sequence \\{}. Ignoring.", s),
                }
            }
            // a literal % is escaped as %%
            '%' => {
                let Some(n) = chars.next() else {
                    log::warn!("No character after percentage in escape sequence {}", input);
                    continue;
                };
                match n {
                    '%' => escaped_result.push(n),
                    'i' => {
                        if let Some(icon) = icon {
                            log::trace!(
                                "Hit funny %i (sub in icon) escape sequence for app with icon {icon}"
                            );
                            escaped_result += format!("--icon {icon}").as_str();
                        };
                    }
                    'c' => {
                        if let Some(name) = name {
                            log::trace!(
                                "Hit funny %c (sub in name) escape sequence for app with name {name}"
                            );
                            escaped_result += name;
                        }
                    }
                    _ => {} // do nothing
                }
            }
            _ => escaped_result.push(c),
        }
    }

    escaped_result
}

#[test]
fn can_parse_exec_key() {
    assert_eq!(parse_exec_key(r#"\\"#, None, None), "\\".to_string());
    assert_eq!(parse_exec_key(r#"\`"#, None, None), "`".to_string());
    assert_eq!(parse_exec_key(r#"\%"#, None, None), "%".to_string());
    // Test % escaping
    assert_eq!(
        parse_exec_key(r#"%i"#, Some("Sylvan Franklin"), None),
        "--icon Sylvan Franklin".to_string()
    );
    assert_eq!(
        parse_exec_key(r#"%c"#, None, Some("name")),
        "name".to_string()
    );
}

fn parse_bool(s: &str) -> Option<bool> {
    match s {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

// fn find_icon(name: &str, size: i32, scale: i32) -> std::path::PathBuf {
    // gsettings get org.gnome.desktop.interface gtk-theme
    // let user_theme = std::process::Command::new("gsettings")
    //     .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
    //     .spawn()
    //     .or_else(|e| {
    //         log::warn!("failed ");
    //     });

    // filename = find_icon_helper(name, size, scale, user_theme);
    // if filename.
// }

// fn find_icon_helper(name: &str, size: i32, scale: i32, user_theme: &str) -> std::path::PathBuf {}

#[test]
fn can_parse_full_app() {
    let test = r#"
[Desktop Entry]
Type=Application
TryExec=execme
Name=Test Name
Exec=alacritty
Icon=Alacritty
Terminal=false
Categories=System;TerminalEmulator;
GenericName=Terminal
Comment=A fast, cross-platform, OpenGL terminal emulator
Actions=New;

[Desktop Action New]
Name=New Terminal
Exec=testaction
    "#;
    // a

    let entry = parse_from_ini(Ini::load_from_str(test).unwrap()).unwrap();

    // println!("{entry:#?}");

    assert_eq!(entry.name, "Test Name");
    assert_eq!(entry.entry_type, EntryType::Application);
    assert_eq!(
        entry.categories,
        vec!["System".to_string(), "TerminalEmulator".to_string()]
    );
    assert_eq!(
        entry.action_list[0],
        Action {
            name: "New Terminal".to_string(),
            exec: Some("testaction".to_string()),
            icon_path: None
        }
    );
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
