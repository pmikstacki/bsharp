use super::traits::{AstNavigate, FindDeclarations, DeclarationInfo, DeclarationType};
use crate::syntax::ast::CompilationUnit;
use crate::syntax::nodes::{
    declarations::{ClassDeclaration, MethodDeclaration, InterfaceDeclaration, StructDeclaration, EnumDeclaration, RecordDeclaration, DelegateDeclaration},
    statements::statement::Statement,
    expressions::expression::Expression,
};

// Internal helpers to streamline iteration over declarations across namespaces and file-scoped namespaces
fn iter_namespace_members<'a>(cu: &'a CompilationUnit) -> impl Iterator<Item = &'a crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration> + 'a {
    let top_level = cu.declarations.iter().filter_map(|member| {
        if let crate::syntax::ast::TopLevelDeclaration::Namespace(ns) = member {
            Some(ns.declarations.iter())
        } else {
            None
        }
    }).flatten();

    let file_scoped = cu.file_scoped_namespace.iter().flat_map(|fs| fs.declarations.iter());

    top_level.chain(file_scoped)
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
    
    fn find_try_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_try_statements());
        }
        results
    }
    
    fn find_using_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_using_statements());
        }
        results
    }
    
    fn find_expressions<F>(&self, predicate: F) -> Vec<&Expression>
    where
        F: Fn(&Expression) -> bool
    {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_expressions(&predicate));
        }
        results
    }
}

impl FindDeclarations for CompilationUnit {
    fn find_classes(&self) -> Vec<&ClassDeclaration> {
        let mut classes = Vec::new();

        // Direct top-level classes
        for member in &self.declarations {
            if let crate::syntax::ast::TopLevelDeclaration::Class(class) = member {
                classes.push(class);
            }
        }

        // Classes inside namespaces (including file-scoped)
        for ns_member in iter_namespace_members(self) {
            if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Class(class) = ns_member {
                classes.push(class);
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
    
    fn find_interfaces(&self) -> Vec<&InterfaceDeclaration> {
        let mut interfaces = Vec::new();
        
        // Search top-level declarations
        for member in &self.declarations {
            match member {
                crate::syntax::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Interface(interface) = ns_member {
                            interfaces.push(interface);
                        }
                    }
                }
                crate::syntax::ast::TopLevelDeclaration::Interface(interface) => {
                    interfaces.push(interface);
                }
                _ => {}
            }
        }
        
        // Search file-scoped namespace
        if let Some(file_scoped_ns) = &self.file_scoped_namespace {
            for declaration in &file_scoped_ns.declarations {
                if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Interface(interface) = declaration {
                    interfaces.push(interface);
                }
            }
        }
        
        interfaces
    }
    
    fn find_structs(&self) -> Vec<&StructDeclaration> {
        let mut structs = Vec::new();
        
        for member in &self.declarations {
            match member {
                crate::syntax::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Struct(struct_decl) = ns_member {
                            structs.push(struct_decl);
                        }
                    }
                }
                crate::syntax::ast::TopLevelDeclaration::Struct(struct_decl) => {
                    structs.push(struct_decl);
                }
                _ => {}
            }
        }
        
        if let Some(file_scoped_ns) = &self.file_scoped_namespace {
            for declaration in &file_scoped_ns.declarations {
                if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Struct(struct_decl) = declaration {
                    structs.push(struct_decl);
                }
            }
        }
        
        structs
    }
    
    fn find_enums(&self) -> Vec<&EnumDeclaration> {
        let mut enums = Vec::new();
        
        for member in &self.declarations {
            match member {
                crate::syntax::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Enum(enum_decl) = ns_member {
                            enums.push(enum_decl);
                        }
                    }
                }
                crate::syntax::ast::TopLevelDeclaration::Enum(enum_decl) => {
                    enums.push(enum_decl);
                }
                _ => {}
            }
        }
        
        if let Some(file_scoped_ns) = &self.file_scoped_namespace {
            for declaration in &file_scoped_ns.declarations {
                if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Enum(enum_decl) = declaration {
                    enums.push(enum_decl);
                }
            }
        }
        
        enums
    }
    
    fn find_records(&self) -> Vec<&RecordDeclaration> {
        let mut records = Vec::new();
        
        for member in &self.declarations {
            match member {
                crate::syntax::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Record(record_decl) = ns_member {
                            records.push(record_decl);
                        }
                    }
                }
                crate::syntax::ast::TopLevelDeclaration::Record(record_decl) => {
                    records.push(record_decl);
                }
                _ => {}
            }
        }
        
        if let Some(file_scoped_ns) = &self.file_scoped_namespace {
            for declaration in &file_scoped_ns.declarations {
                if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Record(record_decl) = declaration {
                    records.push(record_decl);
                }
            }
        }
        
        records
    }
    
    fn find_delegates(&self) -> Vec<&DelegateDeclaration> {
        let mut delegates = Vec::new();
        
        for member in &self.declarations {
            match member {
                crate::syntax::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Delegate(delegate_decl) = ns_member {
                            delegates.push(delegate_decl);
                        }
                    }
                }
                crate::syntax::ast::TopLevelDeclaration::Delegate(delegate_decl) => {
                    delegates.push(delegate_decl);
                }
                _ => {}
            }
        }
        
        if let Some(file_scoped_ns) = &self.file_scoped_namespace {
            for declaration in &file_scoped_ns.declarations {
                if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Delegate(delegate_decl) = declaration {
                    delegates.push(delegate_decl);
                }
            }
        }
        
        delegates
    }
    
    fn find_by_name(&self, name: &str) -> Vec<DeclarationInfo> {
        let mut results = Vec::new();
        
        // Find classes
        for class in self.find_classes() {
            if class.name.name == name {
                results.push(DeclarationInfo {
                    name: class.name.name.clone(),
                    declaration_type: DeclarationType::Class,
                    location: None,
                });
            }
        }
        
        // Find interfaces
        for interface in self.find_interfaces() {
            if interface.name.name == name {
                results.push(DeclarationInfo {
                    name: interface.name.name.clone(),
                    declaration_type: DeclarationType::Interface,
                    location: None,
                });
            }
        }
        
        // Find methods
        for method in self.find_methods() {
            if method.name.name == name {
                results.push(DeclarationInfo {
                    name: method.name.name.clone(),
                    declaration_type: DeclarationType::Method,
                    location: None,
                });
            }
        }
        
        results
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
    
    fn find_try_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_try_statements());
        }
        results
    }
    
    fn find_using_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_using_statements());
        }
        results
    }
    
    fn find_expressions<F>(&self, predicate: F) -> Vec<&Expression>
    where
        F: Fn(&Expression) -> bool
    {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_expressions(&predicate));
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
            if let crate::syntax::nodes::declarations::ClassBodyDeclaration::Method(method) = member {
                methods.push(method);
            }
        }
        methods
    }
    
    fn find_interfaces(&self) -> Vec<&InterfaceDeclaration> {
        Vec::new() // Classes don't contain interfaces directly
    }
    
    fn find_structs(&self) -> Vec<&StructDeclaration> {
        Vec::new() // TODO: Add nested struct support when needed
    }
    
    fn find_enums(&self) -> Vec<&EnumDeclaration> {
        Vec::new() // TODO: Add nested enum support when needed
    }
    
    fn find_records(&self) -> Vec<&RecordDeclaration> {
        Vec::new() // TODO: Add nested record support when needed
    }
    
    fn find_delegates(&self) -> Vec<&DelegateDeclaration> {
        Vec::new() // TODO: Add nested delegate support when needed
    }
    
    fn find_by_name(&self, name: &str) -> Vec<DeclarationInfo> {
        let mut results = Vec::new();
        
        if self.name.name == name {
            results.push(DeclarationInfo {
                name: self.name.name.clone(),
                declaration_type: DeclarationType::Class,
                location: None,
            });
        }
        
        for method in self.find_methods() {
            if method.name.name == name {
                results.push(DeclarationInfo {
                    name: method.name.name.clone(),
                    declaration_type: DeclarationType::Method,
                    location: None,
                });
            }
        }
        
        results
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
    
    fn find_try_statements(&self) -> Vec<&Statement> {
        if let Some(body) = &self.body {
            body.find_try_statements()
        } else {
            Vec::new()
        }
    }
    
    fn find_using_statements(&self) -> Vec<&Statement> {
        if let Some(body) = &self.body {
            body.find_using_statements()
        } else {
            Vec::new()
        }
    }
    
    fn find_expressions<F>(&self, predicate: F) -> Vec<&Expression>
    where
        F: Fn(&Expression) -> bool
    {
        if let Some(body) = &self.body {
            body.find_expressions(predicate)
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
    
    fn find_interfaces(&self) -> Vec<&InterfaceDeclaration> {
        Vec::new()
    }
    
    fn find_structs(&self) -> Vec<&StructDeclaration> {
        Vec::new()
    }
    
    fn find_enums(&self) -> Vec<&EnumDeclaration> {
        Vec::new()
    }
    
    fn find_records(&self) -> Vec<&RecordDeclaration> {
        Vec::new()
    }
    
    fn find_delegates(&self) -> Vec<&DelegateDeclaration> {
        Vec::new()
    }
    
    fn find_by_name(&self, name: &str) -> Vec<DeclarationInfo> {
        if self.name.name == name {
            vec![DeclarationInfo {
                name: self.name.name.clone(),
                declaration_type: DeclarationType::Method,
                location: None,
            }]
        } else {
            Vec::new()
        }
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
    
    fn find_try_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        collect_try_statements(self, &mut results);
        results
    }
    
    fn find_using_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        collect_using_statements(self, &mut results);
        results
    }
    
    fn find_expressions<F>(&self, predicate: F) -> Vec<&Expression>
    where
        F: Fn(&Expression) -> bool
    {
        let mut results = Vec::new();
        collect_expressions(self, &predicate, &mut results);
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
        Statement::For(_) => {
            results.push(stmt);
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_for_loops(s, results);
            }
        }
        Statement::If(if_stmt) => {
            collect_for_loops(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_for_loops(alt, results);
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
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_while_loops(s, results);
            }
        }
        Statement::If(if_stmt) => {
            collect_while_loops(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_while_loops(alt, results);
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
        Statement::Switch(_) => {
            results.push(stmt);
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_switch_statements(s, results);
            }
        }
        Statement::If(if_stmt) => {
            collect_switch_statements(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_switch_statements(alt, results);
            }
        }
        Statement::For(for_stmt) => collect_switch_statements(&for_stmt.body, results),
        Statement::While(while_stmt) => collect_switch_statements(&while_stmt.body, results),
        Statement::DoWhile(do_while_stmt) => collect_switch_statements(&do_while_stmt.body, results),
        _ => {}
    }
}

fn collect_try_statements<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::Try(_) => {
            results.push(stmt);
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_try_statements(s, results);
            }
        }
        Statement::If(if_stmt) => {
            collect_try_statements(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_try_statements(alt, results);
            }
        }
        Statement::For(for_stmt) => collect_try_statements(&for_stmt.body, results),
        Statement::While(while_stmt) => collect_try_statements(&while_stmt.body, results),
        Statement::DoWhile(do_while_stmt) => collect_try_statements(&do_while_stmt.body, results),
        Statement::Switch(switch_stmt) => {
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    collect_try_statements(s, results);
                }
            }
        }
        _ => {}
    }
}

fn collect_using_statements<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::Using(_) => {
            results.push(stmt);
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_using_statements(s, results);
            }
        }
        Statement::If(if_stmt) => {
            collect_using_statements(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_using_statements(alt, results);
            }
        }
        Statement::For(for_stmt) => collect_using_statements(&for_stmt.body, results),
        Statement::While(while_stmt) => collect_using_statements(&while_stmt.body, results),
        Statement::DoWhile(do_while_stmt) => collect_using_statements(&do_while_stmt.body, results),
        Statement::Switch(switch_stmt) => {
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    collect_using_statements(s, results);
                }
            }
        }
        _ => {}
    }
}

fn collect_expressions<'a, F>(stmt: &'a Statement, predicate: &F, results: &mut Vec<&'a Expression>)
where
    F: Fn(&Expression) -> bool
{
    // TODO: Implement expression collection within statements
    // This would require traversing all expression nodes within statements
    let _ = (stmt, predicate, results); // Suppress unused warnings for now
} 