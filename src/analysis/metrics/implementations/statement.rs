use super::super::core::{AstAnalysis, AstAnalyze};
use crate::syntax::nodes::statements::statement::Statement;

impl AstAnalyze for Statement {
    fn analyze(&self) -> AstAnalysis {
        let mut analysis = AstAnalysis::default();

        match self {
            Statement::If(if_stmt) => {
                analysis.total_if_statements += 1;
                analysis.cyclomatic_complexity += 1; // Each branch adds complexity
                analysis = analysis.combine(if_stmt.consequence.analyze());
                if let Some(alt) = &if_stmt.alternative {
                    analysis = analysis.combine(alt.analyze());
                }
            }
            Statement::For(for_stmt) => {
                analysis.total_for_loops += 1;
                analysis.cyclomatic_complexity += 1;
                analysis = analysis.combine(for_stmt.body.analyze());
            }
            Statement::While(while_stmt) => {
                analysis.total_while_loops += 1;
                analysis.cyclomatic_complexity += 1;
                analysis = analysis.combine(while_stmt.body.analyze());
            }
            Statement::DoWhile(do_while_stmt) => {
                analysis.total_while_loops += 1;
                analysis.cyclomatic_complexity += 1;
                analysis = analysis.combine(do_while_stmt.body.analyze());
            }
            Statement::Switch(switch_stmt) => {
                analysis.total_switch_statements += 1;
                analysis.cyclomatic_complexity += switch_stmt.sections.len(); // Each case adds complexity
                for section in &switch_stmt.sections {
                    for stmt in &section.statements {
                        analysis = analysis.combine(stmt.analyze());
                    }
                }
            }
            Statement::Try(try_stmt) => {
                analysis.total_try_statements += 1;
                analysis.cyclomatic_complexity += 1;

                // Analyze the try block
                analysis = analysis.combine(try_stmt.try_block.analyze());

                // Analyze each catch block
                for catch_clause in &try_stmt.catches {
                    analysis.cyclomatic_complexity += 1; // Each catch adds complexity
                    analysis = analysis.combine(catch_clause.block.analyze());
                }

                // Analyze the finally block if present
                if let Some(finally_clause) = &try_stmt.finally_clause {
                    analysis = analysis.combine(finally_clause.block.analyze());
                }
            }
            Statement::Using(using_stmt) => {
                analysis.total_using_statements += 1;
                // Analyze the using statement body
                analysis = analysis.combine(using_stmt.body.analyze());
            }
            Statement::Block(statements) => {
                for stmt in statements {
                    analysis = analysis.combine(stmt.analyze());
                }
            }
            Statement::ForEach(foreach_stmt) => {
                analysis.total_for_loops += 1; // Count foreach as a type of for loop
                analysis.cyclomatic_complexity += 1;
                analysis = analysis.combine(foreach_stmt.body.analyze());
            }
            Statement::LocalFunction(local_func) => {
                analysis.total_methods += 1; // Local functions count as methods
                analysis = analysis.combine(local_func.body.analyze());
            }
            Statement::Declaration(_) => {
                // Variable declarations - no additional complexity but count as statements
            }
            Statement::Expression(_) => {
                // Expression statements - no additional complexity but count as statements
            }
            Statement::Return(_) => {
                // Return statements - no additional complexity but count as statements
            }
            Statement::Throw(_) => {
                // Throw statements - no additional complexity but count as statements
            }
            Statement::Break(_) => {
                // Break statements - no additional complexity but count as statements
            }
            Statement::Continue(_) => {
                // Continue statements - no additional complexity but count as statements
            }
            Statement::Goto(_) => {
                // Goto statements - no additional complexity but count as statements
            }
            Statement::GotoCase(_) => {
                // Goto case statements - no additional complexity but count as statements
            }
            Statement::Label(_) => {
                // Label statements - no additional complexity but count as statements
            }
            Statement::Empty => {
                // Empty statements - no additional complexity but count as statements
            }
            Statement::Deconstruction(_) => {
                // Deconstruction statements - no additional complexity but count as statements
            }
            Statement::Yield(_) => {
                // Yield statements - no additional complexity but count as statements
            }
            // TODO: Add analysis for Checked, Unchecked, Lock, Unsafe, Fixed when their structures are defined
            _ => {}
        }

        analysis
    }
}
