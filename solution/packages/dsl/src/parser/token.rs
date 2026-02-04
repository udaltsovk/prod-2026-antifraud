use std::num::ParseFloatError;

use logos::Logos;

#[derive(thiserror::Error, Default, Debug, Clone, PartialEq, Eq)]
pub enum LexingError {
    #[error("Invalid number literal: {0}")]
    InvalidNumber(#[from] ParseFloatError),

    #[default]
    #[error("Unexpected character")]
    UnexpectedChar,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error(LexingError))]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'src> {
    Error(LexingError),

    #[token("AND", ignore(case))]
    And,

    #[token("OR", ignore(case))]
    Or,

    #[token("NOT", ignore(case))]
    Not,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token(">")]
    Gt,

    #[token(">=")]
    Gte,

    #[token("<")]
    Lt,

    #[token("<=")]
    Lte,

    #[token("=")]
    Eq,

    #[token("!=")]
    Neq,

    #[regex(r"[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse::<f64>())]
    Number(f64),

    #[regex(r"'([^']*)'", |lex| lex.slice())]
    String(&'src str),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_.]*")]
    Ident(&'src str),
}
