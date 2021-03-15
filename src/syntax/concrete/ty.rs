use crate::syntax::token::{TokenKind, SourceRange};

pub enum Type {
    BuiltinType(BuiltinType),
    RefType(RefType),
    ArrayType(ArrayType)
}

pub trait TypeVisitor {
    type TypeResult;

    fn visit_type(&mut self, ty: &Type) -> Self::TypeResult {
        match ty {
            Type::BuiltinType(builtin_type) => self.visit_builtin_type(builtin_type),
            Type::RefType(ref_type) => self.visit_ref_type(ref_type),
            Type::ArrayType(array_type) => self.visit_array_type(array_type)
        }
    }

    fn visit_builtin_type(&mut self, builtin_type: &BuiltinType) -> Self::TypeResult;
    fn visit_ref_type(&mut self, ref_type: &RefType) -> Self::TypeResult;
    fn visit_array_type(&mut self, arr_type: &ArrayType) -> Self::TypeResult;
}

pub struct BuiltinType {
    pub type_token_kind: TokenKind,
    pub source_range: SourceRange
}

impl BuiltinType {
    pub fn new(type_token_kind: TokenKind, source_range: SourceRange) -> Self {
        Self {
            type_token_kind, source_range
        }
    }
}

pub struct RefType {
    pub base_type: Box<Type>,
    pub wave_line_position: SourceRange
}

impl RefType {
    pub fn new(base_type: Box<Type>, wave_line_position: SourceRange) -> Self {
        Self {
            base_type, wave_line_position
        }
    }
}

pub struct ArrayType {
    pub base_type: Box<Type>,
    pub aster_position: SourceRange
}

impl ArrayType {
    pub fn new(base_type: Box<Type>, aster_position: SourceRange) -> Self {
        Self {
            base_type, aster_position
        }
    }
}
