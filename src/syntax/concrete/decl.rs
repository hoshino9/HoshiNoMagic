use crate::syntax::concrete::{Expr, Type, Stmt};
use crate::syntax::token::SourceRange;

pub enum Decl {
    BoxDecl(BoxDecl),
    MagicDecl(MagicDecl)
}

pub trait DeclVisitor {
    type DeclResult;

    fn visit_decl(&mut self, decl: &Decl) -> Self::DeclResult {
        match decl {
            Decl::BoxDecl(box_decl) => self.visit_box_decl(box_decl),
            Decl::MagicDecl(magic_decl) => self.visit_magic_decl(magic_decl)
        }
    }

    fn visit_box_decl(&mut self, box_decl: &BoxDecl) -> Self::DeclResult;
    fn visit_magic_decl(&mut self, magic_decl: &MagicDecl) -> Self::DeclResult;
}

pub struct BoxDecl {
    pub box_name: String,
    pub box_type: Option<Box<Type>>,
    pub box_kwd_range: SourceRange,
    pub box_name_range: SourceRange,
    pub eq_range: Option<SourceRange>,
    pub init_expr: Option<Box<Expr>>
}

impl BoxDecl {
    pub fn new(box_name: impl ToString,
               box_type: Option<Box<Type>>,
               box_kwd_range: SourceRange,
               box_name_range: SourceRange) -> Self {
        Self {
            box_name: box_name.to_string(),
            box_type,
            box_kwd_range,
            box_name_range,
            eq_range: None,
            init_expr: None
        }
    }

    pub fn with_init_expr(box_name: impl ToString,
                          box_type: Option<Box<Type>>,
                          box_kwd_range: SourceRange,
                          box_name_range: SourceRange,
                          eq_range: SourceRange,
                          init_expr: Box<Expr>) -> Self {
        Self {
            box_name: box_name.to_string(),
            box_type,
            box_kwd_range,
            box_name_range,
            eq_range: Some(eq_range),
            init_expr: Some(init_expr)
        }
    }
}

pub struct MagicDecl {
    pub magic_name: String,
    pub parameters: Vec<Vec<(String, Box<Type>)>>,
    pub magic_kwd_range: SourceRange,
    pub magic_name_range: SourceRange,
    pub left_bracket_pos: SourceRange,
    pub right_bracket_pos: SourceRange,
    pub body: Option<Box<Stmt>>
}

impl MagicDecl {
    pub fn decl(magic_name: impl ToString,
                parameters: Vec<Vec<(String, Box<Type>)>>,
                magic_kwd_range: SourceRange,
                magic_name_range: SourceRange,
                left_bracket_pos: SourceRange,
                right_bracket_pos: SourceRange) -> Self {
        Self {
            magic_name: magic_name.to_string(),
            parameters,
            magic_kwd_range,
            magic_name_range,
            left_bracket_pos,
            right_bracket_pos,
            body: None
        }
    }

    pub fn def(magic_name: impl ToString,
               parameters: Vec<Vec<(String, Box<Type>)>>,
               magic_kwd_range: SourceRange,
               magic_name_range: SourceRange,
               left_bracket_pos: SourceRange,
               right_bracket_pos: SourceRange,
               body: Box<Stmt>) -> Self {
        Self {
            magic_name: magic_name.to_string(),
            parameters,
            magic_kwd_range,
            magic_name_range,
            left_bracket_pos,
            right_bracket_pos,
            body: Some(body)
        }
    }
}
