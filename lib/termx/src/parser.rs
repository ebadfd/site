use crate::{
    errors::ParserError,
    lexer::{Lexer, Token, TokenEnum},
};

#[derive(Debug)]
pub enum ParserResult {
    SystemCmd(String, Vec<Token>),
    FunctionCall(String, Vec<Token>),
    FunctionDeclaration(String, Vec<Token>),
    Error(String),
}

#[derive(Debug)]
pub struct Parser {
    pub lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser { lexer }
    }

    pub fn parse(&mut self, system_commands: Vec<&str>) -> Result<Vec<ParserResult>, ParserError> {
        let mut results = Vec::new();

        while !self.lexer.is_eof() {
            let next_token = match self.lexer.next_token() {
                Ok(token) => token,
                Err(err) => return Err(ParserError::LexerError(err.to_string())),
            };

            match next_token.token {
                TokenEnum::Name => {
                    if self.is_valid_system_command(&next_token.value, &system_commands) {
                        let args = self.parse_arguments()?;
                        results.push(ParserResult::SystemCmd(next_token.value, args));
                    } else if self.is_function_declaration() {
                        let body = self.parse_function_declaration()?;
                        results.push(ParserResult::FunctionDeclaration(next_token.value, body));
                    } else {
                        let args = self.parse_arguments()?;
                        results.push(ParserResult::FunctionCall(next_token.value, args));
                    }
                }
                _ => {
                    return Err(ParserError::UnexpectedToken(next_token.loc.to_string()));
                }
            }
        }

        Ok(results)
    }

    fn is_valid_system_command(&self, name: &str, system_commands: &Vec<&str>) -> bool {
        system_commands.contains(&name)
    }

    fn is_function_declaration(&self) -> bool {
        // check if the next token is `(` in that case it's a func definiation
        match self.lexer.curent_char_peek() {
            Some(p) => p == '(',
            None => false,
        }
    }

    fn parse_function_declaration(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut body = Vec::new();

        while self.lexer.has_char() {
            let next_token = match self.lexer.next_token() {
                Ok(token) => token,
                Err(err) => return Err(ParserError::LexerError(err.to_string())),
            };

            match next_token.token {
                TokenEnum::OParan => continue,
                TokenEnum::CParan => continue,
                TokenEnum::OCurly => continue,
                TokenEnum::CCurly => break,
                _ => body.push(next_token),
            }
        }

        Ok(body)
    }

    fn parse_arguments(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut args = Vec::new();

        while self.lexer.has_char() {
            let next_token = match self.lexer.next_token() {
                Ok(token) => token,
                Err(err) => return Err(ParserError::LexerError(err.to_string())),
            };

            match next_token.token {
                TokenEnum::SemiColon => break,
                _ => args.push(next_token),
            }
        }

        Ok(args)
    }
}
