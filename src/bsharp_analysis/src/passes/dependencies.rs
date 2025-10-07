use crate::artifacts::dependencies::{DependencyGraph, DependencyNodeType, DependencyType};
use crate::artifacts::symbols::{SymbolId, SymbolIndex};
use crate::framework::{AnalysisSession, AnalyzerPass, Phase};
use crate::syntax::ast::{CompilationUnit, TopLevelDeclaration};
use crate::syntax::nodes::declarations::{
    ClassBodyDeclaration, ClassDeclaration, NamespaceBodyDeclaration, NamespaceDeclaration,
};
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::invocation_expression::InvocationExpression;
use crate::syntax::nodes::expressions::member_access_expression::MemberAccessExpression;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::types::Type;
use bsharp_syntax::statements::statement::Statement;

pub struct DependenciesPass;

impl AnalyzerPass for DependenciesPass {
    fn id(&self) -> &'static str {
        "passes.dependencies"
    }
    fn phase(&self) -> Phase {
        Phase::Global
    }

    fn run(&self, cu: &CompilationUnit, session: &mut AnalysisSession) {
        let Some(symbols) = session.get_artifact::<SymbolIndex>() else {
            // Without symbols, we cannot build a SymbolId-based dependency graph
            return;
        };
        let mut graph = DependencyGraph::new();
        analyze_compilation_unit(cu, &symbols, &mut graph);
        session.insert_artifact(graph);
    }
}

fn analyze_compilation_unit(
    cu: &CompilationUnit,
    symbols: &SymbolIndex,
    graph: &mut DependencyGraph,
) {
    for decl in &cu.declarations {
        match decl {
            TopLevelDeclaration::Namespace(ns) => analyze_namespace(ns, symbols, graph),
            TopLevelDeclaration::Class(class) => add_class_dependencies(class, symbols, graph),
            _ => {}
        }
    }
}

fn analyze_namespace(
    ns: &NamespaceDeclaration,
    symbols: &SymbolIndex,
    graph: &mut DependencyGraph,
) {
    for member in &ns.declarations {
        match member {
            NamespaceBodyDeclaration::Namespace(inner) => analyze_namespace(inner, symbols, graph),
            NamespaceBodyDeclaration::Class(class) => add_class_dependencies(class, symbols, graph),
            _ => {}
        }
    }
}

fn add_class_dependencies(
    class: &ClassDeclaration,
    symbols: &SymbolIndex,
    graph: &mut DependencyGraph,
) {
    let class_name = class.name.name.clone();
    let class_id = match resolve_first_id(symbols, &class_name) {
        Some(id) => id,
        None => return,
    };
    graph.add_node(class_id, DependencyNodeType::Class);

    for bt in &class.base_types {
        if let Some(target) = type_name(bt) {
            if let Some(to_id) = resolve_first_id(symbols, &target) {
                graph.add_node(to_id, DependencyNodeType::Class);
                graph.add_dependency(class_id, to_id, DependencyType::Inheritance);
            }
        }
    }

    // Field type usage, method return/param types, and invocations
    for member in &class.body_declarations {
        match member {
            ClassBodyDeclaration::Field(field) => {
                if let Some(target) = type_name(&field.field_type) {
                    if let Some(to_id) = resolve_first_id(symbols, &target) {
                        graph.add_node(to_id, DependencyNodeType::Class);
                        graph.add_dependency(class_id, to_id, DependencyType::FieldAccess);
                    }
                }
            }
            ClassBodyDeclaration::Method(method) => {
                // Return type usage
                if let Some(target) = type_name(&method.return_type) {
                    if let Some(to_id) = resolve_first_id(symbols, &target) {
                        graph.add_node(to_id, DependencyNodeType::Class);
                        graph.add_dependency(class_id, to_id, DependencyType::Usage);
                    }
                }
                // Param types
                for p in &method.parameters {
                    if let Some(target) = type_name(&p.parameter_type) {
                        if let Some(to_id) = resolve_first_id(symbols, &target) {
                            graph.add_node(to_id, DependencyNodeType::Class);
                            graph.add_dependency(class_id, to_id, DependencyType::Usage);
                        }
                    }
                }
                // Invocations in body
                if let Some(body) = &method.body {
                    collect_invocations_from_statement(body, &mut |callee| {
                        if let Some(to_id) = resolve_first_id(symbols, &callee) {
                            graph.add_node(to_id, DependencyNodeType::Method);
                            graph.add_dependency(class_id, to_id, DependencyType::MethodCall);
                        }
                    });
                }
            }
            ClassBodyDeclaration::NestedClass(nested) => {
                add_class_dependencies(nested, symbols, graph)
            }
            _ => {}
        }
    }
}

fn resolve_first_id(symbols: &SymbolIndex, name: &str) -> Option<SymbolId> {
    if let Some(v) = symbols.get_ids_by_name(name) {
        return v.first().cloned();
    }
    if let Some(pos) = name.rfind('.') {
        let last = &name[pos + 1..];
        if let Some(v) = symbols.get_ids_by_name(last) {
            return v.first().cloned();
        }
    }
    None
}

fn type_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Reference(id) => Some(id.name.clone()),
        Type::Generic { base, .. } => Some(base.name.clone()),
        _ => None,
    }
}

fn collect_invocations_from_statement(stmt: &Statement, sink: &mut impl FnMut(String)) {
    use crate::syntax::nodes::statements::statement::Statement::*;
    match stmt {
        Expression(expr) => collect_invocations_from_expression(expr, sink),
        If(if_stmt) => {
            collect_invocations_from_expression(&if_stmt.condition, sink);
            collect_invocations_from_statement(&if_stmt.consequence, sink);
            if let Some(alt) = &if_stmt.alternative {
                collect_invocations_from_statement(alt, sink);
            }
        }
        For(for_stmt) => {
            for it in &for_stmt.iterator {
                collect_invocations_from_expression(it, sink);
            }
            if let Some(cond) = &for_stmt.condition {
                collect_invocations_from_expression(cond, sink);
            }
            collect_invocations_from_statement(&for_stmt.body, sink);
        }
        While(while_stmt) => {
            collect_invocations_from_expression(&while_stmt.condition, sink);
            collect_invocations_from_statement(&while_stmt.body, sink);
        }
        DoWhile(dw) => {
            collect_invocations_from_statement(&dw.body, sink);
            collect_invocations_from_expression(&dw.condition, sink);
        }
        Switch(sw) => {
            collect_invocations_from_expression(&sw.expression, sink);
            for sec in &sw.sections {
                for s in &sec.statements {
                    collect_invocations_from_statement(s, sink);
                }
            }
        }
        Try(try_stmt) => {
            collect_invocations_from_statement(&try_stmt.try_block, sink);
            for h in &try_stmt.catches {
                collect_invocations_from_statement(&h.block, sink);
            }
            if let Some(fin) = &try_stmt.finally_clause {
                collect_invocations_from_statement(&fin.block, sink);
            }
        }
        Block(stmts) => {
            for s in stmts {
                collect_invocations_from_statement(s, sink);
            }
        }
        _ => {}
    }
}
fn collect_invocations_from_expression(expr: &Expression, sink: &mut impl FnMut(String)) {
    match expr {
        Expression::Invocation(inv) => {
            if let Some(name) = invocation_target_name(inv) {
                sink(name);
            }
            for arg in &inv.arguments {
                collect_invocations_from_expression(&arg.expr, sink);
            }
        }
        Expression::MemberAccess(ma) => {
            collect_invocations_from_expression(&ma.object, sink);
        }
        Expression::Unary { expr, .. } | Expression::PostfixUnary { expr, .. } => {
            collect_invocations_from_expression(expr, sink)
        }
        Expression::Binary { left, right, .. } => {
            collect_invocations_from_expression(left, sink);
            collect_invocations_from_expression(right, sink);
        }
        Expression::Conditional(c) => {
            collect_invocations_from_expression(&c.condition, sink);
            collect_invocations_from_expression(&c.consequence, sink);
            collect_invocations_from_expression(&c.alternative, sink);
        }
        Expression::Assignment(a) => {
            collect_invocations_from_expression(&a.target, sink);
            collect_invocations_from_expression(&a.value, sink);
        }
        Expression::Indexing(ix) => {
            collect_invocations_from_expression(&ix.target, sink);
            collect_invocations_from_expression(&ix.index, sink);
        }
        Expression::Tuple(t) => {
            for e in &t.elements {
                collect_invocations_from_expression(&e.value, sink);
            }
        }
        Expression::Index(i) => collect_invocations_from_expression(&i.value, sink),
        Expression::NullConditional(nc) => {
            collect_invocations_from_expression(&nc.target, sink);
            if let Some(arg) = &nc.argument {
                collect_invocations_from_expression(arg, sink);
            }
        }
        Expression::Await(a) => collect_invocations_from_expression(&a.expr, sink),
        Expression::Cast { expression, .. } | Expression::As { expression, .. } => {
            collect_invocations_from_expression(expression, sink)
        }
        Expression::IsPattern { expression, .. } => {
            collect_invocations_from_expression(expression, sink)
        }
        Expression::Nameof(n) => collect_invocations_from_expression(&n.expr, sink),
        Expression::Ref(e) => collect_invocations_from_expression(e, sink),
        _ => {}
    }
}
fn invocation_target_name(inv: &InvocationExpression) -> Option<String> {
    // Try to produce a simple name for the callee (best-effort)
    match *inv.callee.as_ref() {
        Expression::MemberAccess(ref ma) => Some(member_access_path(ma)),
        Expression::Variable(Identifier { ref name }) => Some(name.clone()),
        _ => None,
    }
}
fn member_access_path(ma: &MemberAccessExpression) -> String {
    // Build a dotted path like obj.method
    let mut parts = Vec::new();
    parts.push(ma.member.name.clone());
    let mut current = &*ma.object;
    loop {
        match current {
            Expression::MemberAccess(inner) => {
                parts.push(inner.member.name.clone());
                current = &inner.object;
            }
            Expression::Variable(Identifier { name }) => {
                parts.push(name.clone());
                break;
            }
            _ => break,
        }
    }
    parts.reverse();
    parts.join(".")
}
