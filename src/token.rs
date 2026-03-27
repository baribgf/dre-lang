use std::fmt::Display;

use logos::Logos;

#[derive(Logos, PartialEq, Clone, Debug)]
#[logos(skip r" +")]
#[logos(utf8 = false)]
#[logos(subpattern alpha = r"[a-zA-Z]")]
#[logos(subpattern digit = r"[0-9]")]
#[logos(subpattern alphanum = r"(?&alpha)|(?&digit)")]
#[logos(subpattern intltrl = r"-?(?&digit)+")]
pub enum TokenType {
    #[token("\n")]
    Newline,
    #[token(r"use")]
    Use,
    #[token(r"let")]
    Let,
    #[token(r"struct")]
    Struct,
    #[token(r"enum")]
    Enum,
    #[token(r"prot")]
    Prot,
    #[token(r"func")]
    Func,
    #[token(r"begin")]
    Begin,
    #[token(r"end")]
    End,
    #[token(r"is")]
    Is,
    #[token(r"public")]
    Public,
    #[token(r"private")]
    Private,
    #[token(r"self")]
    Self_,
    #[token(r"return")]
    Return,
    #[token(r"until")]
    Until,
    #[token(r"while")]
    While,
    #[token(r"for")]
    For,
    #[token(r"do")]
    Do,
    #[token(r"in")]
    In,
    #[token(r"if")]
    If,
    #[token(r"then")]
    Then,
    #[token(r"else")]
    Else,
    #[token(r"true")]
    True,
    #[token(r"false")]
    False,
    #[token(r"not")]
    Not,
    #[token(r"and")]
    And,
    #[token(r"or")]
    Or,
    #[token(r"inc")]
    Inc,
    #[token(r"dec")]
    Dec,
    #[token(r":")]
    Colon,
    #[token(r"::")]
    DoubleColon,
    #[token(r"=")]
    Eq,
    #[token(r"<>")]
    Diamond,
    #[token(r"<")]
    Lt,
    #[token(r"<=")]
    Lte,
    #[token(r">")]
    Gt,
    #[token(r">=")]
    Gte,
    #[token(r"(")]
    OpenParen,
    #[token(r")")]
    ClosParen,
    #[token(r"[")]
    OpenBracket,
    #[token(r"]")]
    ClosBracket,
    #[token(r"“")]
    OpenDQuote,
    #[token(r"”")]
    ClosDQuote,
    #[token(r"'")]
    SingleQuote,
    #[token(r", ")]
    SpacedComma,
    #[token(r"->")]
    RightArrow,
    #[token(r"<-")]
    LeftArrow,
    #[token(r"@")]
    At,
    #[token(".")]
    Period,

    #[regex(r"[+\-*/%]")]
    Arith,

    #[regex(r"<<|>>|\||[\^&!]")]
    Bitwise,

    #[regex(r"(?&intltrl)")]
    IntegerLtrl,

    #[regex(r"-?(?&digit)+\.(?&digit)+")]
    FloatLtrl,

    #[regex(r"“[^”]*?”")]
    StringLtrl,

    #[regex(r"(?s)---[^(---)]*?---")]
    MultiComment,

    #[regex(r"--.*?")]
    SingleComment,

    #[regex(r"(?&alpha)_?((?&alphanum)_?)*|_")]
    Identifier,
}

pub struct Token {
    type_: TokenType,
    lexeme: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "<{:?}, \"{}\">",
            self.type_,
            self.lexeme.clone().replace("\n", "\\n")
        ))
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Self {
        Token {
            type_: token_type,
            lexeme,
        }
    }
}
