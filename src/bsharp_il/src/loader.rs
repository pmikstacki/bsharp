use crate::errors::{IlError, Result};
use crate::model::{AssemblyHandle, MethodHandle, MethodSig, TypeHandle, TypeSig};
use dotscope::metadata::cilobject::CilObject;
use dotscope::metadata::signatures::{SignatureMethod, TypeSignature};
use dotscope::metadata::token::Token;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub trait IlProvider {
    fn load_assembly(&mut self, path: &Path) -> Result<AssemblyHandle>;
    fn list_types(&self, asm: AssemblyHandle) -> Vec<TypeHandle>;
    fn find_type(&self, fqn: &str) -> Option<TypeHandle>;
    fn list_methods(&self, ty: &TypeHandle) -> Vec<MethodHandle>;
    fn method_name(&self, method: &MethodHandle) -> Option<String>;
    fn method_sig(&self, method: &MethodHandle) -> Result<MethodSig>;
}

pub struct DotscopeProvider {
    assemblies: Vec<AssemblyEntry>,
    by_path: HashMap<PathBuf, AssemblyHandle>,
    global_type_index: HashMap<String, TypeHandle>,
}

struct AssemblyEntry {
    obj: CilObject,
    type_index: HashMap<String, ()>,
}

impl DotscopeProvider {
    pub fn new() -> Self {
        Self {
            assemblies: Vec::new(),
            by_path: HashMap::new(),
            global_type_index: HashMap::new(),
        }
    }
}

impl Default for DotscopeProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl IlProvider for DotscopeProvider {
    fn load_assembly(&mut self, path: &Path) -> Result<AssemblyHandle> {
        let canon = path.canonicalize().map_err(IlError::from)?;
        if let Some(h) = self.by_path.get(&canon).copied() {
            return Ok(h);
        }
        let obj = CilObject::from_file(&canon).map_err(IlError::from)?;
        let asm_handle = AssemblyHandle(self.assemblies.len());

        // Build per-assembly type index and global index
        let mut type_index = HashMap::new();
        let types = obj.types();
        for entry in types.iter() {
            let t = entry.value();
            let ns = t.namespace.clone();
            let name = t.name.clone();
            let fqn = if ns.is_empty() {
                name.clone()
            } else {
                format!("{}.{}", ns, name)
            };
            type_index.insert(fqn.clone(), ());
            self.global_type_index
                .entry(fqn.clone())
                .or_insert(TypeHandle {
                    assembly: asm_handle,
                    fullname: fqn,
                });
        }

        self.assemblies.push(AssemblyEntry { obj, type_index });
        self.by_path.insert(canon, asm_handle);
        Ok(asm_handle)
    }

    fn list_types(&self, asm: AssemblyHandle) -> Vec<TypeHandle> {
        let Some(entry) = self.assemblies.get(asm.0) else {
            return Vec::new();
        };
        entry
            .type_index
            .keys()
            .map(|fqn| TypeHandle {
                assembly: asm,
                fullname: fqn.clone(),
            })
            .collect()
    }

    fn find_type(&self, fqn: &str) -> Option<TypeHandle> {
        self.global_type_index.get(fqn).cloned()
    }

    fn list_methods(&self, ty: &TypeHandle) -> Vec<MethodHandle> {
        let Some(entry) = self.assemblies.get(ty.assembly.0) else {
            return Vec::new();
        };
        // naive lookup by scanning types
        let types = entry.obj.types();
        for tentry in types.iter() {
            let t = tentry.value();
            let ns = &t.namespace;
            let name = &t.name;
            let fqn = if ns.is_empty() {
                name.clone()
            } else {
                format!("{}.{}", ns, name)
            };
            if fqn == ty.fullname {
                let mut out = Vec::new();
                for (i, _mref) in t.methods.iter() {
                    out.push(MethodHandle {
                        assembly: ty.assembly,
                        owner_fqn: ty.fullname.clone(),
                        ordinal: i,
                    });
                }
                return out;
            }
        }
        Vec::new()
    }

    fn method_name(&self, method: &MethodHandle) -> Option<String> {
        let entry = self.assemblies.get(method.assembly.0)?;
        let types = entry.obj.types();
        for tentry in types.iter() {
            let t = tentry.value();
            let ns = &t.namespace;
            let name = &t.name;
            let fqn = if ns.is_empty() {
                name.clone()
            } else {
                format!("{}.{}", ns, name)
            };
            if fqn == method.owner_fqn {
                if let Some((_, mref)) = t.methods.iter().find(|(i, _)| *i == method.ordinal) {
                    if let Some(m) = mref.upgrade() {
                        return Some(m.name.clone());
                    }
                }
            }
        }
        None
    }

    fn method_sig(&self, method: &MethodHandle) -> Result<MethodSig> {
        let entry = self
            .assemblies
            .get(method.assembly.0)
            .ok_or_else(|| IlError::NotFound("assembly".into()))?;
        let types = entry.obj.types();
        for tentry in types.iter() {
            let t = tentry.value();
            let ns = &t.namespace;
            let name = &t.name;
            let fqn = if ns.is_empty() {
                name.clone()
            } else {
                format!("{}.{}", ns, name)
            };
            if fqn == method.owner_fqn {
                if let Some((_, mref)) = t.methods.iter().find(|(i, _)| *i == method.ordinal) {
                    if let Some(m) = mref.upgrade() {
                        return Ok(map_signature(&entry.obj, &m.signature));
                    }
                }
            }
        }
        Err(IlError::NotFound("method".into()))
    }
}

fn map_signature(obj: &CilObject, sig: &SignatureMethod) -> MethodSig {
    MethodSig {
        has_this: sig.has_this,
        ret: map_type_signature(obj, &sig.return_type.base),
        params: sig
            .params
            .iter()
            .map(|p| map_type_signature(obj, &p.base))
            .collect(),
    }
}

fn map_type_signature(obj: &CilObject, ts: &TypeSignature) -> TypeSig {
    match ts {
        TypeSignature::Void => TypeSig::Named("System.Void".into()),
        TypeSignature::Boolean => TypeSig::Named("System.Boolean".into()),
        TypeSignature::Char => TypeSig::Named("System.Char".into()),
        TypeSignature::I1 => TypeSig::Named("System.SByte".into()),
        TypeSignature::U1 => TypeSig::Named("System.Byte".into()),
        TypeSignature::I2 => TypeSig::Named("System.Int16".into()),
        TypeSignature::U2 => TypeSig::Named("System.UInt16".into()),
        TypeSignature::I4 => TypeSig::Named("System.Int32".into()),
        TypeSignature::U4 => TypeSig::Named("System.UInt32".into()),
        TypeSignature::I8 => TypeSig::Named("System.Int64".into()),
        TypeSignature::U8 => TypeSig::Named("System.UInt64".into()),
        TypeSignature::R4 => TypeSig::Named("System.Single".into()),
        TypeSignature::R8 => TypeSig::Named("System.Double".into()),
        TypeSignature::String => TypeSig::Named("System.String".into()),
        TypeSignature::Object => TypeSig::Named("System.Object".into()),
        TypeSignature::I => TypeSig::Named("System.IntPtr".into()),
        TypeSignature::U => TypeSig::Named("System.UIntPtr".into()),
        TypeSignature::ByRef(inner) => TypeSig::ByRef(Box::new(map_type_signature(obj, inner))),
        TypeSignature::Class(tok) | TypeSignature::ValueType(tok) => {
            TypeSig::Named(resolve_token_fqn(obj, tok).unwrap_or_else(|| "<unknown>".into()))
        }
        _ => TypeSig::Unknown,
    }
}

fn resolve_token_fqn(obj: &CilObject, tok: &Token) -> Option<String> {
    let types = obj.types();
    for tentry in types.iter() {
        let t = tentry.value();
        if &t.token == tok {
            let ns = &t.namespace;
            let name = &t.name;
            return Some(if ns.is_empty() {
                name.clone()
            } else {
                format!("{}.{}", ns, name)
            });
        }
    }
    None
}
