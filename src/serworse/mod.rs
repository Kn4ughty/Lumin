/// Like serde, but worse!
use std::collections::HashMap;

#[derive(Debug)]
pub enum ParseError {
    MissingSeperator,
    IntoFailure,
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

/// Invere function of parse_csv
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
