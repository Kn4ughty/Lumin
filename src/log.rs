#![allow(dead_code)]

const ESC: &str = "\x1B[";

const CLEAR_FMT: &str = "0";

const BOLD: &str = "1";

// Foreground colors (text colour)
const FGRED: &str = "31";
const FGYELLOW: &str = "33";
const FGBLUE: &str = "34";

pub fn info(s: &str) {
    println!("{ESC}{FGBLUE}minfo{ESC}{CLEAR_FMT}m: {s}");
}

pub fn warn(s: &str) {
    println!("{ESC}{FGYELLOW};{BOLD}mwarning{ESC}{CLEAR_FMT}m: {s}");
}

pub fn err(s: &str) {
    println!("{ESC}{FGRED};{BOLD}merror:{ESC}{CLEAR_FMT}m: {s}");
}
