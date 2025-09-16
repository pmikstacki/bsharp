use crate::parser::ast::*;
use crate::parser::nodes::statements::{UsingStatement, ForInitializer};
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::nodes::declarations::{
    NamespaceDeclaration as Namespace, ClassBodyDeclaration, InterfaceBodyDeclaration, StructBodyDeclaration,
    MethodDeclaration, PropertyDeclaration, FieldDeclaration, ConstructorDeclaration
};
use crate::parser::nodes::expressions::Expression;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Naming convention violations and inconsistencies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NamingViolation {
    /// Class should follow PascalCase
    ClassNotPascalCase(String),
    /// Method should follow PascalCase
    MethodNotPascalCase(String),
    /// Property should follow PascalCase
    PropertyNotPascalCase(String),
    /// Field should follow camelCase
    FieldNotCamelCase(String),
    /// Variable should follow camelCase
    VariableNotCamelCase(String),
    /// Parameter should follow camelCase
    ParameterNotCamelCase(String),
    /// Constant should follow PascalCase
    ConstantNotPascalCase(String),
    /// Enum should follow PascalCase
    EnumNotPascalCase(String),
    /// Interface should start with 'I' and follow PascalCase
    InterfaceNotIPascalCase(String),
    /// Namespace should follow PascalCase
    NamespaceNotPascalCase(String),
    /// Generic type parameter should be single uppercase letter
    GenericParameterNotSingleLetter(String),
    /// Abbreviations should be PascalCase
    AbbreviationNotPascalCase(String),
    /// Name is too short (less than 2 characters, except for loop variables)
    NameTooShort(String),
    /// Name is too long (more than 50 characters)
    NameTooLong(String),
    /// Name contains underscores where it shouldn't
    UnexpectedUnderscore(String),
    /// Private field doesn't start with underscore
    PrivateFieldShouldStartWithUnderscore(String),
}

/// Naming convention metrics
#[derive(Debug, Clone, Default)]
pub struct NamingMetrics {
    pub total_identifiers: usize,
    pub violations: Vec<NamingViolation>,
    pub compliance_rate: f64,
    pub avg_identifier_length: f64,
    pub identifier_length_distribution: HashMap<usize, usize>,
}

/// Analyzes naming conventions in B# code
pub struct NamingAnalyzer;

impl NamingAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Analyzes naming conventions in a compilation unit
    pub fn analyze(&self, compilation_unit: &CompilationUnit) -> NamingMetrics {
        let mut metrics = NamingMetrics::default();
        
        for _using_directive in &compilation_unit.using_directives {
            // using_directives don't typically have naming violations - skip for now
        }
        
        // Iterate through declarations to find namespaces and other top-level declarations
        for declaration in &compilation_unit.declarations {
            if let TopLevelDeclaration::Namespace(namespace) = declaration {
                self.analyze_namespace(namespace, &mut metrics);
            }
        }
        
        // Calculate metrics
        metrics.compliance_rate = if metrics.total_identifiers > 0 {
            1.0 - (metrics.violations.len() as f64 / metrics.total_identifiers as f64)
        } else {
            1.0
        };
        
        if metrics.total_identifiers > 0 {
            let total_length: usize = metrics.identifier_length_distribution
                .iter()
                .map(|(length, count)| length * count)
                .sum();
            metrics.avg_identifier_length = total_length as f64 / metrics.total_identifiers as f64;
        }
        
        metrics
    }

    

    fn analyze_namespace(&self, namespace: &Namespace, metrics: &mut NamingMetrics) {
        self.check_identifier(&namespace.name.name, IdentifierType::Namespace, metrics);
        
        for declaration in &namespace.declarations {
            match declaration {
                crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Class(class) => {
                    self.analyze_class(class, metrics);
                }
                crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Interface(interface) => {
                    self.analyze_interface(interface, metrics);
                }
                crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Enum(enum_decl) => {
                    self.analyze_enum(enum_decl, metrics);
                }
                crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Struct(struct_decl) => {
                    self.analyze_struct(struct_decl, metrics);
                }
                crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Record(record) => {
                    self.analyze_record(record, metrics);
                }
                crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Delegate(delegate) => {
                    self.analyze_delegate(delegate, metrics);
                }
                _ => {}
            }
        }
    }

    fn analyze_class(&self, class: &ClassDeclaration, metrics: &mut NamingMetrics) {
        self.check_identifier(&class.name.name, IdentifierType::Class, metrics);
        
        if let Some(ref type_params) = class.type_parameters {
            for param in type_params {
                self.check_identifier(&param.name.name, IdentifierType::GenericParameter, metrics);
            }
        }
        
        for member in &class.body_declarations {
            self.analyze_class_member(member, metrics);
        }
    }

    fn analyze_interface(&self, interface: &InterfaceDeclaration, metrics: &mut NamingMetrics) {
        self.check_identifier(&interface.name.name, IdentifierType::Interface, metrics);
        
        if let Some(ref type_params) = interface.type_parameters {
            for param in type_params {
                self.check_identifier(&param.name.name, IdentifierType::GenericParameter, metrics);
            }
        }
        
        for member in &interface.body_declarations {
            self.analyze_interface_member(member, metrics);
        }
    }

    fn analyze_enum(&self, enum_decl: &EnumDeclaration, metrics: &mut NamingMetrics) {
        self.check_identifier(&enum_decl.name.name, IdentifierType::Enum, metrics);
        
        for member in &enum_decl.enum_members {
            self.check_identifier(&member.name.name, IdentifierType::Constant, metrics);
        }
    }

    fn analyze_struct(&self, struct_decl: &StructDeclaration, metrics: &mut NamingMetrics) {
        self.check_identifier(&struct_decl.name.name, IdentifierType::Class, metrics);
        
        for member in &struct_decl.body_declarations {
            self.analyze_struct_member(member, metrics);
        }
    }

    fn analyze_record(&self, record: &RecordDeclaration, metrics: &mut NamingMetrics) {
        self.check_identifier(&record.name.name, IdentifierType::Class, metrics);
        
        if let Some(ref parameters) = record.parameters {
            for param in parameters {
                self.check_identifier(&param.name.name, IdentifierType::Property, metrics);
            }
        }
    }

    fn analyze_delegate(&self, delegate: &DelegateDeclaration, metrics: &mut NamingMetrics) {
        self.check_identifier(&delegate.name.name, IdentifierType::Class, metrics);
        
        for param in &delegate.parameters {
            self.check_identifier(&param.name.name, IdentifierType::Parameter, metrics);
        }
    }

    fn analyze_class_member(&self, member: &ClassBodyDeclaration, metrics: &mut NamingMetrics) {
        match member {
            ClassBodyDeclaration::Method(method) => self.analyze_method(method, metrics),
            ClassBodyDeclaration::Property(property) => self.analyze_property(property, metrics),
            ClassBodyDeclaration::Field(field) => self.analyze_field(field, metrics),
            ClassBodyDeclaration::Constructor(constructor) => self.analyze_constructor(constructor, metrics),
            ClassBodyDeclaration::Event(_) => {
                // TODO: Implement event analysis if needed
            }
            ClassBodyDeclaration::Indexer(_) => {
                // TODO: Implement indexer analysis if needed
            }
            ClassBodyDeclaration::Operator(_) => {
                // TODO: Implement operator analysis if needed
            }
            ClassBodyDeclaration::Destructor(_) => {
                // TODO: Implement destructor analysis if needed
            }
            ClassBodyDeclaration::Record(_) => {
                // TODO: Implement nested record analysis if needed
            }
            ClassBodyDeclaration::NestedClass(nested_class) => {
                self.analyze_class(nested_class, metrics);
            }
            ClassBodyDeclaration::NestedStruct(nested_struct) => {
                self.analyze_struct(nested_struct, metrics);
            }
            ClassBodyDeclaration::NestedInterface(nested_interface) => {
                self.analyze_interface(nested_interface, metrics);
            }
            ClassBodyDeclaration::NestedEnum(nested_enum) => {
                self.analyze_enum(nested_enum, metrics);
            }
            ClassBodyDeclaration::NestedRecord(nested_record) => {
                self.analyze_record(nested_record, metrics);
            }
        }
    }

    fn analyze_interface_member(&self, member: &InterfaceBodyDeclaration, metrics: &mut NamingMetrics) {
        match member {
            InterfaceBodyDeclaration::Method(method) => self.analyze_method(method, metrics),
            InterfaceBodyDeclaration::Property(property) => self.analyze_property(property, metrics),
            InterfaceBodyDeclaration::Event(_) => {
                // TODO: Implement event analysis if needed
            }
            InterfaceBodyDeclaration::Indexer(_) => {
                // TODO: Implement indexer analysis if needed
            }
        }
    }

    fn analyze_struct_member(&self, member: &StructBodyDeclaration, metrics: &mut NamingMetrics) {
        match member {
            StructBodyDeclaration::Method(method) => self.analyze_method(method, metrics),
            StructBodyDeclaration::Property(property) => self.analyze_property(property, metrics),
            StructBodyDeclaration::Field(field) => self.analyze_field(field, metrics),
            StructBodyDeclaration::Constructor(constructor) => self.analyze_constructor(constructor, metrics),
        }
    }

    fn analyze_method(&self, method: &MethodDeclaration, metrics: &mut NamingMetrics) {
        self.check_identifier(&method.name.name, IdentifierType::Method, metrics);
        
        for param in &method.parameters {
            self.check_identifier(&param.name.name, IdentifierType::Parameter, metrics);
        }
        
        if let Some(ref body) = method.body {
            self.analyze_statement(body, metrics);
        }
    }

    fn analyze_property(&self, property: &PropertyDeclaration, metrics: &mut NamingMetrics) {
        self.check_identifier(&property.name.name, IdentifierType::Property, metrics);
    }

    fn analyze_field(&self, field: &FieldDeclaration, metrics: &mut NamingMetrics) {
        self.check_identifier(&field.name.name, IdentifierType::Field, metrics);
    }

    fn analyze_constructor(&self, constructor: &ConstructorDeclaration, metrics: &mut NamingMetrics) {
        for param in &constructor.parameters {
            self.check_identifier(&param.name.name, IdentifierType::Parameter, metrics);
        }
        
        if let Some(ref body) = constructor.body {
            self.analyze_statement(body, metrics);
        }
    }

    fn analyze_statement(&self, statement: &Statement, metrics: &mut NamingMetrics) {
        match statement {
            Statement::Block(statements) => {
                for stmt in statements {
                    self.analyze_statement(stmt, metrics);
                }
            }
            Statement::Declaration(var_decl) => {
                for declarator in &var_decl.declarators {
                    self.check_identifier(&declarator.name.name, IdentifierType::Variable, metrics);
                    if let Some(ref init) = declarator.initializer {
                        self.analyze_expression(init, metrics);
                    }
                }
            }
            Statement::If(if_stmt) => {
                self.analyze_expression(&if_stmt.condition, metrics);
                self.analyze_statement(&if_stmt.consequence, metrics);
                if let Some(ref else_stmt) = if_stmt.alternative {
                    self.analyze_statement(else_stmt, metrics);
                }
            }
            Statement::For(for_stmt) => {
                if let Some(ref init) = for_stmt.initializer {
                    match init {
                        ForInitializer::Declaration(var) => {
                            for declarator in &var.declarators {
                                self.check_identifier(&declarator.name.name, IdentifierType::Variable, metrics);
                            }
                        }
                        ForInitializer::Expressions(exprs) => {
                            for expr in exprs {
                                self.analyze_expression(expr, metrics);
                            }
                        }
                    }
                }
                if let Some(ref condition) = for_stmt.condition {
                    self.analyze_expression(condition, metrics);
                }
                for update_expr in &for_stmt.iterator {
                    self.analyze_expression(update_expr, metrics);
                }
                self.analyze_statement(&for_stmt.body, metrics);
            }
            Statement::While(while_stmt) => {
                self.analyze_expression(&while_stmt.condition, metrics);
                self.analyze_statement(&while_stmt.body, metrics);
            }
            Statement::DoWhile(do_while_stmt) => {
                self.analyze_statement(&do_while_stmt.body, metrics);
                self.analyze_expression(&do_while_stmt.condition, metrics);
            }
            Statement::ForEach(foreach_stmt) => {
                self.check_identifier(&foreach_stmt.var_name.name, IdentifierType::Variable, metrics);
                self.analyze_expression(&foreach_stmt.collection, metrics);
                self.analyze_statement(&foreach_stmt.body, metrics);
            }
            Statement::Try(try_stmt) => {
                self.analyze_statement(&try_stmt.try_block, metrics);
                for catch in &try_stmt.catches {
                    if let Some(var) = &catch.exception_variable {
                        let name = &var.name;
                        if !self.is_valid_variable_name(name) {
                            metrics.violations.push(NamingViolation::PrivateFieldShouldStartWithUnderscore(name.to_string()));
                        }
                    }
                    self.analyze_statement(&catch.block, metrics);
                }
                if let Some(ref finally_clause) = try_stmt.finally_clause {
                    self.analyze_statement(&finally_clause.block, metrics);
                }
            }
            Statement::Expression(expr) => {
                self.analyze_expression(expr, metrics);
            }
            _ => {}
        }
    }

    fn analyze_expression(&self, _expression: &Expression, _metrics: &mut NamingMetrics) {
        // Expression analysis for identifier usage would go here
        // This is a simplified implementation
    }

    fn check_identifier(&self, name: &str, id_type: IdentifierType, metrics: &mut NamingMetrics) {
        metrics.total_identifiers += 1;
        
        // Track length distribution
        let length = name.len();
        *metrics.identifier_length_distribution.entry(length).or_insert(0) += 1;
        
        // Check various naming violations
        self.check_length_violations(name, metrics);
        self.check_convention_violations(name, id_type, metrics);
    }

    fn check_length_violations(&self, name: &str, metrics: &mut NamingMetrics) {
        if name.len() < 2 && !self.is_acceptable_short_name(name) {
            metrics.violations.push(NamingViolation::NameTooShort(name.to_string()));
        }
        
        if name.len() > 50 {
            metrics.violations.push(NamingViolation::NameTooLong(name.to_string()));
        }
    }

    fn check_convention_violations(&self, name: &str, id_type: IdentifierType, metrics: &mut NamingMetrics) {
        match id_type {
            IdentifierType::Class | IdentifierType::Enum => {
                if !self.is_pascal_case(name) {
                    metrics.violations.push(match id_type {
                        IdentifierType::Class => NamingViolation::ClassNotPascalCase(name.to_string()),
                        IdentifierType::Enum => NamingViolation::EnumNotPascalCase(name.to_string()),
                        _ => unreachable!(),
                    });
                }
            }
            IdentifierType::Interface => {
                if !name.starts_with('I') || !self.is_pascal_case(&name[1..]) {
                    metrics.violations.push(NamingViolation::InterfaceNotIPascalCase(name.to_string()));
                }
            }
            IdentifierType::Method | IdentifierType::Property | IdentifierType::Namespace => {
                if !self.is_pascal_case(name) {
                    metrics.violations.push(match id_type {
                        IdentifierType::Method => NamingViolation::MethodNotPascalCase(name.to_string()),
                        IdentifierType::Property => NamingViolation::PropertyNotPascalCase(name.to_string()),
                        IdentifierType::Namespace => NamingViolation::NamespaceNotPascalCase(name.to_string()),
                        _ => unreachable!(),
                    });
                }
            }
            IdentifierType::Field | IdentifierType::Variable | IdentifierType::Parameter => {
                if !self.is_camel_case(name) {
                    metrics.violations.push(match id_type {
                        IdentifierType::Field => NamingViolation::FieldNotCamelCase(name.to_string()),
                        IdentifierType::Variable => NamingViolation::VariableNotCamelCase(name.to_string()),
                        IdentifierType::Parameter => NamingViolation::ParameterNotCamelCase(name.to_string()),
                        _ => unreachable!(),
                    });
                }
            }
            IdentifierType::Constant => {
                if !self.is_pascal_case(name) {
                    metrics.violations.push(NamingViolation::ConstantNotPascalCase(name.to_string()));
                }
            }
            IdentifierType::GenericParameter => {
                if name.len() != 1 || !name.chars().all(|c| c.is_uppercase()) {
                    metrics.violations.push(NamingViolation::GenericParameterNotSingleLetter(name.to_string()));
                }
            }
        }
    }

    fn is_pascal_case(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        
        let mut chars = name.chars();
        let first = chars.next().unwrap();
        
        if !first.is_uppercase() {
            return false;
        }
        
        let remaining: String = chars.collect();
        
        // All uppercase is not PascalCase
        if remaining.chars().all(|c| c.is_uppercase()) && remaining.len() > 0 {
            return false;
        }
        
        // Check that subsequent words start with uppercase
        let mut prev_was_lower = first.is_lowercase(); // This will be false for first char
        for ch in remaining.chars() {
            if ch.is_uppercase() && prev_was_lower {
                prev_was_lower = false;
            } else if ch.is_lowercase() {
                prev_was_lower = true;
            } else if ch == '_' {
                return false; // PascalCase shouldn't have underscores
            } else if ch.is_uppercase() && !prev_was_lower {
                // Two consecutive uppercase chars are okay for abbreviations
                continue;
            }
        }
        
        // Must have at least one lowercase letter to be valid PascalCase
        name.chars().any(|c| c.is_lowercase())
    }

    fn is_camel_case(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        
        let mut chars = name.chars();
        let first = chars.next().unwrap();
        
        if !first.is_lowercase() {
            return false;
        }
        
        // Rest should follow camelCase rules
        let mut prev_was_lower = true;
        for ch in chars {
            if ch.is_uppercase() && prev_was_lower {
                prev_was_lower = false;
            } else if ch.is_lowercase() {
                prev_was_lower = true;
            } else if ch == '_' {
                return false; // camelCase shouldn't have underscores
            }
        }
        
        true
    }

    fn is_acceptable_short_name(&self, name: &str) -> bool {
        // Loop variables like 'i', 'j', 'k' are acceptable
        matches!(name, "i" | "j" | "k" | "x" | "y" | "z")
    }

    fn is_valid_variable_name(&self, _name: &str) -> bool {
        // Implement the logic to check if the variable name is valid
        true // Placeholder, actual implementation needed
    }
}

#[derive(Debug, Clone, Copy)]
enum IdentifierType {
    Class,
    Interface,
    Enum,
    Method,
    Property,
    Field,
    Variable,
    Parameter,
    Constant,
    Namespace,
    GenericParameter,
}

impl Default for NamingAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_case_detection() {
        let analyzer = NamingAnalyzer::new();
        
        assert!(analyzer.is_pascal_case("PascalCase"));
        assert!(analyzer.is_pascal_case("MyClass"));
        assert!(analyzer.is_pascal_case("XMLHttpRequest"));
        
        assert!(!analyzer.is_pascal_case("camelCase"));
        assert!(!analyzer.is_pascal_case("snake_case"));
        assert!(!analyzer.is_pascal_case("UPPERCASE"));
        assert!(!analyzer.is_pascal_case(""));
    }

    #[test]
    fn test_camel_case_detection() {
        let analyzer = NamingAnalyzer::new();
        
        assert!(analyzer.is_camel_case("camelCase"));
        assert!(analyzer.is_camel_case("myVariable"));
        assert!(analyzer.is_camel_case("xmlHttpRequest"));
        
        assert!(!analyzer.is_camel_case("PascalCase"));
        assert!(!analyzer.is_camel_case("snake_case"));
        assert!(!analyzer.is_camel_case("UPPERCASE"));
        assert!(!analyzer.is_camel_case(""));
    }
} 