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
                            self.line += lexeme.chars().filter(|c| *c == '\n').count()
                        }
                        _ => {}
                    }

                    Some(token)
                }
                Err(_) => {
                    let col = self.get_curr_column();
                    error(
                        "Invalid token",
                        &get_nth_line(self.inner.source(), self.line),
                        self.source_path,
                        self.line,
                        col,
                    );
                }
            },
            None => None, // End of input reached
        }
    }

    fn get_curr_column(&self) -> usize {
        let token_start = self.inner.span().start;
        let mut i = token_start;
        let source = self.inner.source();
        while source[i] != b'\n' && i > 0 {
            i -= 1;
        }
        let col = token_start - i;
        col
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let span = self.inner.span();
        let start = span.start;
        let end = span.end;

        match self.step() {
            Some(token) => Some(Ok((start, token, end))),
            None => None,
        }
    }
}
