use crate::syntax::token::{TokenKind, SourceRange};
use crate::syntax::concrete::Stmt;

pub enum Expr {
    MagicSymRefExpr(MagicSymRefExpr),
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    InvokeLikeExpr(InvokeLikeExpr),
    BlockExpr(BlockExpr),
}

pub trait ExprVisitor {
    type ExprResult;

    fn visit_expr(&mut self, expr: &Expr) -> Self::ExprResult {
        match expr {
            Expr::MagicSymRefExpr(magic_sym_ref_expr) =>
                self.visit_magic_sym_ref_expr(magic_sym_ref_expr),
            Expr::BinaryExpr(binary_expr) => self.visit_binary_expr(binary_expr),
            Expr::UnaryExpr(unary_expr) => self.visit_unary_expr(unary_expr),
            Expr::InvokeLikeExpr(invoke_like_expr) =>
                self.visit_invoke_like_expr(invoke_like_expr),
            Expr::BlockExpr(block_expr) => self.visit_block_expr(block_expr)
        }
    }

    fn visit_magic_sym_ref_expr(&mut self,
                                magic_sym_ref_expr: &MagicSymRefExpr) -> Self::ExprResult;
    fn visit_binary_expr(&mut self, binary_expr: &BinaryExpr) -> Self::ExprResult;
    fn visit_unary_expr(&mut self, unary_expr: &UnaryExpr) -> Self::ExprResult;
    fn visit_invoke_like_expr(&mut self, invoke_like_expr: &InvokeLikeExpr) -> Self::ExprResult;
    fn visit_block_expr(&mut self, block_expr: &BlockExpr) -> Self::ExprResult;
}

pub struct MagicSymRefExpr {
    magic_sym: String,
    magic_sym_range: SourceRange
}

impl MagicSymRefExpr {
    pub fn new(magic_sym: String, magic_sym_range: SourceRange) -> Self {
        Self {
            magic_sym,
            magic_sym_range
        }
    }
}

pub struct BinaryExpr {
    operator: TokenKind,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    operator_position: SourceRange
}

impl BinaryExpr {
    pub fn new(
        operator: TokenKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        operator_position: SourceRange
    ) -> Self {
        Self { operator, lhs, rhs, operator_position }
    }
}

pub struct UnaryExpr {
    operator: TokenKind,
    base: Box<Expr>,
    operator_position: SourceRange
}

impl UnaryExpr {
    pub fn new(
        operator: TokenKind,
        base: Box<Expr>,
        operator_position: SourceRange
    ) -> Self {
        Self { operator, base, operator_position }
    }
}

pub struct InvokeLikeExpr {
    base: Box<Expr>,
    params: Vec<Vec<Box<Expr>>>,
    left_bracket_pos: SourceRange,
    right_bracket_pos: SourceRange
}

impl InvokeLikeExpr {
    pub fn new(
        base: Box<Expr>,
        params: Vec<Vec<Box<Expr>>>,
        left_bracket_pos: SourceRange,
        right_bracket_pos: SourceRange
    ) -> Self {
        Self { base, params, left_bracket_pos, right_bracket_pos }
    }
}

pub struct BlockExpr {
    stmts: Vec<Stmt>,
    last_expr: Option<Box<Expr>>,
    left_bracket_pos: SourceRange,
    right_bracket_pos: SourceRange
}

impl BlockExpr {
    pub fn new(
        stmts: Vec<Stmt>,
        last_expr: Option<Box<Expr>>,
        left_bracket_pos: SourceRange,
        right_bracket_pos: SourceRange
    ) -> Self {
        Self {
            stmts,
            last_expr,
            left_bracket_pos,
            right_bracket_pos
        }
    }
}
