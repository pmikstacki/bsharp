use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AssemblyHandle(pub usize);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TypeHandle {
    pub assembly: AssemblyHandle,
    pub fullname: String,
}

impl fmt::Display for TypeHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fullname)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MethodHandle {
    pub assembly: AssemblyHandle,
    pub owner_fqn: String,
    pub ordinal: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum TypeSig {
    Primitive(&'static str),
    Named(String),
    ByRef(Box<TypeSig>),
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MethodSig {
    pub has_this: bool,
    pub ret: TypeSig,
    pub params: Vec<TypeSig>,
}
