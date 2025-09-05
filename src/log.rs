#![allow(dead_code)]

const ESC: &str = "\x1B[";

const CLEAR_FMT: &str = "0";

const BOLD: &str = "1";

// Foreground colors (text colour)
const FGRED: &str = "31";
const FGYELLOW: &str = "33";
const FGBLUE: &str = "34";

pub fn info(s: impl std::string::ToString) {
    let s = s.to_string();
    println!("{ESC}{FGBLUE}minfo{ESC}{CLEAR_FMT}m: {s}");
}

pub fn warn(s: impl std::string::ToString) {
    let s = s.to_string();
    println!("{ESC}{FGYELLOW};{BOLD}mwarning{ESC}{CLEAR_FMT}m: {s}");
}

pub fn err(s: impl std::string::ToString) {
    let s = s.to_string();
    println!("{ESC}{FGRED};{BOLD}merror:{ESC}{CLEAR_FMT}m: {s}");
}
