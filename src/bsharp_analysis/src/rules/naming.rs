use crate::framework::{AnalysisSession, Rule, RuleSet};
use crate::framework::NodeRef;
use crate::syntax::ast::TopLevelDeclaration;
use crate::{diag, DiagnosticCode, rule, ruleset};
use bsharp_syntax::declarations::ClassBodyDeclaration;
use bsharp_syntax::declarations::Modifier::Const;

fn ident_text(id: &crate::syntax::Identifier) -> String {
    match id {
        crate::syntax::Identifier::Simple(s) => s.clone(),
        crate::syntax::Identifier::QualifiedIdentifier(parts) => parts.join("."),
        crate::syntax::Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
    }
}

fn is_pascal_case(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_uppercase() => {}
        _ => return false,
    }
    // Don't allow underscores for basic style rule
    !name.contains('_')
}

fn is_camel_case(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_lowercase() => {}
        _ => return false,
    }
    !name.contains('_')
}

fn is_upper_case_constant(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    name.chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

fn is_interface_ipascal_case(name: &str) -> bool {
    let mut chars = name.chars();
    match (chars.next(), chars.next()) {
        (Some('I'), Some(c2)) if c2.is_uppercase() => {}
        _ => return false,
    }
    !name.contains('_')
}

rule! {
    PropertyPascalCase: "naming.property_pascal_case", "Naming", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(c) = decl {
                for m in &c.body_declarations {
                    if let ClassBodyDeclaration::Property(p) = m {
                        let name_owned = ident_text(&p.name);
                        let name = name_owned.as_str();
                        if !is_pascal_case(name) {
                            diag!(
                                session,
                                DiagnosticCode::BSW02002,
                                at p,
                                msg: format!("Property '{}' should be PascalCase", name)
                            );
                        }
                    }
                }
            }
        }
    },
    FieldCamelOrConstUpper: "naming.field_camel_or_const_upper", "Naming", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(c) = decl {
                for m in &c.body_declarations {
                    if let ClassBodyDeclaration::Field(f) = m {
                        let name_owned = ident_text(&f.name);
                        let name = name_owned.as_str();
                        let is_const = f.modifiers.iter().any(|m| m == &Const);

                        if is_const {
                            if !is_upper_case_constant(name) {
                                diag!(
                                    session,
                                    DiagnosticCode::BSW02002,
                                    at f,
                                    msg: format!("Constant '{}' should be UPPER_CASE", name)
                                );
                            }
                        } else if !is_camel_case(name) {
                            diag!(
                                session,
                                DiagnosticCode::BSW02002,
                                at f,
                                msg: format!("Field '{}' should be camelCase", name)
                            );
                        }
                    }
                }
            }
        }
    },
    ParameterCamelCase: "naming.parameter_camel_case", "Naming", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(c) = decl {
                for m in &c.body_declarations {
                    match m {
                        ClassBodyDeclaration::Method(md) => {
                            for p in &md.parameters {
                                let name_owned = ident_text(&p.name);
                                let name = name_owned.as_str();
                                if !is_camel_case(name) {
                                    diag!(
                                        session,
                                        DiagnosticCode::BSW02002,
                                        at p,
                                        msg: format!("Parameter '{}' should be camelCase", name)
                                    );
                                }
                            }
                        }
                        ClassBodyDeclaration::Constructor(ctor) => {
                            for p in &ctor.parameters {
                                let name_owned = ident_text(&p.name);
                                let name = name_owned.as_str();
                                if !is_camel_case(name) {
                                    diag!(
                                        session,
                                        DiagnosticCode::BSW02002,
                                        at p,
                                        msg: format!("Parameter '{}' should be camelCase", name)
                                    );
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    },
    ClassPascalCase: "naming.class_pascal_case", "Naming", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(c) = decl {
                let name_owned = ident_text(&c.name);
                let name = name_owned.as_str();
                if !is_pascal_case(name) {
                    diag!(
                        session,
                        DiagnosticCode::BSW02002,
                        at c,
                        msg: format!("Type '{}' should be PascalCase", name)
                    );
                }
            }
        }
    },
    InterfaceIPascalCase: "naming.interface_ipascal_case", "Naming", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Interface(i) = decl {
                let name_owned = ident_text(&i.name);
                let name = name_owned.as_str();
                if !is_interface_ipascal_case(name) {
                    diag!(
                        session,
                        DiagnosticCode::BSW02002,
                        at i,
                        msg: format!("Interface '{}' should follow I* PascalCase", name)
                    );
                }
            }
        }
    },
    MethodPascalCase: "naming.method_pascal_case", "Naming", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(c) = decl {
                let _class_name = ident_text(&c.name);
                for m in &c.body_declarations {
                    if let ClassBodyDeclaration::Method(md) = m {
                        let name_owned = ident_text(&md.name);
                        let name = name_owned.as_str();
                        if !is_pascal_case(name) {
                            diag!(
                                session,
                                DiagnosticCode::BSW02002,
                                at md,
                                msg: format!("Method '{}' should be PascalCase", name)
                            );
                        }
                    }
                }
            }
        }
    }
}

ruleset! {
    naming: PropertyPascalCase, FieldCamelOrConstUpper, ParameterCamelCase, ClassPascalCase, InterfaceIPascalCase, MethodPascalCase
}
