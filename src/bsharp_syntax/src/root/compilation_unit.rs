use crate::ast::TopLevelDeclaration;
use crate::declarations::{
    FileScopedNamespaceDeclaration, GlobalAttribute, GlobalUsingDirective, UsingDirective,
};
use crate::statements::statement::Statement;
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompilationUnit {
    pub global_attributes: Vec<GlobalAttribute>,
    pub using_directives: Vec<UsingDirective>,
    pub global_using_directives: Vec<GlobalUsingDirective>,
    pub declarations: Vec<TopLevelDeclaration>,
    pub file_scoped_namespace: Option<FileScopedNamespaceDeclaration>,
    pub top_level_statements: Vec<Statement>,
}

impl crate::node::ast_node::AstNode for CompilationUnit {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn children<'a>(&'a self, push: &mut dyn FnMut(crate::node::ast_node::NodeRef<'a>)) {
        use crate::node::ast_node::push_child;
        // Flatten file-scoped namespace (if present)
        if let Some(ns) = self.file_scoped_namespace.as_ref() {
            push_child(push, ns);
        }
        // Flatten top-level declarations to their inner concrete nodes
        for d in &self.declarations {
            match d {
                TopLevelDeclaration::Namespace(n) => push_child(push, n),
                TopLevelDeclaration::FileScopedNamespace(n) => push_child(push, n),
                TopLevelDeclaration::Class(c) => push_child(push, c),
                TopLevelDeclaration::Struct(s) => push_child(push, s),
                TopLevelDeclaration::Record(r) => push_child(push, r),
                TopLevelDeclaration::Interface(i) => push_child(push, i),
                TopLevelDeclaration::Enum(e) => push_child(push, e),
                TopLevelDeclaration::Delegate(d) => push_child(push, d),
                TopLevelDeclaration::GlobalAttribute(_ga) => { /* omit from CU direct children */ }
            }
        }
        // Optionally expose statements and using/global attributes as children if desired later.
    }
}
