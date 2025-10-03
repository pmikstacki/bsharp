use nom::branch::alt;

use crate::syntax::comment_parser::parse_whitespace_or_comments;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::context;

use crate::parser::expressions::declarations::variable_declaration_parser::parse_local_variable_declaration_statement;
use crate::parser::expressions::statements::block_statement_parser::parse_block_statement;
use crate::parser::expressions::statements::break_statement_parser::parse_break_statement;
use crate::parser::expressions::statements::continue_statement_parser::parse_continue_statement;
use crate::parser::expressions::statements::do_while_statement_parser::parse_do_while_statement;
use crate::parser::expressions::statements::empty_statement_parser::parse_empty_statement;
use crate::parser::expressions::statements::expression_statement_parser::parse_expression_statement;
use crate::parser::expressions::statements::for_statement_parser::parse_for_statement;
use crate::parser::expressions::statements::foreach_statement_parser::parse_foreach_statement;
use crate::parser::expressions::statements::if_statement_parser::parse_if_statement;
use crate::parser::expressions::statements::local_function_statement_parser::parse_local_function_statement;
use crate::parser::expressions::statements::return_statement_parser::parse_return_statement;
use crate::parser::expressions::statements::switch_statement_parser::parse_switch_statement;
use crate::parser::expressions::statements::throw_statement_parser::parse_throw_statement;
use crate::parser::expressions::statements::try_catch_finally_parser::parse_try_statement;
use crate::parser::expressions::statements::using_statement_parser::parse_using_statement;
use crate::parser::expressions::statements::while_statement_parser::parse_while_statement;

// New statement parser
use crate::parser::expressions::statements::checked_statement_parser::parse_checked_unchecked_statement;
use crate::parser::expressions::statements::fixed_statement_parser::parse_fixed_statement;
use crate::parser::expressions::statements::goto_case_statement_parser::parse_goto_case_statement;
use crate::parser::expressions::statements::goto_statement_parser::parse_goto_statement;
use crate::parser::expressions::statements::label_statement_parser::parse_label_statement;
use crate::parser::expressions::statements::lock_statement_parser::parse_lock_statement;
use crate::parser::expressions::statements::unsafe_statement_parser::parse_unsafe_statement;
use crate::parser::expressions::statements::yield_statement_parser::parse_yield_statement;

/// DEBUG: Test individual parser to see which one succeeds incorrectly
pub fn debug_test_individual_parsers(input: &str) -> String {
    let mut results = Vec::new();

    // Test each syntax individually
    let parsers = [
        (
            "block_statement",
            parse_block_statement as fn(&str) -> BResult<&str, Statement>,
        ),
        ("if_statement", parse_if_statement),
        ("for_statement", parse_for_statement),
        ("while_statement", parse_while_statement),
        ("do_while_statement", parse_do_while_statement),
        ("foreach_statement", parse_foreach_statement),
        ("switch_statement", parse_switch_statement),
        ("try_statement", parse_try_statement),
        ("using_statement", parse_using_statement),
        ("local_function_statement", parse_local_function_statement),
        (
            "variable_declaration_statement",
            parse_local_variable_declaration_statement,
        ),
        ("return_statement", parse_return_statement),
        ("throw_statement", parse_throw_statement),
        ("break_statement", parse_break_statement),
        ("continue_statement", parse_continue_statement),
        ("goto_statement", parse_goto_statement),
        ("goto_case_statement", parse_goto_case_statement),
        ("label_statement", parse_label_statement),
        ("yield_statement", parse_yield_statement),
        (
            "checked_unchecked_statement",
            parse_checked_unchecked_statement,
        ),
        ("lock_statement", parse_lock_statement),
        ("unsafe_statement", parse_unsafe_statement),
        ("fixed_statement", parse_fixed_statement),
        ("empty_statement", parse_empty_statement),
        ("expression_statement", parse_expression_statement),
    ];

    for (name, parser) in parsers.iter() {
        match parser(input) {
            Ok((remaining, _)) => {
                results.push(format!("✅ {} SUCCESS - remaining: {:?}", name, remaining));
            }
            Err(e) => {
                results.push(format!("❌ {} FAILED - {:?}", name, e));
            }
        }
    }

    results.join("\n")
}

/// Build Group 1 parser with an option to allow or disallow block statements
fn build_group1_parser(allow_block: bool) -> impl Fn(&str) -> BResult<&str, Statement> {
    move |input: &str| {
        if allow_block {
            alt((
                context(
                    "block statement (expected '{' followed by statements and '}')",
                    parse_block_statement,
                ),
                context(
                    "if statement (expected 'if' followed by condition in parentheses)",
                    parse_if_statement,
                ),
                context(
                    "for statement (expected 'for' followed by initialization, condition, and increment)",
                    parse_for_statement,
                ),
                context(
                    "while statement (expected 'while' followed by condition in parentheses)",
                    parse_while_statement,
                ),
                context(
                    "do-while statement (expected 'do' followed by statement and 'while' condition)",
                    parse_do_while_statement,
                ),
                context(
                    "foreach statement (expected 'foreach' with type and variable declaration)",
                    parse_foreach_statement,
                ),
                context(
                    "switch statement (expected 'switch' followed by expression in parentheses)",
                    parse_switch_statement,
                ),
                context(
                    "try statement (expected 'try' followed by block and catch/finally clauses)",
                    parse_try_statement,
                ),
            ))(input)
        } else {
            alt((
                context(
                    "if statement inside block (expected 'if' followed by condition)",
                    parse_if_statement,
                ),
                context(
                    "for statement inside block (expected 'for' loop declaration)",
                    parse_for_statement,
                ),
                context(
                    "while statement inside block (expected 'while' followed by condition)",
                    parse_while_statement,
                ),
                context(
                    "do-while statement inside block (expected 'do' followed by statement)",
                    parse_do_while_statement,
                ),
                context(
                    "foreach statement inside block (expected 'foreach' variable declaration)",
                    parse_foreach_statement,
                ),
                context(
                    "switch statement inside block (expected 'switch' expression)",
                    parse_switch_statement,
                ),
                context(
                    "try statement inside block (expected 'try' with catch/finally)",
                    parse_try_statement,
                ),
            ))(input)
        }
    }
}

/// Group 1 with block statements allowed
fn parse_group1_with_block(input: &str) -> BResult<&str, Statement> {
    build_group1_parser(true)(input)
}

/// Group 1 without block statements (for use inside blocks to prevent recursion)
fn parse_group1_without_block(input: &str) -> BResult<&str, Statement> {
    build_group1_parser(false)(input)
}

/// Group 2: Special statements that need early parsing
fn parse_group2_special(input: &str) -> BResult<&str, Statement> {
    alt((
        context(
            "checked/unchecked statement (expected 'checked' or 'unchecked' followed by block)",
            parse_checked_unchecked_statement,
        ),
        context(
            "lock statement (expected 'lock' followed by expression in parentheses and statement)",
            parse_lock_statement,
        ),
        context(
            "unsafe statement (expected 'unsafe' followed by block)",
            parse_unsafe_statement,
        ),
        context(
            "fixed statement (expected 'fixed' followed by variable declarations and statement)",
            parse_fixed_statement,
        ),
        context(
            "using statement (expected 'using' followed by resource declaration or expression)",
            parse_using_statement,
        ),
        context(
            "local function statement (expected function signature with optional body)",
            parse_local_function_statement,
        ),
        context(
            "variable declaration (expected type followed by variable name and optional initializer)",
            parse_local_variable_declaration_statement,
        ),
    ))(input)
}

/// Group 3: Jump statements
fn parse_group3_jump(input: &str) -> BResult<&str, Statement> {
    alt((
        context(
            "return statement (expected 'return' followed by optional expression and semicolon)",
            parse_return_statement,
        ),
        context(
            "throw statement (expected 'throw' followed by expression and semicolon)",
            parse_throw_statement,
        ),
        context(
            "break statement (expected 'break' followed by semicolon)",
            parse_break_statement,
        ),
        context(
            "continue statement (expected 'continue' followed by semicolon)",
            parse_continue_statement,
        ),
        context(
            "goto statement (expected 'goto' followed by label and semicolon)",
            parse_goto_statement,
        ),
        context(
            "goto case statement (expected 'goto case' or 'goto default' followed by semicolon)",
            parse_goto_case_statement,
        ),
        context(
            "yield statement (expected 'yield return' or 'yield break' followed by semicolon)",
            parse_yield_statement,
        ),
    ))(input)
}

/// Group 4: Label, empty, and expression statements
fn parse_group4_misc(input: &str) -> BResult<&str, Statement> {
    alt((
        context(
            "label statement (expected identifier followed by colon)",
            parse_label_statement,
        ),
        context(
            "empty statement (expected semicolon)",
            parse_empty_statement,
        ),
        context(
            "expression statement (expected valid C# expression followed by semicolon)",
            parse_expression_statement,
        ),
    ))(input)
}

/// Main statement syntax - Enhanced VerboseError with specific diagnostics
/// Following Microsoft's Roslyn approach but using Nom's VerboseError for detailed error reporting
pub fn parse_statement(input: &str) -> BResult<&str, Statement> {
    alt((
        parse_group1_with_block,
        parse_group2_special,
        parse_group3_jump,
        parse_group4_misc,
    ))(input)
}

/// Statement syntax for use inside blocks - EXCLUDES block statements to prevent recursion
/// Enhanced with VerboseError diagnostics explaining the recursive exclusion
pub fn parse_statement_for_block(input: &str) -> BResult<&str, Statement> {
    alt((
        parse_group1_without_block,
        parse_group2_special,
        parse_group3_jump,
        parse_group4_misc,
    ))(input)
}

/// Parse a statement for use inside blocks, consuming any leading whitespace or comments.
/// This version excludes block statements to prevent recursion.
pub fn parse_statement_for_block_ws(input: &str) -> BResult<&str, Statement> {
    let (input, _) = parse_whitespace_or_comments(input)?;
    parse_statement_for_block(input)
}

/// Parse a statement, consuming any leading whitespace or comments.
pub fn parse_statement_ws(input: &str) -> BResult<&str, Statement> {
    let (input, _) = parse_whitespace_or_comments(input)?;
    parse_statement(input)
}

/*
/// Test function to demonstrate VerboseError with enhanced detailed diagnostics
/// Shows exactly what went wrong, where, and what was expected instead
/// Note: Commented out due to error type migration from VerboseError to ErrorTree
pub fn test_verbose_error_demo(input: &str) -> String {
    match parse_statement(input) {
        Ok((remaining, _)) => {
            format!("✅ Parsed successfully! Remaining: {:?}", remaining)
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            let detailed_error = format_parse_error(input, e);
            format!("❌ Parse failed with enhanced VerboseError diagnostics:\n{}\n\n🔍 This error shows:\n• Exact location in the input where parsing failed\n• Context stack showing what the syntax was trying to match\n• Specific expectations vs. what was actually found\n• Breadcrumb trail of parsing attempts for easier debugging", detailed_error)
        }
        Err(nom::Err::Incomplete(_)) => {
            "⏳ Incomplete input: more data needed (this shouldn't happen with complete parser)".to_string()
        }
    }
}
*/
