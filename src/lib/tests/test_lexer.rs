#![cfg(test)]

use std::path::Path;

use crate::{Lexer, Scanner, Token};

use test_log::test;

use Token::*;

const BASE_PATH: &'static str = "src/lib/tests/inputs/tokens/";

fn tokenise<S: Into<String>>(source: S) -> Vec<Token> {
    let scanner = Scanner::from_string(source.into());
    Lexer::new(scanner).collect()
}

fn tokenise_file(path: &str) -> Vec<Token> {
    let path = Path::new(BASE_PATH).join(Path::new(path));
    let path = dbg!(path);
    let scanner = Scanner::from_path(&path).expect("Test file should exist");
    Lexer::new(scanner).collect()
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
    assert_eq!(vec![IntLiteral("100".into())], tokenise("100"));
    assert_eq!(vec![IntLiteral("100e2".into())], tokenise("100e2"));
    assert_eq!(vec![IntLiteral("100e2".into())], tokenise("100E2"));
    assert_eq!(vec![IntLiteral("100e+2".into())], tokenise("100E+2"));
    assert_eq!(vec![IntLiteral("100e-2".into())], tokenise("100E-2"));
    assert_eq!(vec![IntLiteral(".02".into())], tokenise(".02"));
    assert_eq!(vec![IntLiteral("100.10".into())], tokenise("100.10"));
    assert_eq!(vec![IntLiteral("100.10e2".into())], tokenise("100.10e2"));
    assert_eq!(vec![IntLiteral(".100e1".into())], tokenise(".100e1"));
    assert_eq!(vec![Dot, Id("e1".into())], tokenise(".e1"));
}

#[test]
fn char() {
    assert_eq!(vec![CharLiteral('c')], tokenise("'c'"));
    assert_eq!(vec![CharLiteral('\n')], tokenise("'\\n'"));
}

#[test]
fn string() {
    assert_eq!(
        vec![StringLiteral("test.hi".into())],
        tokenise("\"test.hi\"")
    );

    // assert_eq!(
    //     vec![StringLiteral("\n \t \\ \'".into())],
    //     tokenise("\\n \\t \\ \\'")
    // );
}

#[test]
fn lookahead() {
    let scanner = Scanner::from_string("a b c d e".into());
    let mut tt = Lexer::new(scanner);

    assert_eq!(Some(Id("a".into())), tt.peek(0));
    assert_eq!(Some(Id("a".into())), tt.peek(0));

    assert_eq!(Some(Id("b".into())), tt.peek(1));
    assert_eq!(Some(Id("b".into())), tt.peek(1));

    assert_eq!(Some(Id("a".into())), tt.peek(0));

    assert_eq!(Some(Id("a".into())), tt.next());

    assert_eq!(Some(Id("e".into())), tt.peek(3));
    assert_eq!(Some(Id("b".into())), tt.peek(0));

    assert_eq!(
        vec![
            Id("b".into()),
            Id("c".into()),
            Id("d".into()),
            Id("e".into()),
        ],
        tt.collect::<Vec<Token>>()
    )
}
