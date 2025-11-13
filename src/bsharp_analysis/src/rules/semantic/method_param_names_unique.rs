use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, StructBodyDeclaration, InterfaceBodyDeclaration};
use std::collections::HashSet;
use super::utils::ident_text;

rule! {
    MethodParamNamesUnique: "semantic.method.param_names_unique", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else { return; };
        for decl in &cu.declarations {
            match decl {
                TopLevelDeclaration::Class(class) => {
                    for member in &class.body_declarations {
                        if let ClassBodyDeclaration::Method(m) = member {
                            let mut seen = HashSet::new();
                            let mut dup = false;
                            for p in &m.parameters {
                                let name = ident_text(&p.name);
                                if !seen.insert(name) { dup = true; break; }
                            }
                            if dup { diag!(session, DiagnosticCode::BSE02010, at m); }
                        }
                    }
                }
                TopLevelDeclaration::Struct(s) => {
                    for member in &s.body_declarations {
                        if let StructBodyDeclaration::Method(m) = member {
                            let mut seen = HashSet::new();
                            let mut dup = false;
                            for p in &m.parameters {
                                let name = ident_text(&p.name);
                                if !seen.insert(name) { dup = true; break; }
                            }
                            if dup { diag!(session, DiagnosticCode::BSE02010, at m); }
                        }
                    }
                }
                TopLevelDeclaration::Interface(i) => {
                    for member in &i.body_declarations {
                        if let InterfaceBodyDeclaration::Method(m) = member {
                            let mut seen = HashSet::new();
                            let mut dup = false;
                            for p in &m.parameters {
                                let name = ident_text(&p.name);
                                if !seen.insert(name) { dup = true; break; }
                            }
                            if dup { diag!(session, DiagnosticCode::BSE02010, at m); }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
