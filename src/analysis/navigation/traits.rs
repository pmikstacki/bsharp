use crate::syntax::nodes::{
    declarations::{
        ClassDeclaration, DelegateDeclaration, EnumDeclaration, InterfaceDeclaration,
        MethodDeclaration, RecordDeclaration, StructDeclaration,
    },
    expressions::expression::Expression,
    statements::statement::Statement,
};
use crate::analysis::diagnostics::source_location::SourceLocation;

/// Trait for navigating and searching through AST nodes
pub trait AstNavigate {
    /// Find all if statements within this node
    fn find_if_statements(&self) -> Vec<&Statement>;

    /// Find all for loops within this node
    fn find_for_loops(&self) -> Vec<&Statement>;

    /// Find all while loops within this node
    fn find_while_loops(&self) -> Vec<&Statement>;

    /// Find all switch statements within this node
    fn find_switch_statements(&self) -> Vec<&Statement>;

    /// Find all try statements within this node
    fn find_try_statements(&self) -> Vec<&Statement>;

    /// Find all using statements within this node
    fn find_using_statements(&self) -> Vec<&Statement>;

    /// Find all expressions of a specific type
    fn find_expressions<F>(&self, predicate: F) -> Vec<&Expression>
    where
        F: Fn(&Expression) -> bool;
}

/// Trait for finding specific declaration types
pub trait FindDeclarations {
    /// Find all classes within this node
    fn find_classes(&self) -> Vec<&ClassDeclaration>;

    /// Find all methods within this node
    fn find_methods(&self) -> Vec<&MethodDeclaration>;

    /// Find all interfaces within this node
    fn find_interfaces(&self) -> Vec<&InterfaceDeclaration>;

    /// Find all structs within this node
    fn find_structs(&self) -> Vec<&StructDeclaration>;

    /// Find all enums within this node
    fn find_enums(&self) -> Vec<&EnumDeclaration>;

    /// Find all records within this node
    fn find_records(&self) -> Vec<&RecordDeclaration>;

    /// Find all delegates within this node
    fn find_delegates(&self) -> Vec<&DelegateDeclaration>;

    /// Find declarations by name
    fn find_by_name(&self, name: &str) -> Vec<DeclarationInfo>;
}

/// Information about a found declaration
#[derive(Debug, Clone)]
pub struct DeclarationInfo {
    pub name: String,
    pub declaration_type: DeclarationType,
    pub location: Option<SourceLocation>, // Precise source location when available
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationType {
    Class,
    Interface,
    Struct,
    Enum,
    Record,
    Delegate,
    Method,
    Property,
    Field,
    Event,
    Constructor,
}
