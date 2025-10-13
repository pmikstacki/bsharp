use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use bsharp_il::{DotscopeProvider, TypeHandle};

#[derive(Debug, Clone)]
pub struct ExternalAssemblyIndex {
    pub assemblies: Vec<AssemblyRecord>,
}

#[derive(Debug, Clone)]
pub struct AssemblyRecord {
    pub path: PathBuf,
    pub type_count: usize,
}

#[derive(Debug, Default, Clone)]
pub struct IlTypeIndex {
    pub by_fqn: HashMap<String, TypeHandle>,
}

#[derive(Debug, Default, Clone)]
pub struct TypeEnvironment {
    pub primitive_aliases: HashMap<String, String>,
}

impl TypeEnvironment {
    pub fn with_defaults() -> Self {
        let mut map = HashMap::new();
        map.insert("void".to_string(), "System.Void".to_string());
        map.insert("bool".to_string(), "System.Boolean".to_string());
        map.insert("char".to_string(), "System.Char".to_string());
        map.insert("sbyte".to_string(), "System.SByte".to_string());
        map.insert("byte".to_string(), "System.Byte".to_string());
        map.insert("short".to_string(), "System.Int16".to_string());
        map.insert("ushort".to_string(), "System.UInt16".to_string());
        map.insert("int".to_string(), "System.Int32".to_string());
        map.insert("uint".to_string(), "System.UInt32".to_string());
        map.insert("long".to_string(), "System.Int64".to_string());
        map.insert("ulong".to_string(), "System.UInt64".to_string());
        map.insert("float".to_string(), "System.Single".to_string());
        map.insert("double".to_string(), "System.Double".to_string());
        map.insert("nint".to_string(), "System.IntPtr".to_string());
        map.insert("nuint".to_string(), "System.UIntPtr".to_string());
        map.insert("object".to_string(), "System.Object".to_string());
        map.insert("string".to_string(), "System.String".to_string());
        Self {
            primitive_aliases: map,
        }
    }
}

#[derive(Clone)]
pub struct IlProviderArtifact(pub Arc<DotscopeProvider>);
