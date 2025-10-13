use crate::loader::IlProvider;
use crate::model::{MethodHandle, MethodSig, TypeHandle, TypeSig};

pub fn resolve_method<P: IlProvider>(
    provider: &P,
    owner: &TypeHandle,
    name: &str,
    arg_types: &[TypeSig],
) -> Option<MethodHandle> {
    let cands = provider.list_methods(owner);
    for mh in cands {
        if let Some(mname) = provider.method_name(&mh) {
            if mname != name {
                continue;
            }
            if let Ok(sig) = provider.method_sig(&mh) {
                if arity_matches(&sig, arg_types) && params_match(&sig, arg_types) {
                    return Some(mh);
                }
            }
        }
    }
    None
}

fn arity_matches(sig: &MethodSig, args: &[TypeSig]) -> bool {
    sig.params.len() == args.len()
}

fn params_match(sig: &MethodSig, args: &[TypeSig]) -> bool {
    sig.params
        .iter()
        .zip(args.iter())
        .all(|(p, a)| type_eq(p, a))
}

fn type_eq(a: &TypeSig, b: &TypeSig) -> bool {
    match (a, b) {
        (TypeSig::ByRef(x), y) => type_eq(x, y),
        (x, TypeSig::ByRef(y)) => type_eq(x, y),
        (TypeSig::Primitive(pa), TypeSig::Primitive(pb)) => pa == pb,
        (TypeSig::Named(na), TypeSig::Named(nb)) => na == nb,
        _ => false,
    }
}
