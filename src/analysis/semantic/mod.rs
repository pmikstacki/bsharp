// Semantic analysis module - handles the validation that was removed from the syntax

use crate::analysis::diagnostics::{Diagnostic, DiagnosticCode, DiagnosticCollection};
use crate::syntax::nodes::declarations::{ClassDeclaration, MemberDeclaration, Modifier};
use crate::syntax::ast::CompilationUnit;
use serde::{Deserialize, Serialize};

/// Semantic member type determined by analysis, not parsing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticMemberType {
    Method,
    Constructor,
    Property,
    Field,
    Event,
    Indexer,
    Operator,
    Destructor,
}

/// Result of semantic analysis for a member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberAnalysis {
    pub semantic_type: SemanticMemberType,
    pub is_valid: bool,
    pub diagnostics: DiagnosticCollection,
}

/// Result of semantic analysis for a compilation unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticAnalysisResult {
    pub is_valid: bool,
    pub diagnostics: DiagnosticCollection,
    pub member_analyses: Vec<MemberAnalysis>,
}

/// Semantic analyzer that validates C# language rules
pub struct SemanticAnalyzer {
    diagnostics: DiagnosticCollection,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            diagnostics: DiagnosticCollection::new(),
        }
    }

    /// Analyze a compilation unit for semantic errors
    pub fn analyze(&mut self, compilation_unit: &CompilationUnit) -> SemanticAnalysisResult {
        self.diagnostics = DiagnosticCollection::new();
        let mut member_analyses = Vec::new();

        // Analyze all members in all declarations
        for declaration in &compilation_unit.declarations {
            match declaration {
                crate::syntax::ast::TopLevelDeclaration::Namespace(namespace) => {
                    for ns_member in &namespace.declarations {
                        if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Class(class) = ns_member {
                            let class_analyses = self.analyze_class(class);
                            member_analyses.extend(class_analyses);
                        }
                    }
                }
                crate::syntax::ast::TopLevelDeclaration::Class(class) => {
                    let class_analyses = self.analyze_class(class);
                    member_analyses.extend(class_analyses);
                }
                _ => {}
            }
        }

        SemanticAnalysisResult {
            is_valid: !self.diagnostics.has_errors(),
            diagnostics: self.diagnostics.clone(),
            member_analyses,
        }
    }

    /// Analyze a class and its members
    fn analyze_class(&mut self, class: &ClassDeclaration) -> Vec<MemberAnalysis> {
        let mut analyses = Vec::new();

        for member in &class.body_declarations {
            match member {
                crate::syntax::nodes::declarations::ClassBodyDeclaration::Method(method) => {
                    // Convert method back to unified representation for analysis
                    let unified_member = MemberDeclaration {
                        modifiers: method.modifiers.clone(),
                        return_type: Some(method.return_type.clone()),
                        name: method.name.clone(),
                        type_parameters: method.type_parameters.clone(),
                        parameters: method.parameters.clone(),
                        body: method.body.clone(),
                        constraints: method.constraints.clone(),
                    };
                    analyses.push(self.analyze_member(&unified_member, &class.name.name));
                }
                crate::syntax::nodes::declarations::ClassBodyDeclaration::Constructor(constructor) => {
                    // Convert constructor back to unified representation for analysis
                    let unified_member = MemberDeclaration {
                        modifiers: constructor.modifiers.clone(),
                        return_type: None,
                        name: constructor.name.clone(),
                        type_parameters: None,
                        parameters: constructor.parameters.clone(),
                        body: constructor.body.clone(),
                        constraints: None,
                    };
                    analyses.push(self.analyze_member(&unified_member, &class.name.name));
                }
                _ => {}
            }
        }

        analyses
    }

    /// Analyze a unified member declaration for semantic correctness
    fn analyze_member(&mut self, member: &MemberDeclaration, class_name: &str) -> MemberAnalysis {
        let mut member_diagnostics = DiagnosticCollection::new();
        
        // Determine semantic type based on parser
        let semantic_type = if member.return_type.is_none() {
            SemanticMemberType::Constructor
        } else {
            SemanticMemberType::Method
        };

        // Validate based on semantic type
        match semantic_type {
            SemanticMemberType::Constructor => {
                self.validate_constructor(member, class_name, &mut member_diagnostics);
            }
            SemanticMemberType::Method => {
                self.validate_method(member, &mut member_diagnostics);
            }
            _ => {}
        }

        // Add member diagnostics to global collection
        self.diagnostics.extend(member_diagnostics.clone());

        MemberAnalysis {
            semantic_type,
            is_valid: !member_diagnostics.has_errors(),
            diagnostics: member_diagnostics,
        }
    }

    /// Validate constructor-specific semantic rules
    fn validate_constructor(&self, member: &MemberDeclaration, class_name: &str, diagnostics: &mut DiagnosticCollection) {
        // BSE01001: Constructors cannot be declared async
        if member.modifiers.contains(&Modifier::Async) {
            diagnostics.add(Diagnostic::with_default_message(DiagnosticCode::BSE01001));
        }

        // BSE01002: Constructors cannot have an explicit return type  
        if member.return_type.is_some() {
            diagnostics.add(Diagnostic::with_default_message(DiagnosticCode::BSE01002));
        }

        // BSE01003: Constructors cannot be virtual or abstract
        if member.modifiers.contains(&Modifier::Virtual) || member.modifiers.contains(&Modifier::Abstract) {
            diagnostics.add(Diagnostic::with_default_message(DiagnosticCode::BSE01003));
        }

        // BSE01004: Constructor cannot be both static and instance
        let has_static = member.modifiers.contains(&Modifier::Static);
        let has_instance_modifiers = member.modifiers.iter().any(|m| {
            matches!(m, Modifier::Public | Modifier::Private | Modifier::Protected | Modifier::Internal)
        });
        if has_static && has_instance_modifiers {
            diagnostics.add(Diagnostic::with_default_message(DiagnosticCode::BSE01004));
        }

        // BSE01005: Constructor name must match the containing class name
        if member.name.name != class_name {
            diagnostics.add(Diagnostic::new(
                DiagnosticCode::BSE01005,
                format!("Constructor name '{}' does not match class name '{}'", member.name.name, class_name)
            ));
        }

        // BSE01009: Constructors cannot override other constructors
        if member.modifiers.contains(&Modifier::Override) {
            diagnostics.add(Diagnostic::with_default_message(DiagnosticCode::BSE01009));
        }
    }

    /// Validate method-specific semantic rules
    fn validate_method(&self, member: &MemberDeclaration, diagnostics: &mut DiagnosticCollection) {
        // BSE02001: Abstract methods cannot have a body
        if member.modifiers.contains(&Modifier::Abstract) && member.body.is_some() {
            diagnostics.add(Diagnostic::with_default_message(DiagnosticCode::BSE02001));
        }

        // BSE02002: Non-abstract methods must have a body (in non-interface context)
        // Note: This would need context about whether we're in an interface
        if !member.modifiers.contains(&Modifier::Abstract) && member.body.is_none() {
            // Could be an interface method, so this might not be an error
            // We'll skip this for now as it requires interface context
        }

        // BSE02005: Methods cannot be both virtual and static
        if member.modifiers.contains(&Modifier::Virtual) && member.modifiers.contains(&Modifier::Static) {
            diagnostics.add(Diagnostic::with_default_message(DiagnosticCode::BSE02005));
        }

        // BSE02006: Static methods cannot override
        if member.modifiers.contains(&Modifier::Static) && member.modifiers.contains(&Modifier::Override) {
            diagnostics.add(Diagnostic::with_default_message(DiagnosticCode::BSE02006));
        }

        // BSE02009: Async methods must return Task or Task<T>
        if member.modifiers.contains(&Modifier::Async) {
            if let Some(return_type) = &member.return_type {
                // Check if return type is Task or Task<T>
                let is_valid_async_return = match return_type {
                    crate::syntax::nodes::types::Type::Reference(ref_type) => {
                        ref_type.name == "Task"
                    }
                    crate::syntax::nodes::types::Type::Generic { base, .. } => {
                        base.name == "Task"
                    }
                    crate::syntax::nodes::types::Type::Primitive(crate::syntax::nodes::types::PrimitiveType::Void) => {
                        true // async void is technically allowed but discouraged
                    }
                    _ => false,
                };

                if !is_valid_async_return {
                    diagnostics.add(Diagnostic::with_default_message(DiagnosticCode::BSE02009));
                }
            }
        }

        // BSE04006: Abstract members cannot be private
        if member.modifiers.contains(&Modifier::Abstract) && member.modifiers.contains(&Modifier::Private) {
            diagnostics.add(Diagnostic::with_default_message(DiagnosticCode::BSE04006));
        }

        // BSE04007: Virtual members cannot be private  
        if member.modifiers.contains(&Modifier::Virtual) && member.modifiers.contains(&Modifier::Private) {
            diagnostics.add(Diagnostic::with_default_message(DiagnosticCode::BSE04007));
        }
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::nodes::identifier::Identifier;
    use crate::syntax::nodes::types::{PrimitiveType, Type};

    #[test]
    fn test_async_constructor_semantic_error() {
        let mut analyzer = SemanticAnalyzer::new();
        
        let async_constructor = MemberDeclaration {
            modifiers: vec![Modifier::Public, Modifier::Async],
            return_type: None,
            name: Identifier { name: "TestClass".to_string() },
            type_parameters: None,
            parameters: vec![],
            body: None,
            constraints: None,
        };

        let analysis = analyzer.analyze_member(&async_constructor, "TestClass");
        
        assert_eq!(analysis.semantic_type, SemanticMemberType::Constructor);
        assert!(!analysis.is_valid);
        assert!(analysis.diagnostics.has_errors());
        
        let errors: Vec<_> = analysis.diagnostics.errors().collect();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, DiagnosticCode::BSE01001);
    }

    #[test]
    fn test_valid_method_semantic_analysis() {
        let mut analyzer = SemanticAnalyzer::new();
        
        let valid_method = MemberDeclaration {
            modifiers: vec![Modifier::Public],
            return_type: Some(Type::Primitive(PrimitiveType::Void)),
            name: Identifier { name: "TestMethod".to_string() },
            type_parameters: None,
            parameters: vec![],
            body: None, // In interface context this would be valid
            constraints: None,
        };

        let analysis = analyzer.analyze_member(&valid_method, "TestClass");
        
        assert_eq!(analysis.semantic_type, SemanticMemberType::Method);
        assert!(analysis.is_valid);
        assert!(!analysis.diagnostics.has_errors());
    }

    #[test]
    fn test_virtual_static_method_error() {
        let mut analyzer = SemanticAnalyzer::new();
        
        let invalid_method = MemberDeclaration {
            modifiers: vec![Modifier::Public, Modifier::Virtual, Modifier::Static],
            return_type: Some(Type::Primitive(PrimitiveType::Void)),
            name: Identifier { name: "TestMethod".to_string() },
            type_parameters: None,
            parameters: vec![],
            body: None,
            constraints: None,
        };

        let analysis = analyzer.analyze_member(&invalid_method, "TestClass");
        
        assert_eq!(analysis.semantic_type, SemanticMemberType::Method);
        assert!(!analysis.is_valid);
        assert!(analysis.diagnostics.has_errors());
        
        let errors: Vec<_> = analysis.diagnostics.errors().collect();
        assert!(errors.iter().any(|e| e.code == DiagnosticCode::BSE02005));
    }
} 