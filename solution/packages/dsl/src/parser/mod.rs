use chumsky::{
    IterParser as _, Parser,
    error::Rich,
    extra,
    input::{Input as _, Stream, ValueInput},
    primitive::just,
    recursive::recursive,
    select,
    span::SimpleSpan,
};
use logos::Logos as _;

use crate::{
    Expression, Literal, Operator,
    parser::token::{LexingError, Token},
};

mod token;

#[derive(Debug)]
pub struct ParserError<'src> {
    pub message: String,
    pub near: &'src str,
    pub position: usize,
}
impl ParserError<'_> {
    pub const CONTEXT_SIZE: usize = 2;
}

impl<'src> Expression<'src> {
    #[expect(
        clippy::string_slice,
        reason = "we're checking that we map to valid characters"
    )]
    pub fn parse(input: &'src str) -> Result<Self, ParserError<'src>> {
        let token_iter =
            Token::lexer(input).spanned().map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(err) => (Token::Error(err), span.into()),
            });

        let token_stream = Stream::from_iter(token_iter)
            .map((0..input.len()).into(), |(t, s): (_, _)| (t, s));

        parser()
            .parse(token_stream)
            .into_result()
            .map(Self::normalize)
            .map_err(|errs| {
                let err =
                    errs.first().expect("there should be at least one error");

                let span = err.span().into_range();

                let position = span.start;

                let start =
                    input.char_indices().nth(span.start).map_or(0, |(i, _)| i);
                let end = input
                    .char_indices()
                    .nth(span.end)
                    .map_or(input.len(), |(i, _)| i);

                let message = match err.found() {
                    None => "Unexpected end of input".into(),
                    Some(Token::Error(LexingError::UnexpectedChar)) => {
                        format!(
                            "Unexpected character: `{}`",
                            &input[start..end]
                        )
                    },
                    Some(Token::Error(LexingError::InvalidNumber(err))) => {
                        err.to_string()
                    },
                    Some(_) => {
                        format!("Unexpected token: `{}`", &input[start..end])
                    },
                };

                let start = start.saturating_sub(ParserError::CONTEXT_SIZE);
                let end = end
                    .saturating_add(ParserError::CONTEXT_SIZE)
                    .min(input.len());

                ParserError {
                    message,
                    near: &input[start..end],
                    position,
                }
            })
    }
}

fn parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Expression<'src>, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    recursive(|expr| {
        let literal = select! {
            Token::Number(n) => Literal::Number(Some(n)),
            Token::String(s) => Literal::String(Some(s.into())),
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
            .map(|((field, op), value)| Expression::Comparison {
                field,
                op,
                value,
            });

        let atom = comparison
            .or(expr.delimited_by(just(Token::LParen), just(Token::RParen)))
            .map(|e| Expression::Parens(Box::new(e)));

        let unary = just(Token::Not)
            .repeated()
            .foldr(atom, |_op, rhs| Expression::Not(Box::new(rhs)));

        let product = unary.clone().foldl(
            just(Token::And).to(Expression::And).then(unary).repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        product.clone().foldl(
            just(Token::Or).to(Expression::Or).then(product).repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        )
    })
}
