use crate::syntax::token::{SourceRange, TokenKind};

pub trait DeclVisitor {
    type DeclResult;

    fn visit_var_decl(&mut self, var_decl: &VarDecl) -> Self::DeclResult;
}

pub trait ExprVisitor {
    type ExprResult;
}

pub trait TypeVisitor {
    type TypeResult;
}

pub trait StmtVisitor {
    type StmtResult;
}

pub trait ConcreteVisitor : DeclVisitor + ExprVisitor + TypeVisitor + StmtVisitor {
}

pub trait ConcreteNode {}

pub trait Decl : ConcreteNode {
    fn accept<V: DeclVisitor>(&self, visitor: &mut V) -> V::DeclResult;
}

pub trait Expr : ConcreteNode {
    fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::ExprResult;
}

pub trait Type : ConcreteNode {
    fn accept<V: TypeVisitor>(&self, visitor: &mut V) -> V::TypeResult;
}

pub trait Stmt : ConcreteNode {
    fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::StmtResult;
}

pub struct VarDecl {
    pub var_name: String,
    pub var_type: Option<Box<dyn Type>>,
    pub box_range: SourceRange,
    pub var_name_range: SourceRange,
    pub eq_range: SourceRange,
    pub init_expr: Option<Box<dyn Expr>>
}

impl ConcreteNode for VarDecl {}

impl Decl for VarDecl {
    fn accept<V: DeclVisitor>(&self, visitor: &mut V) -> <V as DeclVisitor>::DeclResult {
        visitor.visit_var_decl(self)
    }
}

