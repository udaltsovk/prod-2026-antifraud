use std::fmt;

use strum::{Display, EnumString};

pub use crate::{
    context::{Context, ContextBuilder},
    parser::ParserError,
    validator::ValidatorError,
};

mod context;
mod evaluator;
mod normalizer;
mod parser;
mod validator;

#[derive(EnumString, Display, Debug, Clone, Copy)]
pub enum Operator {
    #[strum(serialize = ">")]
    GreaterThan,

    #[strum(serialize = ">=")]
    GreaterThanOrEqual,

    #[strum(serialize = "<")]
    LessThan,

    #[strum(serialize = "<=")]
    LessThanOrEqual,

    #[strum(serialize = "=")]
    Equal,

    #[strum(serialize = "!=")]
    NotEqual,
}

#[derive(Debug, Clone, Copy)]
pub enum Literal<'src> {
    Number(Option<f64>),
    String(Option<&'src str>),
}

impl fmt::Display for Literal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => {
                n.expect("literal in the ast must be some").fmt(f)
            },
            Self::String(s) => {
                s.expect("literal in the ast must be some").fmt(f)
            },
        }
    }
}

#[derive(Display, Debug, Clone)]
pub enum Expression<'src> {
    #[strum(to_string = "{field} {op} {value}")]
    Comparison {
        field: &'src str,
        op: Operator,
        value: Literal<'src>,
    },

    #[strum(to_string = "({0})")]
    Parens(Box<Self>),

    #[strum(to_string = "NOT {0}")]
    Not(Box<Self>),

    #[strum(to_string = "{0} AND {1}")]
    And(Box<Self>, Box<Self>),

    #[strum(to_string = "{0} OR {1}")]
    Or(Box<Self>, Box<Self>),
}
