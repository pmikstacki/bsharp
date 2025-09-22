use super::traits::{AstNavigate, DeclarationInfo, DeclarationType, FindDeclarations};
use crate::syntax::ast::CompilationUnit;
use crate::syntax::nodes::{
    declarations::{
        ClassDeclaration, DelegateDeclaration, EnumDeclaration, InterfaceDeclaration,
        MethodDeclaration, RecordDeclaration, StructDeclaration,
    },
    expressions::expression::Expression,
    statements::statement::Statement,
};

// Internal helpers to streamline iteration over declarations across namespaces and file-scoped namespaces
fn iter_namespace_members<'a>(
    cu: &'a CompilationUnit,
) -> impl Iterator<
    Item = &'a crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration,
> + 'a {
    let top_level = cu
        .declarations
        .iter()
        .filter_map(|member| {
            if let crate::syntax::ast::TopLevelDeclaration::Namespace(ns) = member {
                Some(ns.declarations.iter())
            } else {
                None
            }
        })
        .flatten();

    let file_scoped = cu
        .file_scoped_namespace
        .iter()
        .flat_map(|fs| fs.declarations.iter());

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
        F: Fn(&Expression) -> bool,
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
        F: Fn(&Expression) -> bool,
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
            if let crate::syntax::nodes::declarations::ClassBodyDeclaration::Method(method) = member
            {
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
        F: Fn(&Expression) -> bool,
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
        F: Fn(&Expression) -> bool,
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
        Statement::DoWhile(do_while_stmt) => {
            collect_switch_statements(&do_while_stmt.body, results)
        }
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
    F: Fn(&Expression) -> bool,
{
    // Inner recursive helpers to traverse expressions and patterns
    fn visit_expr<'a, F>(expr: &'a Expression, predicate: &F, results: &mut Vec<&'a Expression>)
    where
        F: Fn(&Expression) -> bool,
    {
        if predicate(expr) {
            results.push(expr);
        }

        use crate::syntax::nodes::expressions::expression::Expression as E;
        match expr {
            // Simple leaves
            E::Literal(_) | E::Variable(_) | E::This | E::Base => {}

            // Unary/postfix
            E::Unary { expr: inner, .. } | E::PostfixUnary { expr: inner, .. } => {
                visit_expr(inner, predicate, results)
            }

            // Binary
            E::Binary { left, right, .. } => {
                visit_expr(left, predicate, results);
                visit_expr(right, predicate, results);
            }

            // Indexing expression (a[b])
            E::Indexing(idx) => {
                visit_expr(&idx.target, predicate, results);
                visit_expr(&idx.index, predicate, results);
            }

            // Range and Index (^x, a..b)
            E::Range(r) => {
                if let Some(s) = &r.start {
                    visit_expr(s, predicate, results);
                }
                if let Some(e) = &r.end {
                    visit_expr(e, predicate, results);
                }
            }
            E::Index(i) => {
                visit_expr(&i.value, predicate, results);
            }

            // Member access and invocation
            E::MemberAccess(ma) => {
                visit_expr(&ma.object, predicate, results);
            }
            E::Invocation(inv) => {
                visit_expr(&inv.callee, predicate, results);
                for arg in &inv.arguments {
                    visit_expr(&arg.expr, predicate, results);
                }
            }

            // Assignment
            E::Assignment(assign) => {
                visit_expr(&assign.target, predicate, results);
                visit_expr(&assign.value, predicate, results);
            }

            // Parent containers
            E::Tuple(t) => {
                for el in &t.elements {
                    visit_expr(&el.value, predicate, results);
                }
            }
            E::AnonymousObject(obj) => {
                for m in &obj.initializers {
                    visit_expr(&m.value, predicate, results);
                }
            }
            E::New(n) => {
                for a in &n.arguments {
                    visit_expr(a, predicate, results);
                }
                if let Some(inits) = &n.object_initializer {
                    for (_, v) in inits {
                        visit_expr(v, predicate, results);
                    }
                }
                if let Some(coll) = &n.collection_initializer {
                    for v in coll {
                        visit_expr(v, predicate, results);
                    }
                }
            }

            // Conditional operator
            E::Conditional(c) => {
                visit_expr(&c.condition, predicate, results);
                visit_expr(&c.consequence, predicate, results);
                visit_expr(&c.alternative, predicate, results);
            }

            // Lambda and anonymous methods
            E::Lambda(lambda) => match &lambda.body {
                crate::syntax::nodes::expressions::lambda_expression::LambdaBody::ExpressionSyntax(e) => {
                    visit_expr(e, predicate, results)
                }
                crate::syntax::nodes::expressions::lambda_expression::LambdaBody::Block(stmts) => {
                    for s in stmts {
                        collect_expressions(s, predicate, results);
                    }
                }
            },
            E::AnonymousMethod(am) => match &am.body {
                crate::syntax::nodes::expressions::lambda_expression::LambdaBody::ExpressionSyntax(e) => {
                    visit_expr(e, predicate, results)
                }
                crate::syntax::nodes::expressions::lambda_expression::LambdaBody::Block(stmts) => {
                    for s in stmts {
                        collect_expressions(s, predicate, results);
                    }
                }
            },

            // Await
            E::Await(a) => visit_expr(&a.expr, predicate, results),

            // Query expressions: traverse all embedded expressions
            E::Query(q) => {
                use crate::syntax::nodes::expressions::query_expression as qx;
                visit_expr(&q.from.expression, predicate, results);
                for clause in &q.body {
                    match clause {
                        qx::QueryClause::From(f) => visit_expr(&f.expression, predicate, results),
                        qx::QueryClause::Let(l) => visit_expr(&l.expression, predicate, results),
                        qx::QueryClause::Where(w) => visit_expr(&w.condition, predicate, results),
                        qx::QueryClause::Join(j) => {
                            visit_expr(&j.in_expression, predicate, results);
                            visit_expr(&j.on_expression, predicate, results);
                            visit_expr(&j.equals_expression, predicate, results);
                        }
                        qx::QueryClause::OrderBy(ob) => {
                            for ord in &ob.orderings {
                                visit_expr(&ord.expression, predicate, results);
                            }
                        }
                    }
                }
                match &q.select_or_group {
                    qx::QuerySelectOrGroup::Select(e) => visit_expr(e, predicate, results),
                    qx::QuerySelectOrGroup::Group { element, by } => {
                        visit_expr(element, predicate, results);
                        visit_expr(by, predicate, results);
                    }
                }
                if let Some(cont) = &q.continuation {
                    for clause in &cont.body {
                        match clause {
                            qx::QueryClause::From(f) => visit_expr(&f.expression, predicate, results),
                            qx::QueryClause::Let(l) => visit_expr(&l.expression, predicate, results),
                            qx::QueryClause::Where(w) => visit_expr(&w.condition, predicate, results),
                            qx::QueryClause::Join(j) => {
                                visit_expr(&j.in_expression, predicate, results);
                                visit_expr(&j.on_expression, predicate, results);
                                visit_expr(&j.equals_expression, predicate, results);
                            }
                            qx::QueryClause::OrderBy(ob) => {
                                for ord in &ob.orderings {
                                    visit_expr(&ord.expression, predicate, results);
                                }
                            }
                        }
                    }
                    match &cont.select_or_group {
                        qx::QuerySelectOrGroup::Select(e) => visit_expr(e, predicate, results),
                        qx::QuerySelectOrGroup::Group { element, by } => {
                            visit_expr(element, predicate, results);
                            visit_expr(by, predicate, results);
                        }
                    }
                }
            }

            // Switch expression
            E::SwitchExpression(se) => {
                visit_expr(&se.expression, predicate, results);
                for arm in &se.arms {
                    visit_pattern(&arm.pattern, predicate, results);
                    if let Some(w) = &arm.when_clause {
                        visit_expr(w, predicate, results);
                    }
                    visit_expr(&arm.expression, predicate, results);
                }
            }

            // Pattern-containing expressions
            E::Pattern(p) => visit_pattern(p, predicate, results),
            E::IsPattern { expression, pattern } => {
                visit_expr(expression, predicate, results);
                visit_pattern(pattern, predicate, results);
            }

            // Cast/As
            E::As { expression, .. } | E::Cast { expression, .. } => {
                visit_expr(expression, predicate, results)
            }

            // Throw expression
            E::Throw(t) => {
                if let Some(e) = &t.expr {
                    visit_expr(e, predicate, results);
                }
            }

            // Nameof/Typeof/Sizeof/Default/StackAlloc/Checked/Unchecked/NullConditional
            E::Nameof(n) => visit_expr(&n.expr, predicate, results),
            E::Typeof(_) | E::Sizeof(_) | E::Default(_) => {}
            E::StackAlloc(sa) => {
                if let Some(count) = &sa.count {
                    visit_expr(count, predicate, results);
                }
                if let Some(inits) = &sa.initializer {
                    for e in inits {
                        visit_expr(e, predicate, results);
                    }
                }
            }
            E::Checked(ce) => visit_expr(&ce.expr, predicate, results),
            E::Unchecked(ue) => visit_expr(&ue.expr, predicate, results),
            E::NullConditional(nc) => {
                // Traverse the target, and argument when it's an element access
                visit_expr(&nc.target, predicate, results);
                if let Some(arg) = &nc.argument {
                    visit_expr(arg, predicate, results);
                }
            }

            // With-expressions and collection expressions
            E::With { target, initializers } => {
                visit_expr(target, predicate, results);
                for (_, v) in initializers {
                    visit_expr(v, predicate, results);
                }
            }
            E::Collection(items) => {
                for it in items {
                    match it {
                        crate::syntax::nodes::expressions::expression::CollectionElement::Expr(e) => {
                            visit_expr(e, predicate, results)
                        }
                        crate::syntax::nodes::expressions::expression::CollectionElement::Spread(e) => {
                            visit_expr(e, predicate, results)
                        }
                    }
                }
            }
            // Ref expression
            E::Ref(e) => visit_expr(e, predicate, results),
        }
    }

    fn visit_pattern<'a, F>(
        pat: &'a crate::syntax::nodes::expressions::pattern::Pattern,
        predicate: &F,
        results: &mut Vec<&'a Expression>,
    ) where
        F: Fn(&Expression) -> bool,
    {
        use crate::syntax::nodes::expressions::pattern::Pattern as P;
        match pat {
            P::Declaration { .. } | P::Var(_) | P::Discard => {}
            P::Constant(e) => visit_expr(e, predicate, results),
            P::Type { designation, .. } => {
                if let Some(d) = designation {
                    // No expressions inside designations
                    let _ = d;
                }
            }
            P::Property { subpatterns, .. } => {
                for sp in subpatterns {
                    visit_pattern(&sp.pattern, predicate, results);
                }
            }
            P::Positional { subpatterns, .. } | P::Tuple(subpatterns) => {
                for p in subpatterns {
                    visit_pattern(p, predicate, results);
                }
            }
            P::List { patterns } => {
                for el in patterns {
                    match el {
                        crate::syntax::nodes::expressions::pattern::ListPatternElement::Pattern(p) => {
                            visit_pattern(p, predicate, results)
                        }
                        crate::syntax::nodes::expressions::pattern::ListPatternElement::Slice(opt) => {
                            if let Some(p) = opt {
                                visit_pattern(p, predicate, results)
                            }
                        }
                    }
                }
            }
            P::Slice { pattern } => {
                if let Some(p) = pattern {
                    visit_pattern(p, predicate, results)
                }
            }
            P::Relational { value, .. } => visit_expr(value, predicate, results),
            P::LogicalAnd(a, b) | P::LogicalOr(a, b) => {
                visit_pattern(a, predicate, results);
                visit_pattern(b, predicate, results);
            }
            P::Not(p) | P::Parenthesized(p) => visit_pattern(p, predicate, results),
        }
    }

    // Traverse the statement and collect all expressions found
    match stmt {
        Statement::Expression(e) => visit_expr(e, predicate, results),
        Statement::Return(opt) | Statement::Throw(opt) => {
            if let Some(e) = opt.as_deref() {
                visit_expr(e, predicate, results);
            }
        }
        Statement::If(s) => {
            visit_expr(&s.condition, predicate, results);
            collect_expressions(&s.consequence, predicate, results);
            if let Some(alt) = &s.alternative {
                collect_expressions(alt, predicate, results);
            }
        }
        Statement::While(s) => {
            visit_expr(&s.condition, predicate, results);
            collect_expressions(&s.body, predicate, results);
        }
        Statement::DoWhile(s) => {
            collect_expressions(&s.body, predicate, results);
            visit_expr(&s.condition, predicate, results);
        }
        Statement::For(s) => {
            // Initializer
            if let Some(init) = &s.initializer {
                match init {
                    crate::syntax::nodes::statements::for_statement::ForInitializer::Declaration(d) => {
                        for decl in &d.declarators {
                            if let Some(init) = &decl.initializer {
                                visit_expr(init, predicate, results);
                            }
                        }
                    }
                    crate::syntax::nodes::statements::for_statement::ForInitializer::Expressions(exprs) => {
                        for e in exprs {
                            visit_expr(e, predicate, results);
                        }
                    }
                }
            }
            // Condition
            if let Some(cond) = &s.condition {
                visit_expr(cond, predicate, results);
            }
            // Iterators
            for it in &s.iterator {
                visit_expr(it, predicate, results);
            }
            collect_expressions(&s.body, predicate, results);
        }
        Statement::ForEach(s) => {
            visit_expr(&s.collection, predicate, results);
            collect_expressions(&s.body, predicate, results);
        }
        Statement::Switch(s) => {
            visit_expr(&s.expression, predicate, results);
            // Visit labels and statements
            for sec in &s.sections {
                for lbl in &sec.labels {
                    match lbl {
                        crate::syntax::nodes::statements::switch_label::SwitchLabel::Case(e) => {
                            visit_expr(e, predicate, results)
                        }
                        crate::syntax::nodes::statements::switch_label::SwitchLabel::Default => {}
                        crate::syntax::nodes::statements::switch_label::SwitchLabel::Pattern { pattern, when_clause } => {
                            visit_pattern(pattern, predicate, results);
                            if let Some(w) = when_clause {
                                visit_expr(w, predicate, results);
                            }
                        }
                    }
                }
                for st in &sec.statements {
                    collect_expressions(st, predicate, results);
                }
            }
        }
        Statement::Try(s) => {
            collect_expressions(&s.try_block, predicate, results);
            for c in &s.catches {
                collect_expressions(&c.block, predicate, results);
                if let Some(w) = &c.when_clause {
                    visit_expr(w, predicate, results);
                }
            }
            if let Some(f) = &s.finally_clause {
                collect_expressions(&f.block, predicate, results);
            }
        }
        Statement::Using(s) => {
            if let Some(expr) = &s.resource {
                visit_expr(expr, predicate, results);
            }
            if let Some(decl) = &s.declaration {
                for d in &decl.declarators {
                    if let Some(init) = &d.initializer {
                        visit_expr(init, predicate, results);
                    }
                }
            }
            if let Some(body) = &s.body {
                collect_expressions(body, predicate, results);
            }
        }
        Statement::Lock(s) => {
            visit_expr(&s.expr, predicate, results);
            collect_expressions(&s.body, predicate, results);
        }
        Statement::Checked(s) => collect_expressions(&s.body, predicate, results),
        Statement::Unchecked(s) => collect_expressions(&s.body, predicate, results),
        Statement::Fixed(s) => {
            visit_expr(&s.initializer, predicate, results);
            collect_expressions(&s.body, predicate, results);
        }
        Statement::Yield(y) => match y {
            crate::syntax::nodes::statements::yield_statement::YieldStatement::Return(e) => {
                visit_expr(e, predicate, results)
            }
            crate::syntax::nodes::statements::yield_statement::YieldStatement::Break => {}
        },
        Statement::Declaration(d) => {
            for decl in &d.declarators {
                if let Some(init) = &decl.initializer {
                    visit_expr(init, predicate, results);
                }
            }
        }
        Statement::LocalFunction(lf) => {
            collect_expressions(&lf.body, predicate, results);
        }
        Statement::GotoCase(gc) => match &gc.kind {
            crate::syntax::nodes::statements::goto_case_statement::GotoCaseKind::Case(e) => {
                visit_expr(e, predicate, results)
            }
            crate::syntax::nodes::statements::goto_case_statement::GotoCaseKind::Default => {}
        },
        Statement::Goto(_) | Statement::Break(_) | Statement::Continue(_) | Statement::Empty => {}
        Statement::Block(stmts) => {
            for s in stmts {
                collect_expressions(s, predicate, results);
            }
        }
    }
}
