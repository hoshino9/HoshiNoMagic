pub enum Expr {
}

pub trait ExprVisitor {
    type ExprResult;

    fn visit_stmt(&mut self, stmt: &Expr) -> Self::ExprResult {
        match stmt {
            _ => unimplemented!()
        }
    }
}
