use crate::syntax::concrete::{Expr, Decl};
use crate::syntax::token::SourceRange;

pub enum Stmt {
    ExprStmt(ExprStmt),
    DeclStmt(DeclStmt),
    BlockStmt(BlockStmt)
}

pub trait StmtVisitor {
    type StmtResult;

    fn visit_stmt(&mut self, stmt: &Stmt) -> Self::StmtResult {
        match stmt {
            Stmt::ExprStmt(expr_stmt) => self.visit_expr_stmt(expr_stmt),
            Stmt::DeclStmt(decl_stmt) => self.visit_decl_stmt(decl_stmt),
            Stmt::BlockStmt(block_stmt) => self.visit_block_stmt(block_stmt)
        }
    }

    fn visit_expr_stmt(&mut self, expr_stmt: &ExprStmt) -> Self::StmtResult;
    fn visit_decl_stmt(&mut self, decl_stmt: &DeclStmt) -> Self::StmtResult;
    fn visit_block_stmt(&mut self, block_stmt: &BlockStmt) -> Self::StmtResult;
}

pub struct ExprStmt {
    expr: Box<Expr>
}

pub struct DeclStmt {
    decl: Box<Decl>
}

pub struct BlockStmt {
    stmts: Vec<Stmt>,
    left_brace_position: SourceRange,
    right_brace_position: SourceRange
}
