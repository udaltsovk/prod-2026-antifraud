use chumsky::prelude::*;
use logos::Logos as _;

use crate::{
    DslError, Expr, Literal, Operator,
    parser::token::{LexingError, Token},
};

mod token;

#[derive(Debug)]
pub enum ParserError<'src> {
    Lexer {
        message: String,
        near: String,
        position: usize,
    },
    Tokenizer(&'src str),
}

impl<'src> Expr<'src> {
    #[expect(
        clippy::string_slice,
        reason = "we're checking that we map to valid characters"
    )]
    pub fn parse(
        input: &'src str,
        tokens: &'src mut Vec<Token<'src>>,
    ) -> Result<Self, ParserError<'src>> {
        let lexer = Token::lexer(input);

        *tokens = lexer
            .spanned()
            .map(|(token, span)| token.map_err(|err| (err, span)))
            .collect::<Result<_, _>>()
            .map_err(|(err, span)| {
                let start =
                    input.char_indices().nth(span.start).map_or(0, |(i, _)| i);
                let end = input
                    .char_indices()
                    .nth(span.end)
                    .map_or(input.len(), |(i, _)| i);

                let message = match err {
                    LexingError::InvalidNumber(err) => err.to_string(),
                    LexingError::UnexpectedChar => {
                        format!("{err}: `{}`", &input[start..end])
                    },
                };

                ParserError::Lexer {
                    message,
                    near: input[start.saturating_sub(DslError::CONTEXT_SIZE)
                        ..end.saturating_add(DslError::CONTEXT_SIZE)]
                        .to_string(),
                    position: span.start,
                }
            })?;

        parser()
            .parse(tokens)
            .into_result()
            .map_err(|err| {
                dbg!(err);
                ParserError::Tokenizer("parser")
            })
            .map(Self::normalize)
    }
}

fn parser<'src>() -> impl Parser<
    'src,
    &'src [Token<'src>],
    Expr<'src>,
    extra::Err<Simple<'src, Token<'src>>>,
> {
    recursive(|expr| {
        let literal = select! {
            Token::Number(n) => Literal::Number(Some(n)),
            Token::String(s) => Literal::String(Some(s)),
        };

        let comparison = select! { Token::Ident(f) => f }
            .then(select! {
                Token::Gt => Operator::GreaterThan,
                Token::Gte => Operator::GreaterThan,
                Token::Lt => Operator::LessThan,
                Token::Lte =>Operator::LessThanOrEqual,
                Token::Eq => Operator::Equal,
                Token::Neq => Operator::NotEqual,
            })
            .then(literal)
            .map(|((field, op), value)| Expr::Comparison {
                field,
                op,
                value,
            });

        let atom = comparison
            .or(expr.delimited_by(just(Token::LParen), just(Token::RParen)))
            .map(|e| Expr::Parens(Box::new(e)));

        let unary = just(Token::Not)
            .repeated()
            .foldr(atom, |_op, rhs| Expr::Not(Box::new(rhs)));

        let product = unary.clone().foldl(
            just(Token::And).to(Expr::And).then(unary).repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        product.clone().foldl(
            just(Token::Or).to(Expr::Or).then(product).repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        )
    })
}
