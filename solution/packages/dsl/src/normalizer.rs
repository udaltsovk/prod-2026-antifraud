use crate::Expression;

impl Expression<'_> {
    fn precedence(&self) -> u8 {
        match self {
            Expression::Or(_, _) => 1,
            Expression::And(_, _) => 2,
            Expression::Not(_) => 3,
            Expression::Comparison {
                ..
            } => 4,
            Expression::Parens(inner) => inner.precedence(),
        }
    }

    #[must_use]
    pub fn normalize(self) -> Self {
        self.normalize_with_precedence(0)
    }

    fn normalize_with_precedence(self, parent_prec: u8) -> Self {
        match self {
            Expression::Parens(inner) => {
                let inner = inner.normalize_with_precedence(parent_prec);

                if inner.precedence() < parent_prec {
                    Expression::Parens(Box::new(inner))
                } else {
                    inner
                }
            },

            Expression::And(a, b) => {
                let prec = 2;
                let a = a.normalize_with_precedence(prec);
                let b = b.normalize_with_precedence(prec);

                Expression::And(Box::new(a), Box::new(b))
            },

            Expression::Or(a, b) => {
                let prec = 1;
                let a = a.normalize_with_precedence(prec);
                let b = b.normalize_with_precedence(prec);

                Expression::Or(Box::new(a), Box::new(b))
            },

            Expression::Not(e) => {
                let prec = 3;
                let e = e.normalize_with_precedence(prec);

                Expression::Not(Box::new(e))
            },

            other => other,
        }
    }
}
