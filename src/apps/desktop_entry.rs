// https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html

use std::{collections::HashMap, vec::Vec};
use walkdir::WalkDir;

use log;

#[derive(Debug, PartialEq)]
pub struct Action {
    pub name: String,
    pub icon_path: Option<String>,
    pub exec: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum EntryType {
    Application,
    Link,
    Directory,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct DesktopEntry {
    pub entry_type: EntryType,
    pub version: Option<String>,
    pub name: String,
    pub generic_name: Option<String>,
    // No display not included since its irrelevant. Should be handled in parsing
    pub comment: Option<String>,
    // pub icon_path: Option<std::path::PathBuf>, // https://specifications.freedesktop.org/icon-theme-spec/latest/
    pub icon_path: Option<String>, // https://specifications.freedesktop.org/icon-theme-spec/latest/
    // Handle files with Hidden at parsing level
    pub only_show_in: Vec<String>,
    pub not_show_in: Vec<String>,
    // I do not support dbus activation idk what that is
    pub try_exec: Option<String>,
    pub exec: String, // Techicially optional, nuh uh.
    pub working_dir: Option<String>,
    pub terminal: bool,
    pub action_list: Vec<Action>,
    // mime_types: Option<
    pub categories: Vec<String>,
    // No impliments
    pub keywords: Vec<String>,
    pub url: Option<String>,
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
}

fn parse_entry_from_string(
    input: &str,
) -> Result<HashMap<String, HashMap<String, String>>, ParseError> {
    let mut main_map: HashMap<String, HashMap<String, String>> = HashMap::new();

    let mut current_heading = String::new();
    let mut current_map: HashMap<String, String> = HashMap::new();

    for line in input.lines() {
        log::trace!("current_line: {line}");

        if let Some(l) = line.split_once('=') {
            current_map.insert(l.0.trim().to_string(), l.1.trim().to_string());
            continue;
        }

        if line.starts_with("#") {
            continue;
        }

        if line.starts_with("[") {
            if !current_map.is_empty() {
                log::trace!("current map was not empty");
                main_map.insert(
                    std::mem::take(&mut current_heading),
                    std::mem::take(&mut current_map),
                );
            }

            current_heading = line
                .get(1..line.len() - 1)
                .ok_or(ParseError::BadGroupHeader)?
                .to_string();
            log::trace!("current heading being set. Is set to {current_heading}");
            continue;
        }
    }
    if !current_map.is_empty() {
        main_map.insert(
            std::mem::take(&mut current_heading),
            std::mem::take(&mut current_map),
        );
    }

    Ok(main_map)
}

#[test]
fn can_parse_entry_from_str() {
    let mut hash = HashMap::new();
    let mut main_map = HashMap::new();
    main_map.insert("Type".to_string(), "Application".to_string());
    main_map.insert(
        "Categories".to_string(),
        "System;TerminalEmulator;".to_string(),
    );
    hash.insert("Desktop Entry".to_string(), main_map);

    assert_eq!(
        parse_entry_from_string(
            r#"[Desktop Entry]
Type=Application
Categories=System;TerminalEmulator;"#
        )
        .unwrap(),
        hash
    )
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
    let contents = std::fs::read_to_string(file_path).map_err(|_| ParseError::CouldNotLoadFile)?;

    return parse_from_hashmap(parse_entry_from_string(&contents)?);
}

fn parse_from_hashmap(
    input: HashMap<String, HashMap<String, String>>,
) -> Result<DesktopEntry, ParseError> {
    // let mut entry = DesktopEntry::default();

    let Some(entry_keys) = input.get("Desktop Entry") else {
        return Err(ParseError::DesktopEntryHeaderNotFound);
    };

    if matches!(
        entry_keys.get("NoDisplay").map(|s| s.as_str()),
        Some("true")
    ) || matches!(entry_keys.get("Hidden").map(|s| s.as_str()), Some("true"))
    {
        return Err(ParseError::NoDisplayTrue);
    }

    // let entry_keys: &i = entry_keys;

    let entry_type = match entry_keys.get("Type").map(|s| s.as_str()) {
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
                .ok_or(ParseError::MissingRequiredField)?
                .as_str(),
            entry_keys.get("Icon").map(|s| s.as_str()),
            entry_keys.get("Name").map(|s| s.as_str()),
        ),
        generic_name: entry_keys.get("GenericName").map(|s| s.to_string()),
        comment: entry_keys.get("Comment").map(|s| s.to_string()),
        icon_path: entry_keys.get("Icon").map(|s| s.to_string()),
        only_show_in: parse_string_list(entry_keys.get("OnlyShowIn").map(|s| s.as_str())),
        not_show_in: parse_string_list(entry_keys.get("NotShowIn").map(|s| s.as_str())),
        working_dir: entry_keys.get("Path").map(|s| s.to_string()),
        terminal: entry_keys.get("Terminal").map_or(false, |b| b == "true"),
        categories: parse_string_list(entry_keys.get("Categories").map(|s| s.as_str())),
        keywords: parse_string_list(entry_keys.get("Keywords").map(|s| s.as_str())),
        url: match entry_keys.get("URL") {
            Some(s) => Some(s.to_string()),
            _ if entry_type == EntryType::Link => return Err(ParseError::MissingRequiredField),
            _ => None,
        },
        action_list: parse_string_list(entry_keys.get("Actions").map(|s| s.as_str())) // i dont like this whole thing
            .into_iter()
            .map(|name: String| {
                let section = input
                    .get(&format!("Desktop Action {}", name))
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
                            log::debug!(
                                "Hit funny %i (sub in icon) escape sequence for app with icon {icon}"
                            );
                            escaped_result += format!("--icon {icon}").as_str();
                        };
                    }
                    'c' => {
                        if let Some(name) = name {
                            log::debug!(
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


// https://specifications.freedesktop.org/icon-theme-spec/latest/
fn find_icon(name: &str, size: i32, scale: i32) -> Option<std::path::PathBuf> {
    // gsettings get org.gnome.desktop.interface gtk-theme
    let mut user_theme_string: Option<String> = None;

    if let Ok(get_theme_cmd) = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output()
    {
        user_theme_string = Some(
            String::from_utf8_lossy(&get_theme_cmd.stdout)
                .trim()
                .trim_matches('\'')
                .to_string(),
        );
    }

    println!("user theme streing is : {user_theme_string:?}");

    if user_theme_string.is_some() {
        if let Some(filename) = find_icon_helper(name, size, scale, &user_theme_string.unwrap()) {
            return Some(filename)
        };
    }

    if let Some(filename) = find_icon_helper(name, size, scale, "hicolor") {
        return Some(filename)
    };


    return lookup_fallback_icon(name)
}

#[test]
fn can_find_icon() {
    find_icon("", 1, 1);
    assert!(1 == 1);
}

fn find_icon_helper(
    name: &str,
    size: i32,
    scale: i32,
    theme: &str,
) -> Option<std::path::PathBuf> {
    if let Some(filename) = lookup_icon(name, size, scale, theme) {
        return Some(filename)
    };
    // Theme has parents??

    Some("find icon helper".into())
}

fn lookup_icon(
    name: &str,
    size: i32,
    scale: i32,
    theme: &str,) -> Option<std::path::PathBuf> { 

    Some("lookup icon".into())
}

fn lookup_fallback_icon(name: &str) -> Option<std::path::PathBuf> {
    Some("Fallback icon".into())
}


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

    let entry = parse_from_hashmap(parse_entry_from_string(test).unwrap()).unwrap();

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
