use crate::node::ast_node::AstNode;
use crate::node::dyn_node_ref::DynNodeRef;

/// Generic field visitor: default is no-op so derives can call it on any field type safely.
pub trait FieldVisit {
    fn visit_field<'a>(&'a self, _push: &mut dyn FnMut(DynNodeRef<'a>)) {}
}

// Blanket only for concrete AstNode types; containers are handled explicitly in the derive.
impl<T: AstNode> FieldVisit for T {
    fn visit_field<'a>(&'a self, push: &mut dyn FnMut(DynNodeRef<'a>)) {
        push(DynNodeRef(self));
    }
}

// No-op visitors for common scalar field types used in the AST
impl FieldVisit for String {}
impl FieldVisit for &str {}
impl FieldVisit for bool {}
impl FieldVisit for char {}
impl FieldVisit for u8 {}
impl FieldVisit for u16 {}
impl FieldVisit for u32 {}
impl FieldVisit for u64 {}
impl FieldVisit for usize {}
impl FieldVisit for i8 {}
impl FieldVisit for i16 {}
impl FieldVisit for i32 {}
impl FieldVisit for i64 {}
impl FieldVisit for isize {}
impl FieldVisit for f32 {}
impl FieldVisit for f64 {}
