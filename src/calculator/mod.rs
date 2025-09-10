use iced::{Element, widget};
use pest::Parser;
use pest_derive::Parser;


// https://itnext.io/writing-a-mathematical-expression-parser-35b0b78f869e
// https://compilers.iecc.com/crenshaw/
// https://craftinginterpreters.com/contents.html

// use crate::module::Module;
//
// pub struct Calc {}
//
// impl Module for Calc {
//     fn update(&mut self, input: &str) {}
//     fn view(&self) -> Element<'_, String> {}
//     fn run(&self) {
//         // Copy to clipboard?
//     }
// }

enum TokenType {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    OpenParen,
    CloseParen,
}

// https://github.com/pest-parser/book/tree/master/examples/pest-calculator
#[derive(Parser)]
#[grammar = "calculator/math.pest"]
pub struct CalculatorParser;



// TEST THESE
// 4 + 3 - 2 * 9
// -3 * x ^ 2
// (1 + y) * 4.1
// sin PI
// fn(3, x + 1, sin PI / 2)
