/// Like serde, but worse!
use std::collections::HashMap;

#[derive(Debug)]
pub enum ParseError {
    /// Missing seperator between key and value
    MissingSeperator,
    /// Failed to turn key/value of item into type required
    IntoFailure,
    /// Bad Header
    BadHeader,
}

/// Input data is a csv
pub fn parse_csv<T: std::str::FromStr>(data: &str) -> Result<HashMap<String, T>, ParseError> {
    let mut map = HashMap::new();
    for line in data.lines() {
        let (start, end) = line.split_once(',').ok_or(ParseError::MissingSeperator)?;
        map.insert(
            start.to_owned(),
            end.parse::<T>().map_err(|_| ParseError::IntoFailure)?,
        );
    }

    Ok(map)
}

/// Inverse function of parse_csv
pub fn hash_map_to_csv<T: std::string::ToString + Clone>(map: HashMap<String, T>) -> String {
    let mut out = String::new();
    for (key, val) in map.iter() {
        out += &(key.clone() + "," + &val.to_string() + "\n");
    }
    out
}

#[test]
fn can_parse_csv() {
    let raw_string = r#"one,1
two,2
ten,10"#;
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("one".into(), 1);
    map.insert("two".into(), 2);
    map.insert("ten".into(), 10);
    assert_eq!(parse_csv::<u32>(raw_string).expect("Can parse_csv"), map);
    // Check inverse
    // Doesnt work since order is indeterminate
    // assert_eq!(hash_map_to_csv::<u32>(map), raw_string);
}

// Using lifetimes here may look ugly, but it leads to a 30% performance improvement from reduced
// heap allocations
pub fn parse_ini_format<'a>(
    input: &'a str,
) -> Result<HashMap<&'a str, HashMap<&'a str, &'a str>>, ParseError> {
    let mut main_map: HashMap<&'a str, HashMap<&'a str, &'a str>> = HashMap::new();

    let mut current_heading = "";
    let mut current_map: HashMap<&'a str, &'a str> = HashMap::new();

    for line in input.lines() {
        if let Some(l) = line.split_once('=') {
            current_map.insert(l.0, l.1);
            continue;
        }

        if line.starts_with("#") {
            continue;
        }

        if line.starts_with("[") {
            if !current_map.is_empty() {
                main_map.insert(
                    std::mem::take(&mut current_heading),
                    std::mem::take(&mut current_map),
                );
            }

            // Get characters between starting and ending []. i.e, the xxx in "[xxx]"
            current_heading = line.get(1..line.len() - 1).ok_or(ParseError::BadHeader)?;
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
    main_map.insert("Type", "Application");
    main_map.insert("Categories", "System;TerminalEmulator;");
    hash.insert("Desktop Entry", main_map);

    assert_eq!(
        parse_ini_format(
            r#"[Desktop Entry]
Type=Application
Categories=System;TerminalEmulator;"#
        )
        .unwrap(),
        hash
    )
}
