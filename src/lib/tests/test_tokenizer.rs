#![cfg(test)]

use std::path::Path;

use crate::{Scanner, Token, Tokenizer};

use Token::*;

const BASE_PATH: &'static str = "src/lib/tests/inputs/tokens/";

fn tokenise(source: String) -> Vec<Token> {
    let scanner = Scanner::from_string(source);
    Tokenizer::new(scanner).collect()
}

fn tokenise_file(path: &str) -> Vec<Token> {
    let path = Path::new(BASE_PATH).join(Path::new(path));
    let path = dbg!(path);
    let scanner = Scanner::from_path(&path).expect("Test file should exist");
    Tokenizer::new(scanner).collect()
}

#[test]
fn empty() {
    assert_eq!(0, tokenise(String::from("")).len());
}

#[test]
fn comments() {
    let invalid_nest = String::from("/* comments cannot nest */ ****/");
    let invalid_nest_res = tokenise(invalid_nest);
    println!("{:?}", invalid_nest_res);
    assert_eq!(vec![Mult, Mult, Mult, Mult, Div], invalid_nest_res);

    let whitespace = String::from("     /* */  //  ");
    assert_eq!(0, tokenise(whitespace).len());
}

#[test]
fn keywords() {
    let res = tokenise_file("keywords_ident.txt");
    assert_eq!(
        vec![
            Bool,
            Break,
            Continue,
            Else,
            Break,
            Continue,
            Else,
            Float,
            For,
            If,
            Int,
            Return,
            Float,
            Id(String::from("floater")),
            Id(String::from("breaker")),
            Id(String::from("cont")),
            Id(String::from("id")),
            Id(String::from("ifs")),
        ],
        res
    );
}

// fn tests_operators() {
//     let res = tokenise_file("operators.txt");
// }

#[test]
fn int() {
    assert_eq!(vec![IntLiteral("100".into())], tokenise("100".into()));
}
