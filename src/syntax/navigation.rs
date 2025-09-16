use crate::parser::ast::CompilationUnit;
use crate::parser::nodes::{
    declarations::{ClassDeclaration, MethodDeclaration},
    statements::statement::Statement,
};

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
}

/// Trait for finding specific declaration types
pub trait FindDeclarations {
    /// Find all classes within this node
    fn find_classes(&self) -> Vec<&ClassDeclaration>;
    
    /// Find all methods within this node
    fn find_methods(&self) -> Vec<&MethodDeclaration>;
}

impl AstNavigate for CompilationUnit {
    fn find_if_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_if_statements());
        }
        results
    }
    
    fn find_for_loops(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_for_loops());
        }
        results
    }
    
    fn find_while_loops(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_while_loops());
        }
        results
    }
    
    fn find_switch_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_switch_statements());
        }
        results
    }
}

impl FindDeclarations for CompilationUnit {
    fn find_classes(&self) -> Vec<&ClassDeclaration> {
        let mut classes = Vec::new();
        for member in &self.declarations {
            match member {
                crate::parser::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        if let crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Class(class) = ns_member {
                            classes.push(class);
                        }
                    }
                }
                crate::parser::ast::TopLevelDeclaration::Class(class) => {
                    classes.push(class);
                }
                _ => {}
            }
        }
        classes
    }
    
    fn find_methods(&self) -> Vec<&MethodDeclaration> {
        let mut methods = Vec::new();
        for class in self.find_classes() {
            methods.extend(class.find_methods());
        }
        methods
    }
}

impl AstNavigate for ClassDeclaration {
    fn find_if_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_if_statements());
        }
        results
    }
    
    fn find_for_loops(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_for_loops());
        }
        results
    }
    
    fn find_while_loops(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_while_loops());
        }
        results
    }
    
    fn find_switch_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_switch_statements());
        }
        results
    }
}

impl FindDeclarations for ClassDeclaration {
    fn find_classes(&self) -> Vec<&ClassDeclaration> {
        vec![self] // A class contains itself
    }
    
    fn find_methods(&self) -> Vec<&MethodDeclaration> {
        let mut methods = Vec::new();
        for member in &self.body_declarations {
            if let crate::parser::nodes::declarations::ClassBodyDeclaration::Method(method) = member {
                methods.push(method);
            }
        }
        methods
    }
}

impl AstNavigate for MethodDeclaration {
    fn find_if_statements(&self) -> Vec<&Statement> {
        if let Some(body) = &self.body {
            body.find_if_statements()
        } else {
            Vec::new()
        }
    }
    
    fn find_for_loops(&self) -> Vec<&Statement> {
        if let Some(body) = &self.body {
            body.find_for_loops()
        } else {
            Vec::new()
        }
    }
    
    fn find_while_loops(&self) -> Vec<&Statement> {
        if let Some(body) = &self.body {
            body.find_while_loops()
        } else {
            Vec::new()
        }
    }
    
    fn find_switch_statements(&self) -> Vec<&Statement> {
        if let Some(body) = &self.body {
            body.find_switch_statements()
        } else {
            Vec::new()
        }
    }
}

impl FindDeclarations for MethodDeclaration {
    fn find_classes(&self) -> Vec<&ClassDeclaration> {
        Vec::new() // Methods don't contain classes
    }
    
    fn find_methods(&self) -> Vec<&MethodDeclaration> {
        vec![self] // A method contains itself
    }
}

impl AstNavigate for Statement {
    fn find_if_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        collect_if_statements(self, &mut results);
        results
    }
    
    fn find_for_loops(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        collect_for_loops(self, &mut results);
        results
    }
    
    fn find_while_loops(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        collect_while_loops(self, &mut results);
        results
    }
    
    fn find_switch_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        collect_switch_statements(self, &mut results);
        results
    }
}

// Helper functions for recursive collection (kept private as implementation details)
fn collect_if_statements<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::If(if_stmt) => {
            results.push(stmt);
            collect_if_statements(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_if_statements(alt, results);
            }
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_if_statements(s, results);
            }
        }
        Statement::For(for_stmt) => collect_if_statements(&for_stmt.body, results),
        Statement::While(while_stmt) => collect_if_statements(&while_stmt.body, results),
        Statement::DoWhile(do_while_stmt) => collect_if_statements(&do_while_stmt.body, results),
        Statement::Switch(switch_stmt) => {
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    collect_if_statements(s, results);
                }
            }
        }
        _ => {}
    }
}

fn collect_for_loops<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::For(for_stmt) => {
            results.push(stmt);
            collect_for_loops(&for_stmt.body, results);
        }
        Statement::If(if_stmt) => {
            collect_for_loops(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_for_loops(alt, results);
            }
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_for_loops(s, results);
            }
        }
        Statement::While(while_stmt) => collect_for_loops(&while_stmt.body, results),
        Statement::DoWhile(do_while_stmt) => collect_for_loops(&do_while_stmt.body, results),
        Statement::Switch(switch_stmt) => {
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    collect_for_loops(s, results);
                }
            }
        }
        _ => {}
    }
}

fn collect_while_loops<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::While(_) | Statement::DoWhile(_) => {
            results.push(stmt);
            match stmt {
                Statement::While(while_stmt) => collect_while_loops(&while_stmt.body, results),
                Statement::DoWhile(do_while_stmt) => collect_while_loops(&do_while_stmt.body, results),
                _ => unreachable!(),
            }
        }
        Statement::If(if_stmt) => {
            collect_while_loops(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_while_loops(alt, results);
            }
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_while_loops(s, results);
            }
        }
        Statement::For(for_stmt) => collect_while_loops(&for_stmt.body, results),
        Statement::Switch(switch_stmt) => {
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    collect_while_loops(s, results);
                }
            }
        }
        _ => {}
    }
}

fn collect_switch_statements<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::Switch(switch_stmt) => {
            results.push(stmt);
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    collect_switch_statements(s, results);
                }
            }
        }
        Statement::If(if_stmt) => {
            collect_switch_statements(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_switch_statements(alt, results);
            }
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_switch_statements(s, results);
            }
        }
        Statement::For(for_stmt) => collect_switch_statements(&for_stmt.body, results),
        Statement::While(while_stmt) => collect_switch_statements(&while_stmt.body, results),
        Statement::DoWhile(do_while_stmt) => collect_switch_statements(&do_while_stmt.body, results),
        _ => {}
    }
} 