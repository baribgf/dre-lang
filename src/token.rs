use logos::Logos;

use crate::{report::fatal, utils::bytes_to_string};

#[derive(Logos, PartialEq, Clone, Debug)]
#[logos(skip r" +")]
#[logos(utf8 = false)]
#[logos(subpattern alpha = r"[a-zA-Z]")]
#[logos(subpattern digit = r"[0-9]")]
#[logos(subpattern alphanum = r"(?&alpha)|(?&digit)")]
#[logos(subpattern intltrl = r"-?(?&digit)+")] // integer literal (sorry, for obfuscation)
pub enum Token {
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
    #[token(r"pub")]
    Pub,
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
    #[token(r"break")]
    Break,
    #[token(r"skip")]
    Skip,
    #[token(r"if")]
    If,
    #[token(r"then")]
    Then,
    #[token(r"else")]
    Else,
    #[token(r"true")]
    TrueLtrl,
    #[token(r"false")]
    FalseLtrl,
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
    #[token(r"const")]
    Const,
    #[token(r": ")]
    SpacedColon,
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
    #[token(r", ")]
    SpacedComma,
    #[token(r"->")]
    RightArrow,
    #[token(r"<-")]
    LeftArrow,
    #[token(".")]
    Period,
    #[token("..")]
    DoublePeriod,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("%")]
    Mod,
    #[token("<<")]
    ShiftL,
    #[token(">>")]
    ShiftR,
    #[token("|")]
    Pipe,
    #[token("&")]
    Ampersand,
    #[token("^")]
    Hat,
    #[token("~")]
    Tilde,

    #[regex(r"(?&intltrl)", |lex| {
        let literal = bytes_to_string(lex.slice());
        literal
            .parse::<i32>()
            .unwrap_or_else(|err| fatal(&format!("Invalid integer literal '{}': {}", literal, err)))
    })]
    IntegerLtrl(i32),

    #[regex(r"-?(?&digit)+\.(?&digit)+", |lex| {
        let literal = bytes_to_string(lex.slice());
        literal
            .parse::<f64>()
            .unwrap_or_else(|err| fatal(&format!("Invalid float literal '{}': {}", literal, err)))
    })]
    FloatLtrl(f64),

    #[regex(r"'[^']'", |lex| bytes_to_string(lex.slice()).chars().nth(0))]
    CharLtrl(char),

    #[regex(r"“[^”]*?”", |lex| bytes_to_string(lex.slice()))]
    StringLtrl(String),

    #[regex(r"(?s)---[^(---)]*?---")]
    MultiComment,

    #[regex(r"--.*?")]
    SingleComment,

    #[regex(r"((?&alpha)|@)_?((?&alphanum)_?)*|_", |lex| bytes_to_string(lex.slice()))]
    Identifier(String),
}

impl Token {
    pub fn identifier(&self) -> String {
        if let Token::Identifier(s) = self {
            return s.to_owned();
        }
        fatal(&format!("Token: {:?} is not an identifier.", self));
    }

    pub fn int_ltrl(&self) -> i32 {
        if let Token::IntegerLtrl(i) = self {
            return *i;
        }
        fatal(&format!("Token: {:?} is not an integer literal.", self));
    }

    pub fn float_ltrl(&self) -> f64 {
        if let Token::FloatLtrl(f) = self {
            return *f;
        }
        fatal(&format!("Token: {:?} is not a float literal.", self));
    }

    pub fn char_ltrl(&self) -> char {
        if let Token::CharLtrl(c) = self {
            return *c;
        }
        fatal(&format!("Token: {:?} is not a char literal.", self));
    }

    pub fn str_ltrl(&self) -> String {
        if let Token::StringLtrl(s) = self {
            return s.to_owned();
        }
        fatal(&format!("Token: {:?} is not a string literal.", self));
    }
}
