#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueKind {
    Int32,
    Bool,
    String,
    ObjectRef,
    Null,
}
