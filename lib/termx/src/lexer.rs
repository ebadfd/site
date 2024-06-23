/*
* language semantics.
*
* only allow defined commands to run;
* - whoami
* - cd
* - cat
* - echo
*
* Optional commands;
* - for loops
*
* Comment semantics are #
*/

use color_eyre::Result;

use crate::errors::LexerError;

pub static LITERAL_TOKENS: &[(&str, TokenEnum)] = &[
    ("(", TokenEnum::OParan),
    (")", TokenEnum::CParan),
    ("{", TokenEnum::OCurly),
    ("}", TokenEnum::CCurly),
    (";", TokenEnum::SemiColon),
];

#[derive(Debug)]
pub struct Lexer {
    pub source: String,
    pub cur: usize,
    pub bol: usize,
    pub row: usize,
}

#[derive(Debug, Clone)]
pub struct Loc {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenEnum {
    Name,
    OParan,
    CParan,
    OCurly,
    CCurly,
    SemiColon,
    Number,
    String,
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub loc: Loc,
    pub token: TokenEnum,
}

impl Token {
    pub fn new(token: TokenEnum, value: String, loc: Loc) -> Token {
        Token { token, value, loc }
    }
}

impl Loc {
    pub fn new(row: usize, col: usize) -> Loc {
        return Loc { row, col };
    }

    pub fn to_string(&self) -> String {
        return String::from(format!("{}:{}", self.row + 1, self.col + 1));
    }
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            cur: 0,
            bol: 0,
            row: 0,
        }
    }

    pub fn has_char(&self) -> bool {
        return self.cur < self.source.len();
    }

    pub fn is_eof(&self) -> bool {
        return !self.has_char();
    }

    pub fn trim_left(&mut self) {
        while self.has_char() && self.current_char().is_whitespace() {
            self.chop_char();
        }
    }

    pub fn chop_char(&mut self) {
        if self.has_char() {
            let x = self.current_char();
            self.cur += 1;

            if x == 0xA as char {
                self.bol = self.cur;
                self.row += 1
            }
        }
    }

    pub fn curent_char_peek(&self) -> Option<char> {
        self.source.chars().nth(self.cur)
    }

    pub fn current_char(&self) -> char {
        return self.source.chars().nth(self.cur).unwrap();
    }

    pub fn peek(&self) -> Option<char> {
        return self.source.chars().nth(self.cur + 1);
    }

    fn drop_line(&mut self) {
        while self.has_char() && self.current_char() != 0xA as char {
            self.chop_char();
        }

        if self.has_char() {
            self.chop_char();
        }
    }

    pub fn loc(&self) -> Loc {
        return Loc::new(self.row, self.cur);
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.trim_left();
        let sub_str_current: String = self
            .source
            .clone()
            .chars()
            .into_iter()
            .skip(self.cur)
            .collect();

        if sub_str_current.starts_with("#") {
            self.drop_line();
            self.trim_left();
        }

        if self.is_eof() {
            return Err(LexerError::EOF);
        }

        let loc = self.loc();
        let first = self.current_char();

        if first.is_alphabetic() {
            let index = self.cur;

            while self.has_char() && self.current_char().is_alphanumeric() {
                self.chop_char()
            }

            let value = &self.source[index..self.cur];
            let token = Token::new(TokenEnum::Name, value.to_string(), loc.clone());

            return Ok(token);
        }

        if let Some(token_type) = LITERAL_TOKENS
            .iter()
            .find(|&(literal, _)| *literal == first.to_string())
        {
            self.chop_char();
            let token = Token::new(token_type.1.clone(), first.to_string(), loc.clone());
            return Ok(token);
        }

        if first == '"' {
            self.chop_char();
            let start = self.cur;

            while self.has_char() && self.current_char() != '"' {
                self.chop_char()
            }

            if self.has_char() {
                let value = &self.source.clone()[start..self.cur];
                self.chop_char();
                let token = Token::new(TokenEnum::String, value.to_string(), loc.clone());

                return Ok(token);
            }

            return Err(LexerError::UnclosedStringLiteral(loc.to_string()));
        }

        if first.is_ascii_digit() {
            let index = self.cur;

            while self.has_char() && self.current_char().is_ascii_digit() {
                self.chop_char()
            }

            let value = &self.source[index..self.cur];
            let token = Token::new(TokenEnum::Number, value.to_string(), loc.clone());

            return Ok(token);
        }

        return Err(LexerError::NotImplemented);
    }
}
