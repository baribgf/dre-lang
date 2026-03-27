use logos::{Lexer as LogosLexer, Logos};

use crate::{
    report::error,
    token::{Token, TokenType},
    utils::get_nth_line,
};

pub struct Lexer<'a> {
    source_path: &'a String,
    logos_lexer: LogosLexer<'a, TokenType>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(path: &'a String, source: &'a [u8]) -> Self {
        Self {
            source_path: path,
            logos_lexer: TokenType::lexer(source),
            line: 1,
        }
    }

    pub fn step(&mut self) -> Option<Token> {
        let token = self.logos_lexer.next();
        let lexeme = unsafe { String::from_utf8_unchecked(self.logos_lexer.slice().to_vec()) };
        match token {
            Some(token_res) => match token_res {
                Ok(token_type) => {
                    match token_type {
                        TokenType::Newline => {
                            self.line += 1;
                        }
                        TokenType::MultiComment => {
                            self.line += lexeme.chars().filter(|c| *c == '\n').count()
                        }
                        _ => {}
                    }

                    Some(Token::new(token_type, lexeme))
                }
                Err(_) => {
                    let token_start = self.logos_lexer.span().start;
                    let mut i = token_start;
                    while self.logos_lexer.source()[i] != b'\n' {
                        i -= 1;
                    }
                    let col = token_start - i;
                    error(
                        "Invalid token",
                        &get_nth_line(self.logos_lexer.source(), self.line),
                        self.source_path,
                        self.line,
                        col,
                    );
                }
            },
            None => None,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}
