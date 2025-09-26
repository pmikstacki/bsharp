use crate::syntax::ast::CompilationUnit;
use crate::syntax::nodes::declarations::MethodDeclaration;
use crate::syntax::nodes::statements::statement::Statement;
use serde::{Deserialize, Serialize};

/// Control flow analyzer for analyzing program flow
pub struct ControlFlowAnalyzer;

impl ControlFlowAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Analyze control flow for a compilation unit
    pub fn analyze_compilation_unit(&self, _unit: &CompilationUnit) -> ControlFlowGraph {
        // TODO: Implement control flow analysis
        ControlFlowGraph::new()
    }

    /// Calculate cyclomatic complexity for a method
    pub fn calculate_cyclomatic_complexity(&self, method: &MethodDeclaration) -> usize {
        if let Some(body) = &method.body {
            Self::calculate_statement_complexity(body, 1)
        } else {
            1 // Base complexity for methods without body
        }
    }

    /// Calculate complexity for a statement
    fn calculate_statement_complexity(stmt: &Statement, base_complexity: usize) -> usize {
        match stmt {
            Statement::If(if_stmt) => {
                let mut complexity = base_complexity + 1; // +1 for if
                complexity += Self::calculate_statement_complexity(&if_stmt.consequence, 0);
                if let Some(alt) = &if_stmt.alternative {
                    complexity += Self::calculate_statement_complexity(alt, 0);
                }
                complexity
            }
            Statement::For(for_stmt) => {
                base_complexity + 1 + Self::calculate_statement_complexity(&for_stmt.body, 0)
            }
            Statement::While(while_stmt) => {
                base_complexity + 1 + Self::calculate_statement_complexity(&while_stmt.body, 0)
            }
            Statement::DoWhile(do_while_stmt) => {
                base_complexity + 1 + Self::calculate_statement_complexity(&do_while_stmt.body, 0)
            }
            Statement::Switch(switch_stmt) => {
                let mut complexity = base_complexity + switch_stmt.sections.len(); // Each case adds complexity
                for section in &switch_stmt.sections {
                    for s in &section.statements {
                        complexity += Self::calculate_statement_complexity(s, 0);
                    }
                }
                complexity
            }
            Statement::Try(_) => {
                base_complexity + 1 // TODO: Add proper try/catch/finally analysis
            }
            Statement::Block(statements) => {
                let mut complexity = base_complexity;
                for stmt in statements {
                    complexity += Self::calculate_statement_complexity(stmt, 0);
                }
                complexity
            }
            _ => base_complexity,
        }
    }

    /// Detect code smells related to control flow
    pub fn detect_control_flow_smells(&self, method: &MethodDeclaration) -> Vec<ControlFlowSmell> {
        let mut smells = Vec::new();

        let complexity = self.calculate_cyclomatic_complexity(method);
        if complexity > 10 {
            smells.push(ControlFlowSmell::HighCyclomaticComplexity {
                method_name: method.name.name.clone(),
                complexity,
            });
        }

        if let Some(body) = &method.body {
            let nesting_depth = Self::calculate_max_nesting_depth(body, 0);
            if nesting_depth > 4 {
                smells.push(ControlFlowSmell::DeepNesting {
                    method_name: method.name.name.clone(),
                    max_depth: nesting_depth,
                });
            }
        }

        smells
    }

    /// Calculate maximum nesting depth
    fn calculate_max_nesting_depth(stmt: &Statement, current_depth: usize) -> usize {
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
}

impl Default for ControlFlowAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Control flow graph representation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ControlFlowGraph {
    pub nodes: Vec<ControlFlowNode>,
    pub edges: Vec<ControlFlowEdge>,
    pub entry_node: Option<usize>,
    pub exit_nodes: Vec<usize>,
}

impl ControlFlowGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: ControlFlowNode) -> usize {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    pub fn add_edge(&mut self, from: usize, to: usize, edge_type: ControlFlowEdgeType) {
        self.edges.push(ControlFlowEdge {
            from,
            to,
            edge_type,
        });
    }

    /// Calculate the number of decision points in the graph
    pub fn decision_points(&self) -> usize {
        self.nodes
            .iter()
            .filter(|node| matches!(node.node_type, ControlFlowNodeType::Decision))
            .count()
    }

    /// Calculate the number of exit points
    pub fn exit_points(&self) -> usize {
        self.exit_nodes.len()
    }
}

/// Node in a control flow graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlFlowNode {
    pub id: usize,
    pub node_type: ControlFlowNodeType,
    pub statement: Option<String>, // Source code representation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlFlowNodeType {
    Entry,
    Exit,
    Statement,
    Decision,
    Loop,
    Jump,
}

/// Edge in a control flow graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlFlowEdge {
    pub from: usize,
    pub to: usize,
    pub edge_type: ControlFlowEdgeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlFlowEdgeType {
    Sequential,
    True,
    False,
    Break,
    Continue,
    Exception,
}

/// Control flow related code smells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlFlowSmell {
    HighCyclomaticComplexity {
        method_name: String,
        complexity: usize,
    },
    DeepNesting {
        method_name: String,
        max_depth: usize,
    },
    TooManyExitPoints {
        method_name: String,
        exit_count: usize,
    },
    LongMethod {
        method_name: String,
        statement_count: usize,
    },
}

/// Control flow metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ControlFlowMetrics {
    pub cyclomatic_complexity: usize,
    pub max_nesting_depth: usize,
    pub decision_points: usize,
    pub exit_points: usize,
    pub loop_count: usize,
    pub conditional_count: usize,
}
