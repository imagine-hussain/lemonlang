use std::{collections::VecDeque, iter::Peekable, path::Path};

pub struct Scanner {
    source: VecDeque<char>,
}

impl Scanner {
    pub fn from_string(source: String) -> Self {
        let source = source.chars().collect();
        Self::new(source)
    }

    pub fn from_path(path: &Path) -> Result<Self, std::io::Error> {
        let file = std::fs::read(path)?;
        let source = file.into_iter().map(|byte| byte.into()).collect();
        Ok(Self::new(source))
    }

    pub fn new(source: VecDeque<char>) -> Self {
        Self { source }
    }

    pub fn peek(&self, index: usize) -> Option<char> {
        self.source.get(index).map(|c| *c)
    }
    pub fn peek_next(&self) -> Option<char> {
        self.source.front().map(|c| *c)
    }
}

impl Iterator for Scanner {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.pop_front()
    }
}
