use chumsky::{Parser, prelude::just, recursive, select};
use logos::Logos;

#[derive(Debug)]
pub enum Field {
    Amount,
}

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    #[logos("amount")]
    Amount,

    #[logos(">")]
    GreaterThan,

    #[logos(">=")]
    GreaterThanOrEqual,

    #[logos("<")]
    LessThan,

    #[logos("<=")]
    LessThanOrEqual,

    #[logos("=")]
    Equal,

    #[logos("!=")]
    NotEqual,

    #[regex(
        "[0-9]+",
        |lex| {
            lex.slice()
               .parse::<u64>()
               .expect("those numbers should be in the u64 range")
        }
    )]
    Integer(u64),
}

#[derive(Debug)]
pub enum Expression {
    Field(Field),

    Integer(u64),

    GreaterThan(Box<Self>, Box<Self>),
    GreaterThanOrEqual(Box<Self>, Box<Self>),
    LessThan(Box<Self>, Box<Self>),
    LessThanOrEqual(Box<Self>, Box<Self>),
    Equal(Box<Self>, Box<Self>),
    NotEqual(Box<Self>, Box<Self>),
}

fn parser<'src>() -> impl Parser<
    'src,
    &'src [Token],
    Expression,
    chumsky::extra::Err<chumsky::error::Simple<'src, Token>>,
> {
    recursive(|p| {
        let atom = {
            let field = select! {
                Token::Amount => Expression::Field(Field::Amount)
            };

            let integer = select! {
                Token::Integer(n) => Expr::Int(n),
            };

            field.or(integer)
        };

        let unary = just(Token::Minus)
            .repeated()
            .foldr(atom, |_op, rhs| Expr::Neg(Box::new(rhs)));

        let binary_1 = unary.clone().foldl(
            just(Token::Multiply)
                .or(just(Token::Divide))
                .then(unary)
                .repeated(),
            |lhs, (op, rhs)| match op {
                Token::Multiply => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                Token::Divide => Expr::Div(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        );

        let binary_2 = binary_1.clone().foldl(
            just(Token::Plus)
                .or(just(Token::Minus))
                .then(binary_1)
                .repeated(),
            |lhs, (op, rhs)| match op {
                Token::Plus => Expr::Add(Box::new(lhs), Box::new(rhs)),
                Token::Minus => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        );

        binary_2
    })
}
