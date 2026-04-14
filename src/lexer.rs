use logos::{Lexer as LogosLexer, Logos};

use crate::{report::error, token::Token, utils::get_nth_line};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    source_path: &'input String,
    inner: LogosLexer<'input, Token>,
    line: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(path: &'input String, source: &'input [u8]) -> Self {
        Self {
            source_path: path,
            inner: Token::lexer(source),
            line: 1,
        }
    }

    pub fn step(&mut self) -> Option<Token> {
        let token_opt = self.inner.next();
        let lexeme = unsafe { String::from_utf8_unchecked(self.inner.slice().to_vec()) };
        match token_opt {
            Some(token_res) => match token_res {
                Ok(token) => {
                    match token {
                        Token::Newline => {
                            self.line += 1;
                        }
                        Token::MultiComment => {
                            self.line += lexeme.chars().filter(|c| *c == '\n').count();
                            return self.step();
                        }
                        Token::SingleComment => {
                            return self.step();
                        }

                        _ => {}
                    }

                    Some(token)
                }
                Err(_) => {
                    let (line, col) = self.get_position();
                    error(
                        "Invalid token",
                        &get_nth_line(self.inner.source(), line),
                        self.source_path,
                        line,
                        col,
                    );
                }
            },
            None => None, // End of input reached
        }
    }

    fn get_position(&self) -> (usize, usize) {
        let source = self.inner.source();
        if source.is_empty() {
            return (self.line, 1);
        }

        let token_start = self.inner.span().start.min(source.len() - 1);
        let mut i = token_start;
        while source[i] != b'\n' && i > 0 {
            i -= 1;
        }
        let col = (token_start - i).max(1);
        (self.line, col)
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let (line, col) = self.get_position();

        match self.step() {
            Some(token) => Some(Ok((line, token, col))),
            None => None,
        }
    }
}
