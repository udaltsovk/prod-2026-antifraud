use crate::Expr;

impl Expr<'_> {
    fn precedence(&self) -> u8 {
        match self {
            Expr::Or(_, _) => 1,
            Expr::And(_, _) => 2,
            Expr::Not(_) => 3,
            Expr::Comparison {
                ..
            } => 4,
            Expr::Parens(inner) => inner.precedence(),
        }
    }

    #[must_use]
    pub fn normalize(self) -> Self {
        self.normalize_with_precedence(0)
    }

    fn normalize_with_precedence(self, parent_prec: u8) -> Self {
        match self {
            Expr::Parens(inner) => {
                let inner = inner.normalize_with_precedence(parent_prec);

                if inner.precedence() < parent_prec {
                    Expr::Parens(Box::new(inner))
                } else {
                    inner
                }
            },

            Expr::And(a, b) => {
                let prec = 2;
                let a = a.normalize_with_precedence(prec);
                let b = b.normalize_with_precedence(prec);

                Expr::And(Box::new(a), Box::new(b))
            },

            Expr::Or(a, b) => {
                let prec = 1;
                let a = a.normalize_with_precedence(prec);
                let b = b.normalize_with_precedence(prec);

                Expr::Or(Box::new(a), Box::new(b))
            },

            Expr::Not(e) => {
                let prec = 3;
                let e = e.normalize_with_precedence(prec);

                Expr::Not(Box::new(e))
            },

            other => other,
        }
    }
}
