use std::collections::HashMap;

use crate::Literal;

trait ContextField<'src> {
    fn to_literal(self) -> Literal<'src>;
}
impl<'src> ContextField<'src> for Option<f64> {
    fn to_literal(self) -> Literal<'src> {
        Literal::Number(self)
    }
}
impl<'src> ContextField<'src> for Option<&'src str> {
    fn to_literal(self) -> Literal<'src> {
        Literal::String(self)
    }
}

pub struct ContextBuilder<'src>(HashMap<&'src str, Literal<'src>>);

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
        Option<T>: ContextField<'src>,
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
pub struct Context<'src>(HashMap<&'src str, Literal<'src>>);

impl<'src> Context<'src> {
    pub(crate) fn get_field(&self, field: &'src str) -> Option<&Literal<'src>> {
        self.0.get(field)
    }
}
