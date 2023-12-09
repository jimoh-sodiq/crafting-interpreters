use crate::error::*;
use crate::expr::*;
// use token::*;


struct AstPrinter;

impl AstPrinter {
    fn print(&self, expr: &Expr) -> Result<String, LoxError> {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, expr: &[&Box<Expr>])-> Result<String, LoxError> {
        Ok(format!("({} {})", name, expr.iter().map(|e| e.accept(self)).collect::<Result<String, LoxError>>()?))
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, LoxError> {
        self.parenthesize(expr.operator.lexeme.as_str(), &vec![&expr.left, &expr.right])
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, LoxError> {
        self.parenthesize("group", &vec![&expr.expression])
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, LoxError> {
        if let Some(value) = &expr.value {
            Ok(value.to_string() )
        } else { 
           Ok( "nil".to_string())
        }
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, LoxError> {
        self.parenthesize(expr.operator.lexeme.as_str(), &vec![&expr.right])
    }
}

fn main() {
    println!("Hello, world!");
}
