pub mod error;

pub use crate::error::*;

use doner_parser::{BinaryOp, Expr, Program, Stmt};

pub fn eval(ast: Program) -> Result<i64> {
    let mut stmts = ast.statements.iter();
    while let Some(stmt) = stmts.next() {
        eval_stmt(stmt.clone());
    }
    Ok(0)
}

pub fn eval_stmt(stmt: Stmt) {
    match stmt {
        Stmt::Expr(_) => {}
        Stmt::Print(value) => {
            println!("{}", eval_expr(value));
        }
    }
}

pub fn eval_expr(expr: Expr) -> f64 {
    match expr {
        Expr::Int(i) => i,
        Expr::UnaryNeg(expr) => -eval_expr(*expr),
        Expr::Binary { left, op, right } => match op {
            BinaryOp::Add => eval_expr(*left) + eval_expr(*right),
            BinaryOp::Sub => eval_expr(*left) - eval_expr(*right),
            BinaryOp::Div => eval_expr(*left) / eval_expr(*right),
            BinaryOp::Mul => eval_expr(*left) * eval_expr(*right),
        },
    }
}
