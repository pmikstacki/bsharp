use crate::artifacts::control_flow_graph::graph::build_cfg;
use crate::syntax::nodes::declarations::MethodDeclaration;
use crate::syntax::nodes::statements::statement::Statement;
use bsharp_syntax::expressions::binary_operator::BinaryOperator;
use bsharp_syntax::expressions::expression::Expression;
use bsharp_syntax::expressions::unary_operator::UnaryOperator;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Advanced complexity metrics beyond basic cyclomatic complexity
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    pub cyclomatic_complexity: usize,
    pub cognitive_complexity: usize,
    pub essential_complexity: usize,
    pub abc_complexity: ABCComplexity,
    pub halstead_metrics: HalsteadMetrics,
    pub max_nesting_depth: usize,
}

/// ABC (Assignment, Branch, Condition) complexity metrics
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ABCComplexity {
    pub assignments: usize,
    pub branches: usize,
    pub conditions: usize,
}

impl ABCComplexity {
    pub fn magnitude(&self) -> f64 {
        ((self.assignments.pow(2) + self.branches.pow(2) + self.conditions.pow(2)) as f64).sqrt()
    }
}

/// Halstead complexity metrics
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HalsteadMetrics {
    pub distinct_operators: usize,
    pub distinct_operands: usize,
    pub total_operators: usize,
    pub total_operands: usize,
}

impl HalsteadMetrics {
    /// Calculate program vocabulary
    pub fn vocabulary(&self) -> usize {
        self.distinct_operators + self.distinct_operands
    }

    /// Calculate program length
    pub fn length(&self) -> usize {
        self.total_operators + self.total_operands
    }

    /// Calculate calculated length
    pub fn calculated_length(&self) -> f64 {
        if self.distinct_operators == 0 || self.distinct_operands == 0 {
            0.0
        } else {
            (self.distinct_operators as f64) * (self.distinct_operators as f64).log2()
                + (self.distinct_operands as f64) * (self.distinct_operands as f64).log2()
        }
    }

    /// Calculate volume
    pub fn volume(&self) -> f64 {
        if self.vocabulary() == 0 {
            0.0
        } else {
            (self.length() as f64) * (self.vocabulary() as f64).log2()
        }
    }

    /// Calculate difficulty
    pub fn difficulty(&self) -> f64 {
        if self.distinct_operands == 0 || self.total_operands == 0 {
            0.0
        } else {
            ((self.distinct_operators as f64) / 2.0)
                * ((self.total_operands as f64) / (self.distinct_operands as f64))
        }
    }

    /// Calculate effort
    pub fn effort(&self) -> f64 {
        self.difficulty() * self.volume()
    }
}

/// Complexity analyzer for calculating various complexity metrics
#[derive(Debug, PartialEq)]
pub struct ComplexityAnalyzer;

impl ComplexityAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Calculate comprehensive complexity metrics for a method
    pub fn analyze_method(&self, method: &MethodDeclaration) -> ComplexityMetrics {
        let mut metrics = ComplexityMetrics::default();

        if let Some(body) = &method.body {
            metrics.cyclomatic_complexity = Self::calculate_cyclomatic_complexity(body, 1);
            metrics.cognitive_complexity = Self::calculate_cognitive_complexity(body, 0, 0);
            metrics.max_nesting_depth = Self::calculate_max_nesting_depth(body, 0);
            metrics.abc_complexity = Self::calculate_abc_complexity(body);
            metrics.halstead_metrics = Self::calculate_halstead_metrics(body);
            let cfg = build_cfg(body);
            metrics.essential_complexity = cfg.essential_complexity();
        } else {
            metrics.cyclomatic_complexity = 1; // Base complexity for methods without body
        }

        metrics
    }

    /// Calculate cyclomatic complexity (McCabe)
    pub fn calculate_cyclomatic_complexity(stmt: &Statement, base_complexity: usize) -> usize {
        match stmt {
            Statement::If(if_stmt) => {
                let mut complexity = base_complexity + 1; // +1 for if
                complexity += Self::calculate_cyclomatic_complexity(&if_stmt.consequence, 0);
                if let Some(alt) = &if_stmt.alternative {
                    complexity += Self::calculate_cyclomatic_complexity(alt, 0);
                }
                complexity
            }
            Statement::For(for_stmt) => {
                base_complexity + 1 + Self::calculate_cyclomatic_complexity(&for_stmt.body, 0)
            }
            Statement::While(while_stmt) => {
                base_complexity + 1 + Self::calculate_cyclomatic_complexity(&while_stmt.body, 0)
            }
            Statement::DoWhile(do_while_stmt) => {
                base_complexity + 1 + Self::calculate_cyclomatic_complexity(&do_while_stmt.body, 0)
            }
            Statement::Switch(switch_stmt) => {
                let mut complexity = base_complexity + switch_stmt.sections.len(); // Each case adds complexity
                for section in &switch_stmt.sections {
                    for s in &section.statements {
                        complexity += Self::calculate_cyclomatic_complexity(s, 0);
                    }
                }
                complexity
            }
            Statement::Try(try_stmt) => {
                // +1 for try presence, +1 per catch arm
                let mut complexity = base_complexity + 1 + try_stmt.catches.len();
                complexity += Self::calculate_cyclomatic_complexity(&try_stmt.try_block, 0);
                for c in &try_stmt.catches {
                    complexity += Self::calculate_cyclomatic_complexity(&c.block, 0);
                }
                if let Some(fin) = &try_stmt.finally_clause {
                    complexity += Self::calculate_cyclomatic_complexity(&fin.block, 0);
                }
                complexity
            }
            Statement::Block(statements) => {
                let mut complexity = base_complexity;
                for stmt in statements {
                    complexity += Self::calculate_cyclomatic_complexity(stmt, 0);
                }
                complexity
            }
            _ => base_complexity,
        }
    }

    /// Calculate cognitive complexity (SonarSource methodology)
    pub fn calculate_cognitive_complexity(
        stmt: &Statement,
        current_depth: usize,
        base_complexity: usize,
    ) -> usize {
        match stmt {
            Statement::If(if_stmt) => {
                let mut complexity = base_complexity + 1 + current_depth; // +1 for if, +nesting
                complexity += Self::calculate_cognitive_complexity(
                    &if_stmt.consequence,
                    current_depth + 1,
                    0,
                );
                if let Some(alt) = &if_stmt.alternative {
                    if matches!(**alt, Statement::If(_)) {
                        // else if doesn't add nesting
                        complexity += Self::calculate_cognitive_complexity(alt, current_depth, 0);
                    } else {
                        // else adds nesting
                        complexity +=
                            Self::calculate_cognitive_complexity(alt, current_depth + 1, 0);
                    }
                }
                complexity
            }
            Statement::For(for_stmt) => {
                base_complexity
                    + 1
                    + current_depth
                    + Self::calculate_cognitive_complexity(&for_stmt.body, current_depth + 1, 0)
            }
            Statement::While(while_stmt) => {
                base_complexity
                    + 1
                    + current_depth
                    + Self::calculate_cognitive_complexity(&while_stmt.body, current_depth + 1, 0)
            }
            Statement::DoWhile(do_while_stmt) => {
                base_complexity
                    + 1
                    + current_depth
                    + Self::calculate_cognitive_complexity(
                        &do_while_stmt.body,
                        current_depth + 1,
                        0,
                    )
            }
            Statement::Switch(switch_stmt) => {
                let mut complexity = base_complexity + 1 + current_depth; // +1 for switch, +nesting
                for section in &switch_stmt.sections {
                    for s in &section.statements {
                        complexity += Self::calculate_cognitive_complexity(s, current_depth + 1, 0);
                    }
                }
                complexity
            }
            Statement::Try(try_stmt) => {
                let mut complexity = base_complexity + 1 + current_depth;
                complexity +=
                    Self::calculate_cognitive_complexity(&try_stmt.try_block, current_depth + 1, 0);
                for c in &try_stmt.catches {
                    complexity +=
                        Self::calculate_cognitive_complexity(&c.block, current_depth + 1, 0);
                }
                if let Some(fin) = &try_stmt.finally_clause {
                    complexity +=
                        Self::calculate_cognitive_complexity(&fin.block, current_depth + 1, 0);
                }
                complexity
            }
            Statement::Block(statements) => {
                let mut complexity = base_complexity;
                for stmt in statements {
                    complexity += Self::calculate_cognitive_complexity(stmt, current_depth, 0);
                }
                complexity
            }
            Statement::Break(_) => {
                base_complexity + 1 // Break adds cognitive load but no nesting penalty
            }
            Statement::Continue(_) => {
                let nesting_bonus = if current_depth > 0 { 1 } else { 0 };
                base_complexity + 1 + nesting_bonus // Continue adds cognitive load plus limited nesting penalty
            }
            _ => base_complexity,
        }
    }

    /// Calculate maximum nesting depth
    pub fn calculate_max_nesting_depth(stmt: &Statement, current_depth: usize) -> usize {
        match stmt {
            Statement::If(if_stmt) => {
                let new_depth = current_depth + 1;
                let consequence_depth =
                    Self::calculate_max_nesting_depth(&if_stmt.consequence, new_depth);
                let alternative_depth = if let Some(alt) = &if_stmt.alternative {
                    Self::calculate_max_nesting_depth(alt, new_depth)
                } else {
                    new_depth
                };
                consequence_depth.max(alternative_depth)
            }
            Statement::For(for_stmt) => {
                Self::calculate_max_nesting_depth(&for_stmt.body, current_depth + 1)
            }
            Statement::While(while_stmt) => {
                Self::calculate_max_nesting_depth(&while_stmt.body, current_depth + 1)
            }
            Statement::DoWhile(do_while_stmt) => {
                Self::calculate_max_nesting_depth(&do_while_stmt.body, current_depth + 1)
            }
            Statement::Switch(switch_stmt) => {
                let mut max_depth = current_depth + 1;
                for section in &switch_stmt.sections {
                    for s in &section.statements {
                        let depth = Self::calculate_max_nesting_depth(s, current_depth + 1);
                        max_depth = max_depth.max(depth);
                    }
                }
                max_depth
            }
            Statement::Block(statements) => {
                let mut max_depth = current_depth;
                for stmt in statements {
                    let depth = Self::calculate_max_nesting_depth(stmt, current_depth);
                    max_depth = max_depth.max(depth);
                }
                max_depth
            }
            _ => current_depth,
        }
    }

    /// Calculate ABC complexity metrics
    pub fn calculate_abc_complexity(stmt: &Statement) -> ABCComplexity {
        let mut abc = ABCComplexity::default();
        Self::collect_abc_complexity(stmt, &mut abc);
        abc
    }

    fn collect_abc_complexity(stmt: &Statement, abc: &mut ABCComplexity) {
        match stmt {
            Statement::If(if_stmt) => {
                abc.conditions += 1;
                abc.branches += 1;
                Self::collect_abc_complexity(&if_stmt.consequence, abc);
                if let Some(alt) = &if_stmt.alternative {
                    Self::collect_abc_complexity(alt, abc);
                }
            }
            Statement::For(for_stmt) => {
                abc.conditions += 1;
                abc.branches += 1;
                Self::collect_abc_complexity(&for_stmt.body, abc);
            }
            Statement::While(while_stmt) => {
                abc.conditions += 1;
                abc.branches += 1;
                Self::collect_abc_complexity(&while_stmt.body, abc);
            }
            Statement::DoWhile(do_while_stmt) => {
                abc.conditions += 1;
                abc.branches += 1;
                Self::collect_abc_complexity(&do_while_stmt.body, abc);
            }
            Statement::Switch(switch_stmt) => {
                abc.conditions += 1;
                abc.branches += switch_stmt.sections.len();
                for section in &switch_stmt.sections {
                    for s in &section.statements {
                        Self::collect_abc_complexity(s, abc);
                    }
                }
            }
            Statement::Expression(expr) => {
                // Count any expression statement as an assignment-like action in ABC terms
                abc.assignments += 1;
                Self::collect_abc_from_expression(expr, abc);
            }
            Statement::Block(statements) => {
                for stmt in statements {
                    Self::collect_abc_complexity(stmt, abc);
                }
            }
            _ => {}
        }
    }

    fn collect_abc_from_expression(expr: &Expression, abc: &mut ABCComplexity) {
        // Count real assignments and increments/decrements
        match expr {
            Expression::Assignment(a) => {
                match a.op {
                    BinaryOperator::Assign
                    | BinaryOperator::AddAssign
                    | BinaryOperator::SubtractAssign
                    | BinaryOperator::MultiplyAssign
                    | BinaryOperator::DivideAssign
                    | BinaryOperator::ModuloAssign
                    | BinaryOperator::AndAssign
                    | BinaryOperator::OrAssign
                    | BinaryOperator::XorAssign
                    | BinaryOperator::LeftShiftAssign
                    | BinaryOperator::RightShiftAssign
                    | BinaryOperator::UnsignedRightShiftAssign
                    | BinaryOperator::NullCoalescingAssign => {
                        abc.assignments += 1;
                    }
                    _ => {}
                }
                // Recurse into RHS for nested assignments
                Self::collect_abc_from_expression(&a.value, abc);
            }
            Expression::Unary { op, expr: inner } | Expression::PostfixUnary { op, expr: inner } => {
                if matches!(op, UnaryOperator::Increment | UnaryOperator::Decrement) {
                    abc.assignments += 1;
                }
                Self::collect_abc_from_expression(inner, abc);
            }
            Expression::Binary { left, right, .. } => {
                Self::collect_abc_from_expression(left, abc);
                Self::collect_abc_from_expression(right, abc);
            }
            Expression::Invocation(inv) => {
                Self::collect_abc_from_expression(&inv.callee, abc);
                for a in &inv.arguments {
                    Self::collect_abc_from_expression(&a.expr, abc);
                }
            }
            Expression::MemberAccess(m) => {
                Self::collect_abc_from_expression(&m.object, abc);
            }
            Expression::Indexing(ix) => {
                Self::collect_abc_from_expression(&ix.target, abc);
                Self::collect_abc_from_expression(&ix.index, abc);
            }
            Expression::Conditional(c) => {
                // Count a condition in ABC for ternary
                abc.conditions += 1;
                abc.branches += 1;
                Self::collect_abc_from_expression(&c.condition, abc);
                Self::collect_abc_from_expression(&c.consequence, abc);
                Self::collect_abc_from_expression(&c.alternative, abc);
            }
            Expression::SwitchExpression(se) => {
                abc.conditions += 1;
                abc.branches += se.arms.len();
                Self::collect_abc_from_expression(&se.expression, abc);
                for arm in &se.arms {
                    if let Some(when) = &arm.when_clause {
                        // treat when as a condition
                        abc.conditions += 1;
                        Self::collect_abc_from_expression(when, abc);
                    }
                    Self::collect_abc_from_expression(&arm.expression, abc);
                }
            }
            Expression::New(n) => {
                for a in &n.arguments {
                    Self::collect_abc_from_expression(a, abc);
                }
            }
            Expression::NullConditional(nc) => {
                Self::collect_abc_from_expression(&nc.target, abc);
            }
            Expression::Tuple(t) => {
                for e in &t.elements {
                    Self::collect_abc_from_expression(&e.value, abc);
                }
            }
            Expression::Range(r) => {
                if let Some(s) = &r.start {
                    Self::collect_abc_from_expression(s, abc);
                }
                if let Some(e) = &r.end {
                    Self::collect_abc_from_expression(e, abc);
                }
            }
            Expression::AnonymousObject(obj) => {
                for m in &obj.initializers {
                    Self::collect_abc_from_expression(&m.value, abc);
                }
            }
            Expression::With { target, initializers } => {
                Self::collect_abc_from_expression(target, abc);
                for init in initializers {
                    match init {
                        bsharp_syntax::expressions::expression::WithInitializerEntry::Property { value, .. } => Self::collect_abc_from_expression(value, abc),
                        bsharp_syntax::expressions::expression::WithInitializerEntry::Indexer { indices, value } => {
                            for idx in indices { Self::collect_abc_from_expression(idx, abc); }
                            Self::collect_abc_from_expression(value, abc);
                        }
                    }
                }
            }
            Expression::Checked(e) => Self::collect_abc_from_expression(&e.expr, abc),
            Expression::Unchecked(e) => Self::collect_abc_from_expression(&e.expr, abc),
            Expression::Await(a) => Self::collect_abc_from_expression(&a.expr, abc),
            Expression::Throw(t) => {
                if let Some(e) = &t.expr { Self::collect_abc_from_expression(e, abc); }
            }
            Expression::Deconstruction(d) => Self::collect_abc_from_expression(&d.value, abc),
            Expression::Index(i) => { Self::collect_abc_from_expression(&i.value, abc); },
            Expression::StackAlloc(sa) => {
                if let Some(c) = &sa.count { Self::collect_abc_from_expression(c, abc); }
                if let Some(init) = &sa.initializer {
                    for e in init { Self::collect_abc_from_expression(e, abc); }
                }
            }
            Expression::Pattern(p) => {
                // patterns contain expressions; skip detailed handling for now
                let _ = p;
            }
            Expression::Query(q) => {
                Self::collect_abc_from_expression(&q.from.expression, abc);
                for clause in &q.body {
                    use bsharp_syntax::expressions::query_expression::QueryClause::*;
                    match clause {
                        From(c) => Self::collect_abc_from_expression(&c.expression, abc),
                        Let(c) => Self::collect_abc_from_expression(&c.expression, abc),
                        Where(c) => {
                            abc.conditions += 1;
                            Self::collect_abc_from_expression(&c.condition, abc)
                        }
                        Join(c) => {
                            Self::collect_abc_from_expression(&c.in_expression, abc);
                            Self::collect_abc_from_expression(&c.on_expression, abc);
                            Self::collect_abc_from_expression(&c.equals_expression, abc);
                        }
                        OrderBy(c) => {
                            for ord in &c.orderings {
                                Self::collect_abc_from_expression(&ord.expression, abc);
                            }
                        }
                    }
                }
                match &q.select_or_group {
                    bsharp_syntax::expressions::query_expression::QuerySelectOrGroup::Select(e) => {
                        Self::collect_abc_from_expression(e, abc)
                    }
                    bsharp_syntax::expressions::query_expression::QuerySelectOrGroup::Group { element, by } => {
                        Self::collect_abc_from_expression(element, abc);
                        Self::collect_abc_from_expression(by, abc);
                    }
                }
                if let Some(cont) = &q.continuation {
                    for clause in &cont.body {
                        use bsharp_syntax::expressions::query_expression::QueryClause::*;
                        match clause {
                            From(c) => Self::collect_abc_from_expression(&c.expression, abc),
                            Let(c) => Self::collect_abc_from_expression(&c.expression, abc),
                            Where(c) => {
                                abc.conditions += 1;
                                Self::collect_abc_from_expression(&c.condition, abc)
                            }
                            Join(c) => {
                                Self::collect_abc_from_expression(&c.in_expression, abc);
                                Self::collect_abc_from_expression(&c.on_expression, abc);
                                Self::collect_abc_from_expression(&c.equals_expression, abc);
                            }
                            OrderBy(c) => {
                                for ord in &c.orderings {
                                    Self::collect_abc_from_expression(&ord.expression, abc);
                                }
                            }
                        }
                    }
                    match &cont.select_or_group {
                        bsharp_syntax::expressions::query_expression::QuerySelectOrGroup::Select(e) => Self::collect_abc_from_expression(e, abc),
                        bsharp_syntax::expressions::query_expression::QuerySelectOrGroup::Group { element, by } => {
                            Self::collect_abc_from_expression(element, abc);
                            Self::collect_abc_from_expression(by, abc);
                        }
                    }
                }
            }
            Expression::IsPattern { expression, .. } => {
                Self::collect_abc_from_expression(expression, abc);
            }
            Expression::As { expression, .. } => {
                Self::collect_abc_from_expression(expression, abc);
            }
            Expression::Cast { expression, .. } => {
                Self::collect_abc_from_expression(expression, abc);
            }
            Expression::AnonymousMethod(_) | Expression::Lambda(_) => {}
            Expression::Literal(_) | Expression::Variable(_) | Expression::This | Expression::Base
            | Expression::Typeof(_) | Expression::Sizeof(_) | Expression::Default(_) | Expression::Nameof(_) | Expression::Ref(_) | Expression::Collection(_) => {}
        }
    }

    fn calculate_halstead_metrics(stmt: &Statement) -> HalsteadMetrics {
        let mut ops_unique: HashSet<String> = HashSet::new();
        let mut opr_unique: HashSet<String> = HashSet::new();
        let mut total_ops: usize = 0;
        let mut total_opr: usize = 0;

        fn emit_op(ops_unique: &mut HashSet<String>, total_ops: &mut usize, op: &str) {
            ops_unique.insert(op.to_string());
            *total_ops += 1;
        }
        fn emit_operand(opr_unique: &mut HashSet<String>, total_opr: &mut usize, name: &str) {
            opr_unique.insert(name.to_string());
            *total_opr += 1;
        }

        fn walk_expr(
            e: &Expression,
            ops_unique: &mut HashSet<String>,
            opr_unique: &mut HashSet<String>,
            total_ops: &mut usize,
            total_opr: &mut usize,
        ) {
            use Expression::*;
            match e {
                Literal(l) => {
                    emit_operand(opr_unique, total_opr, &format!("lit:{:?}", l));
                }
                Variable(id) => emit_operand(opr_unique, total_opr, &id.name),
                Assignment(a) => {
                    // operator is assignment variant
                    emit_op(ops_unique, total_ops, match a.op {
                        BinaryOperator::Assign => "=",
                        BinaryOperator::AddAssign => "+=",
                        BinaryOperator::SubtractAssign => "-=",
                        BinaryOperator::MultiplyAssign => "*=",
                        BinaryOperator::DivideAssign => "/=",
                        BinaryOperator::ModuloAssign => "%=",
                        BinaryOperator::AndAssign => "&=",
                        BinaryOperator::OrAssign => "|=",
                        BinaryOperator::XorAssign => "^=",
                        BinaryOperator::LeftShiftAssign => "<<=",
                        BinaryOperator::RightShiftAssign => ">>=",
                        BinaryOperator::UnsignedRightShiftAssign => ">>>=",
                        BinaryOperator::NullCoalescingAssign => "??=",
                        _ => "assign",
                    });
                    walk_expr(&a.target, ops_unique, opr_unique, total_ops, total_opr);
                    walk_expr(&a.value, ops_unique, opr_unique, total_ops, total_opr);
                }
                Unary { op, expr } | PostfixUnary { op, expr } => {
                    let name = match op {
                        UnaryOperator::Plus => "+",
                        UnaryOperator::Minus => "-",
                        UnaryOperator::LogicalNot => "!",
                        UnaryOperator::BitwiseNot => "~",
                        UnaryOperator::Increment => "++",
                        UnaryOperator::Decrement => "--",
                        UnaryOperator::AddressOf => "&",
                        UnaryOperator::PointerIndirection => "*",
                        UnaryOperator::IndexFromEnd => "^",
                        UnaryOperator::NullForgiving => "!",
                    };
                    emit_op(ops_unique, total_ops, name);
                    walk_expr(expr, ops_unique, opr_unique, total_ops, total_opr);
                }
                Binary { left, op, right } => {
                    let name = match op {
                        BinaryOperator::Add => "+",
                        BinaryOperator::Subtract => "-",
                        BinaryOperator::Multiply => "*",
                        BinaryOperator::Divide => "/",
                        BinaryOperator::Modulo => "%",
                        BinaryOperator::Equal => "==",
                        BinaryOperator::NotEqual => "!=",
                        BinaryOperator::LessThan => "<",
                        BinaryOperator::GreaterThan => ">",
                        BinaryOperator::LessEqual => "<=",
                        BinaryOperator::GreaterEqual => ">=",
                        BinaryOperator::Is => "is",
                        BinaryOperator::As => "as",
                        BinaryOperator::LogicalAnd => "&&",
                        BinaryOperator::LogicalOr => "||",
                        BinaryOperator::BitwiseAnd => "&",
                        BinaryOperator::BitwiseOr => "|",
                        BinaryOperator::BitwiseXor => "^",
                        BinaryOperator::LeftShift => "<<",
                        BinaryOperator::RightShift => ">>",
                        BinaryOperator::UnsignedRightShift => ">>>",
                        BinaryOperator::NullCoalescing => "??",
                        BinaryOperator::Range => "..",
                        _ => "bin",
                    };
                    emit_op(ops_unique, total_ops, name);
                    walk_expr(left, ops_unique, opr_unique, total_ops, total_opr);
                    walk_expr(right, ops_unique, opr_unique, total_ops, total_opr);
                }
                MemberAccess(m) => {
                    emit_op(ops_unique, total_ops, ".");
                    walk_expr(&m.object, ops_unique, opr_unique, total_ops, total_opr);
                }
                Invocation(inv) => {
                    emit_op(ops_unique, total_ops, "call");
                    walk_expr(&inv.callee, ops_unique, opr_unique, total_ops, total_opr);
                    for a in &inv.arguments {
                        walk_expr(&a.expr, ops_unique, opr_unique, total_ops, total_opr);
                    }
                }
                Indexing(ix) => {
                    emit_op(ops_unique, total_ops, "[]");
                    walk_expr(&ix.target, ops_unique, opr_unique, total_ops, total_opr);
                    walk_expr(&ix.index, ops_unique, opr_unique, total_ops, total_opr);
                }
                Conditional(c) => {
                    emit_op(ops_unique, total_ops, "?:");
                    walk_expr(&c.condition, ops_unique, opr_unique, total_ops, total_opr);
                    walk_expr(&c.consequence, ops_unique, opr_unique, total_ops, total_opr);
                    walk_expr(&c.alternative, ops_unique, opr_unique, total_ops, total_opr);
                }
                SwitchExpression(se) => {
                    emit_op(ops_unique, total_ops, "switch-expr");
                    walk_expr(&se.expression, ops_unique, opr_unique, total_ops, total_opr);
                    for arm in &se.arms {
                        if let Some(when) = &arm.when_clause {
                            walk_expr(when, ops_unique, opr_unique, total_ops, total_opr);
                        }
                        walk_expr(&arm.expression, ops_unique, opr_unique, total_ops, total_opr);
                    }
                }
                New(n) => {
                    emit_op(ops_unique, total_ops, "new");
                    for a in &n.arguments {
                        walk_expr(a, ops_unique, opr_unique, total_ops, total_opr);
                    }
                }
                NullConditional(nc) => {
                    emit_op(ops_unique, total_ops, "?.");
                    walk_expr(&nc.target, ops_unique, opr_unique, total_ops, total_opr);
                }
                Tuple(t) => {
                    for e in &t.elements {
                        walk_expr(&e.value, ops_unique, opr_unique, total_ops, total_opr);
                    }
                }
                Range(r) => {
                    if let Some(s) = &r.start { walk_expr(s, ops_unique, opr_unique, total_ops, total_opr); }
                    if let Some(e) = &r.end { walk_expr(e, ops_unique, opr_unique, total_ops, total_opr); }
                }
                AnonymousObject(obj) => {
                    for m in &obj.initializers {
                        walk_expr(&m.value, ops_unique, opr_unique, total_ops, total_opr);
                    }
                }
                With { target, initializers } => {
                    walk_expr(target, ops_unique, opr_unique, total_ops, total_opr);
                    for init in initializers {
                        match init {
                            bsharp_syntax::expressions::expression::WithInitializerEntry::Property { value, .. } => walk_expr(value, ops_unique, opr_unique, total_ops, total_opr),
                            bsharp_syntax::expressions::expression::WithInitializerEntry::Indexer { indices, value } => {
                                for idx in indices { walk_expr(idx, ops_unique, opr_unique, total_ops, total_opr); }
                                walk_expr(value, ops_unique, opr_unique, total_ops, total_opr);
                            }
                        }
                    }
                }
                Checked(e) => walk_expr(&e.expr, ops_unique, opr_unique, total_ops, total_opr),
                Unchecked(e) => walk_expr(&e.expr, ops_unique, opr_unique, total_ops, total_opr),
                Await(a) => walk_expr(&a.expr, ops_unique, opr_unique, total_ops, total_opr),
                Throw(te) => {
                    emit_op(ops_unique, total_ops, "throw");
                    if let Some(x) = &te.expr { walk_expr(x, ops_unique, opr_unique, total_ops, total_opr); }
                }
                Deconstruction(d) => walk_expr(&d.value, ops_unique, opr_unique, total_ops, total_opr),
                Index(i) => { walk_expr(&i.value, ops_unique, opr_unique, total_ops, total_opr); }
                StackAlloc(sa) => {
                    if let Some(c) = &sa.count { walk_expr(c, ops_unique, opr_unique, total_ops, total_opr); }
                    if let Some(init) = &sa.initializer {
                        for e in init { walk_expr(e, ops_unique, opr_unique, total_ops, total_opr); }
                    }
                }
                Query(q) => {
                    walk_expr(&q.from.expression, ops_unique, opr_unique, total_ops, total_opr);
                    for clause in &q.body {
                        use bsharp_syntax::expressions::query_expression::QueryClause::*;
                        match clause {
                            From(c) => walk_expr(&c.expression, ops_unique, opr_unique, total_ops, total_opr),
                            Let(c) => walk_expr(&c.expression, ops_unique, opr_unique, total_ops, total_opr),
                            Where(c) => walk_expr(&c.condition, ops_unique, opr_unique, total_ops, total_opr),
                            Join(c) => {
                                walk_expr(&c.in_expression, ops_unique, opr_unique, total_ops, total_opr);
                                walk_expr(&c.on_expression, ops_unique, opr_unique, total_ops, total_opr);
                                walk_expr(&c.equals_expression, ops_unique, opr_unique, total_ops, total_opr);
                            }
                            OrderBy(c) => {
                                for ord in &c.orderings {
                                    walk_expr(&ord.expression, ops_unique, opr_unique, total_ops, total_opr);
                                }
                            }
                        }
                    }
                    match &q.select_or_group {
                        bsharp_syntax::expressions::query_expression::QuerySelectOrGroup::Select(e) => walk_expr(e, ops_unique, opr_unique, total_ops, total_opr),
                        bsharp_syntax::expressions::query_expression::QuerySelectOrGroup::Group { element, by } => {
                            walk_expr(element, ops_unique, opr_unique, total_ops, total_opr);
                            walk_expr(by, ops_unique, opr_unique, total_ops, total_opr);
                        }
                    }
                    if let Some(cont) = &q.continuation {
                        for clause in &cont.body {
                            use bsharp_syntax::expressions::query_expression::QueryClause::*;
                            match clause {
                                From(c) => walk_expr(&c.expression, ops_unique, opr_unique, total_ops, total_opr),
                                Let(c) => walk_expr(&c.expression, ops_unique, opr_unique, total_ops, total_opr),
                                Where(c) => walk_expr(&c.condition, ops_unique, opr_unique, total_ops, total_opr),
                                Join(c) => {
                                    walk_expr(&c.in_expression, ops_unique, opr_unique, total_ops, total_opr);
                                    walk_expr(&c.on_expression, ops_unique, opr_unique, total_ops, total_opr);
                                    walk_expr(&c.equals_expression, ops_unique, opr_unique, total_ops, total_opr);
                                }
                                OrderBy(c) => {
                                    for ord in &c.orderings {
                                        walk_expr(&ord.expression, ops_unique, opr_unique, total_ops, total_opr);
                                    }
                                }
                            }
                        }
                        match &cont.select_or_group {
                            bsharp_syntax::expressions::query_expression::QuerySelectOrGroup::Select(e) => walk_expr(e, ops_unique, opr_unique, total_ops, total_opr),
                            bsharp_syntax::expressions::query_expression::QuerySelectOrGroup::Group { element, by } => {
                                walk_expr(element, ops_unique, opr_unique, total_ops, total_opr);
                                walk_expr(by, ops_unique, opr_unique, total_ops, total_opr);
                            }
                        }
                    }
                }
                IsPattern { expression, .. } => {
                    walk_expr(expression, ops_unique, opr_unique, total_ops, total_opr);
                }
                As { expression, .. } => {
                    walk_expr(expression, ops_unique, opr_unique, total_ops, total_opr);
                }
                Cast { expression, .. } => {
                    walk_expr(expression, ops_unique, opr_unique, total_ops, total_opr);
                }
                AnonymousMethod(_) | Lambda(_) | Nameof(_) | Typeof(_) | Sizeof(_) | Default(_) | Ref(_) | Collection(_) | This | Base | Pattern(_) => {}
            }
        }

        fn walk_stmt(
            s: &Statement,
            ops_unique: &mut HashSet<String>,
            opr_unique: &mut HashSet<String>,
            total_ops: &mut usize,
            total_opr: &mut usize,
        ) {
            match s {
                Statement::If(if_stmt) => {
                    emit_op(ops_unique, total_ops, "if");
                    walk_expr(&if_stmt.condition, ops_unique, opr_unique, total_ops, total_opr);
                    walk_stmt(&if_stmt.consequence, ops_unique, opr_unique, total_ops, total_opr);
                    if let Some(alt) = &if_stmt.alternative {
                        emit_op(ops_unique, total_ops, "else");
                        walk_stmt(alt, ops_unique, opr_unique, total_ops, total_opr);
                    }
                }
                Statement::For(for_stmt) => {
                    emit_op(ops_unique, total_ops, "for");
                    walk_stmt(&for_stmt.body, ops_unique, opr_unique, total_ops, total_opr);
                }
                Statement::ForEach(fe) => {
                    emit_op(ops_unique, total_ops, "foreach");
                    walk_stmt(&fe.body, ops_unique, opr_unique, total_ops, total_opr);
                }
                Statement::While(while_stmt) => {
                    emit_op(ops_unique, total_ops, "while");
                    walk_expr(&while_stmt.condition, ops_unique, opr_unique, total_ops, total_opr);
                    walk_stmt(&while_stmt.body, ops_unique, opr_unique, total_ops, total_opr);
                }
                Statement::DoWhile(dw) => {
                    emit_op(ops_unique, total_ops, "do");
                    walk_stmt(&dw.body, ops_unique, opr_unique, total_ops, total_opr);
                    emit_op(ops_unique, total_ops, "while");
                    walk_expr(&dw.condition, ops_unique, opr_unique, total_ops, total_opr);
                }
                Statement::Switch(sw) => {
                    emit_op(ops_unique, total_ops, "switch");
                    walk_expr(&sw.expression, ops_unique, opr_unique, total_ops, total_opr);
                    for section in &sw.sections {
                        for st in &section.statements {
                            walk_stmt(st, ops_unique, opr_unique, total_ops, total_opr);
                        }
                    }
                }
                Statement::Try(try_stmt) => {
                    emit_op(ops_unique, total_ops, "try");
                    walk_stmt(&try_stmt.try_block, ops_unique, opr_unique, total_ops, total_opr);
                    for c in &try_stmt.catches {
                        emit_op(ops_unique, total_ops, "catch");
                        if let Some(expr) = &c.when_clause {
                            walk_expr(expr, ops_unique, opr_unique, total_ops, total_opr);
                        }
                        walk_stmt(&c.block, ops_unique, opr_unique, total_ops, total_opr);
                    }
                    if let Some(fin) = &try_stmt.finally_clause {
                        emit_op(ops_unique, total_ops, "finally");
                        walk_stmt(&fin.block, ops_unique, opr_unique, total_ops, total_opr);
                    }
                }
                Statement::Lock(lock_stmt) => {
                    emit_op(ops_unique, total_ops, "lock");
                    walk_expr(&lock_stmt.expr, ops_unique, opr_unique, total_ops, total_opr);
                    walk_stmt(&lock_stmt.body, ops_unique, opr_unique, total_ops, total_opr);
                }
                Statement::Using(u) => {
                    emit_op(ops_unique, total_ops, "using");
                    if let Some(b) = &u.body { walk_stmt(b, ops_unique, opr_unique, total_ops, total_opr); }
                }
                Statement::Checked(c) => {
                    emit_op(ops_unique, total_ops, "checked");
                    walk_stmt(&c.body, ops_unique, opr_unique, total_ops, total_opr);
                }
                Statement::Unchecked(u) => {
                    emit_op(ops_unique, total_ops, "unchecked");
                    walk_stmt(&u.body, ops_unique, opr_unique, total_ops, total_opr);
                }
                Statement::Unsafe(us) => {
                    emit_op(ops_unique, total_ops, "unsafe");
                    walk_stmt(&us.body, ops_unique, opr_unique, total_ops, total_opr);
                }
                Statement::Fixed(fx) => {
                    emit_op(ops_unique, total_ops, "fixed");
                    walk_stmt(&fx.body, ops_unique, opr_unique, total_ops, total_opr);
                }
                Statement::LocalFunction(lf) => {
                    walk_stmt(&lf.body, ops_unique, opr_unique, total_ops, total_opr);
                }
                Statement::Expression(e) => {
                    walk_expr(e, ops_unique, opr_unique, total_ops, total_opr);
                }
                Statement::Yield(_) | Statement::Declaration(_) | Statement::Label(_) | Statement::Goto(_) | Statement::GotoCase(_) | Statement::Break(_) | Statement::Continue(_) | Statement::Return(_) | Statement::Throw(_) | Statement::Empty | Statement::Deconstruction(_) => {}
                Statement::Block(stmts) => {
                    for s in stmts { walk_stmt(s, ops_unique, opr_unique, total_ops, total_opr); }
                }
            }
        }

        walk_stmt(stmt, &mut ops_unique, &mut opr_unique, &mut total_ops, &mut total_opr);

        HalsteadMetrics {
            distinct_operators: ops_unique.len(),
            distinct_operands: opr_unique.len(),
            total_operators: total_ops,
            total_operands: total_opr,
        }
    }
}

impl Default for ComplexityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
