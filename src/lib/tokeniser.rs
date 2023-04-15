use std::{char, collections::VecDeque};

use log::{error, trace};

use crate::{Scanner, Token};

pub struct Tokenizer {
    scanner: Scanner,
    lookahead: VecDeque<Token>,
}

impl Tokenizer {
    pub fn new(scanner: Scanner) -> Self {
        return Self {
            scanner,
            lookahead: VecDeque::new(),
        };
    }

    pub fn peek(&mut self, idx: usize) -> Option<Token> {
        if idx < self.lookahead.len() {
            return self.lookahead.get(idx).map(|t| t.clone());
        }

        let backfill: Vec<Token> = (0..(idx + 1 - self.lookahead.len()))
            .filter_map(|_| self.parse_next_token())
            .collect();

        self.lookahead.extend(backfill.into_iter());

        self.lookahead.get(idx).map(|t| t.clone())
    }

    fn parse_next_token(&mut self) -> Option<Token> {
        self.trim_whitespace_comments();
        self.parse_separator()
            .or_else(|| self.parse_operator())
            .or_else(|| self.parse_idents_keywords())
            .or_else(|| self.parse_string_literal())
            .or_else(|| self.parse_char_literal())
            .or_else(|| self.parse_numeric_literal())
    }

    fn parse_separator(&mut self) -> Option<Token> {
        let next = self.scanner.peek_next();
        let token = match next {
            Some('{') => Some(Token::LCurly),
            Some('}') => Some(Token::RCurly),
            Some('[') => Some(Token::LBracket),
            Some(']') => Some(Token::RBracket),
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),
            Some(';') => Some(Token::Semicolon),
            Some(',') => Some(Token::Comma),
            Some(':') => Some(Token::Colon),
            _ => None,
        }?;
        self.accept();
        Some(token)
    }

    fn parse_operator(&mut self) -> Option<Token> {
        let first = self.scanner.peek_next();
        let second = self.scanner.peek(1);
        let (token, two_tokens) = match first {
            Some('+') => match second {
                Some('=') => Some((Token::PlusEq, true)),
                _ => Some((Token::Plus, false)),
            },
            Some('*') => match second {
                Some('=') => Some((Token::MultEq, true)),
                _ => Some((Token::Mult, false)),
            },
            Some('-') => match second {
                Some('>') => Some((Token::Stab, true)),
                Some('=') => Some((Token::MinusEq, true)),
                _ => Some((Token::Minus, false)),
            },
            Some('/') => match second {
                Some('*') | Some('/') => None,
                Some('=') => Some((Token::DivEq, true)),
                _ => Some((Token::Div, false)),
            },
            Some('<') => match second {
                Some('=') => Some((Token::LtEq, true)),
                _ => Some((Token::Lt, false)),
            },
            Some('>') => match second {
                Some('=') => Some((Token::GtEq, true)),
                _ => Some((Token::Gt, false)),
            },
            Some('=') => match second {
                Some('=') => Some((Token::EqEq, true)),
                Some('>') => Some((Token::Arrow, true)),
                _ => Some((Token::Eq, false)),
            },
            Some('&') => match second {
                Some('&') => Some((Token::AndAnd, true)),
                Some('=') => Some((Token::AndEq, true)),
                _ => Some((Token::And, false)),
            },
            Some('|') => match second {
                Some('|') => Some((Token::OrOr, true)),
                Some('=') => Some((Token::OrEq, true)),
                _ => Some((Token::Or, false)),
            },
            Some('!') => match second {
                Some('=') => Some((Token::NotEq, true)),
                _ => Some((Token::Not, false)),
            },
            _ => None,
        }?;
        self.accept();
        if two_tokens {
            self.accept();
        }
        Some(token)
    }

    fn parse_exponent(&mut self) -> String {
        let mut spelling = String::new();
        let e = match self.scanner.peek_next() {
            Some('e') | Some('E') => {
                self.accept();
                'e'
            }
            _ => return spelling,
        };
        spelling.push(e);
        let plus_minus = self.scanner.peek_next();
        if let Some(pm) = plus_minus {
            if pm == '+' || pm == '-' {
                self.accept();
                spelling.push(pm);
            }
        }
        spelling + &self.parse_digits()
    }

    fn parse_idents_keywords(&mut self) -> Option<Token> {
        if !self.scanner.peek_next()?.is_ascii_alphabetic() {
            return None;
        }
        let mut spelling = String::new();
        while let Some(c) = self.scanner.peek_next() {
            if !c.is_ascii_alphanumeric() || c == '_' {
                break;
            }
            self.accept();
            spelling.push(c);
        }
        Some(ident_or_keyword_from_spelling(spelling))
    }

    fn parse_string_literal(&mut self) -> Option<Token> {
        match self.scanner.peek_next() == Some('\"') {
            true => self.accept(),
            false => return None,
        };
        let mut spelling = String::new();
        loop {
            let next = self.accept();
            match next {
                Some('\"') => {
                    break;
                }
                Some('\\') => match escape_char(self.accept()) {
                    Some(c) => {
                        spelling.push(c);
                    }
                    None => {
                        unimplemented!("error: bad escape")
                    }
                },
                Some(c) => {
                    spelling.push(c);
                }
                None => {
                    error!("unterminated string with spelling: {:?}", spelling);
                    unimplemented!("Unterminated string")
                }
            }
        }
        Some(Token::StringLiteral(spelling))
    }

    fn parse_char_literal(&mut self) -> Option<Token> {
        let first = self.scanner.peek_next();
        if first != Some('\'') {
            return None;
        }
        self.accept();
        match self.accept() {
            Some('\\') => {
                let c = match escape_char(self.accept()) {
                    Some(c) => c,
                    None => unimplemented!("invalid escape"),
                };
                if self.accept() != Some('\'') {
                    unimplemented!("err: char must be one character")
                };
                return Some(Token::CharLiteral(c));
            }
            Some('\'') => {
                unimplemented!("empty char");
            }
            Some(c) => {
                if self.accept() != Some('\'') {
                    unimplemented!("err: char must be one character")
                }
                return Some(Token::CharLiteral(c));
            }
            None => unimplemented!("unterminated char"),
        };
    }

    fn parse_numeric_literal(&mut self) -> Option<Token> {
        let first = self.scanner.peek_next()?;
        if !(first.is_ascii_digit() || first == '.') {
            return None;
        }
        let lhs = self.parse_digits();
        let decimal = match self.scanner.peek_next() {
            Some('.') => {
                self.accept();
                String::from(".")
            }
            _ => String::from(""),
        };
        let rhs = self.parse_digits();
        if lhs.len() == 0 && rhs.len() == 0 {
            return match decimal.len() {
                0 => None,
                _ => Some(Token::Dot),
            };
        }
        let exponent = self.parse_exponent();
        let spelling = lhs + &decimal + &rhs + &exponent;
        Some(Token::IntLiteral(spelling))
    }

    fn parse_digits(&mut self) -> String {
        let mut spelling = String::new();
        while let Some(c) = self.scanner.peek_next() {
            if !c.is_ascii_digit() {
                break;
            }
            self.accept();
            spelling.push(c);
        }
        spelling
    }

    fn trim_whitespace_comments(&mut self) {
        loop {
            if self.at_whitespace() {
                self.accept();
            } else if self.at_comment() {
                self.clear_comment()
            } else {
                break;
            }
        }
    }

    fn at_whitespace(&self) -> bool {
        match self.scanner.peek_next() {
            Some(c) => c == '\n' || c == '\t' || c == '\r' || c == ' ',
            None => false,
        }
    }

    fn at_comment(&self) -> bool {
        match (self.scanner.peek_next(), self.scanner.peek(1)) {
            (Some('/'), Some('/')) => true,
            (Some('/'), Some('*')) => true,
            _ => false,
        }
    }

    fn clear_comment(&mut self) {
        self.accept();
        if self.scanner.next() == Some('/') {
            while let Some(next) = self.scanner.next() {
                if next == '\n' {
                    return;
                }
            }
        } else {
            self.accept();
            while let Some(next) = self.scanner.next() {
                if next == '*' && self.scanner.peek_next() == Some('/') {
                    self.accept();
                    return;
                }
            }
        }
    }

    fn accept(&mut self) -> Option<char> {
        let next = self.scanner.next();
        trace!("Accepting: {:?}", next);
        next
    }
}

fn ident_or_keyword_from_spelling(spelling: String) -> Token {
    match spelling.as_str() {
        "bool" => Token::Bool,
        "break" => Token::Break,
        "char" => Token::Char,
        "continue" => Token::Continue,
        "struct" => Token::Struct,
        "enum" => Token::Enum,
        "else" => Token::Else,
        "float" => Token::Float,
        "for" => Token::For,
        "if" => Token::If,
        "int" => Token::Int,
        "return" => Token::Return,
        "void" => Token::Void,
        "while" => Token::While,
        "loop" => Token::Loop,
        "match" => Token::Match,
        "true" => Token::BooleanLiteral(true),
        "false" => Token::BooleanLiteral(false),
        _ => Token::Id(spelling),
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.lookahead
            .pop_front()
            .or_else(|| self.parse_next_token())
    }
}

fn escape_char(c: Option<char>) -> Option<char> {
    match c? {
        '\"' => Some('\"'),
        'n' => Some('\n'),
        't' => Some('\t'),
        '\\' => Some('\\'),
        '\'' => Some('\''),
        _ => None,
    }
}
