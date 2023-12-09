use crate::error::*;
use crate::token::*;


pub enum Expr {
Binary(BinaryExpr),
Grouping(GroupingExpr),
Literal(LiteralExpr),
Unary(UnaryExpr),
 }

 impl Expr{
    pub fn accept<T>(&self, expr_visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError>{
        match self {
            Expr::Binary(expr) => expr.accept(expr_visitor),
            Expr::Grouping(expr) => expr.accept(expr_visitor),
            Expr::Literal(expr) => expr.accept(expr_visitor),
            Expr::Unary(expr) => expr.accept(expr_visitor),
        
        }
    }
 }



pub struct BinaryExpr {
pub left:  Box<Expr>,
pub operator:  Token,
pub right:  Box<Expr>,
}


pub struct GroupingExpr {
pub expression:  Box<Expr>,
}


pub struct LiteralExpr {
pub value:  Option<Object>,
}


pub struct UnaryExpr {
pub operator:  Token,
pub right:  Box<Expr>,
}

pub trait ExprVisitor<T>{
fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, LoxError>;
fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, LoxError> ;
fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, LoxError>;
fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, LoxError>;
}

impl BinaryExpr {
 fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
 visitor.visit_binary_expr(&self)
}
 }

impl GroupingExpr {
 fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
 visitor.visit_grouping_expr(&self)
}
 }

impl LiteralExpr {
 fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
 visitor.visit_literal_expr(&self)
}
 }

impl UnaryExpr {
 fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
 visitor.visit_unary_expr(&self)
}
 }

