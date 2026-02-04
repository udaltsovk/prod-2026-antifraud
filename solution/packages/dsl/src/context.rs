use std::{collections::HashMap, sync::Arc};

use crate::Literal;

trait ContextField {
    fn to_literal(self) -> Literal;
}
impl ContextField for Option<f64> {
    fn to_literal(self) -> Literal {
        Literal::Number(self)
    }
}

impl ContextField for Option<&str> {
    fn to_literal(self) -> Literal {
        Literal::String(self.map(Arc::from))
    }
}

impl ContextField for Option<String> {
    fn to_literal(self) -> Literal {
        Literal::String(self.map(Arc::from))
    }
}

impl ContextField for Option<&String> {
    fn to_literal(self) -> Literal {
        Literal::String(self.map(String::as_str).map(Arc::from))
    }
}

pub struct ContextBuilder<'src>(HashMap<&'src str, Literal>);

impl<'src> ContextBuilder<'src> {
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[expect(
        private_bounds,
        reason = "we must ensure that only we control field values"
    )]
    #[must_use]
    pub fn add_field<T>(
        mut self,
        field_name: &'static str,
        field_value: Option<T>,
    ) -> Self
    where
        Option<T>: ContextField,
    {
        self.0.insert(field_name, field_value.to_literal());
        self
    }

    #[must_use]
    pub fn build(self) -> Context<'src> {
        Context(self.0)
    }
}

impl Default for ContextBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Context<'src>(HashMap<&'src str, Literal>);

impl<'src> Context<'src> {
    #[must_use]
    pub fn builder() -> ContextBuilder<'src> {
        ContextBuilder::new()
    }

    pub(crate) fn get_field(&self, field: &'src str) -> Option<&Literal> {
        self.0.get(field)
    }
}
