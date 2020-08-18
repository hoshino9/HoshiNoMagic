pub enum Stmt {
}

pub trait StmtVisitor {
    type StmtResult;

    fn visit_stmt(&mut self, stmt: &Stmt) -> Self::StmtResult {
        match stmt {
            _ => unimplemented!()
        }
    }
}
