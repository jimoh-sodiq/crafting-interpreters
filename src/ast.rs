use crate::token::Token;

struct Expr {}

struct Binary {
    left: Expr,
    operator: Token,
    right: Expr,
}

impl Binary {
    fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}