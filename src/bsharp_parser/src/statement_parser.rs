use crate::parser::expressions::declarations::variable_declaration_parser::parse_local_variable_declaration_statement;
use crate::trivia::comment_parser::parse_whitespace_or_comments;
use crate::errors::BResult;
use crate::span::{Spanned, ByteRange, LineOffset, TextRange};
use syntax::statements::statement::Statement;

use nom::Parser;
use nom_supreme::ParserExt;

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

/// Build Group 1 parser with an option to allow or disallow block statements
fn build_group1_parser<'a>(allow_block: bool) -> impl Fn(Span<'a>) -> BResult<'a, Statement> {
    move |input: Span<'a>| {
        if allow_block {
            if let Ok(r) = (|i| parse_block_statement(i))
                .context("block statement")
                .parse(input)
            {
                return Ok(r);
            }
            if let Ok(r) = (|i| parse_if_statement(i))
                .context("if statement")
                .parse(input)
            {
                return Ok(r);
            }
            if let Ok(r) = (|i| parse_for_statement(i))
                .context("for statement")
                .parse(input)
            {
                return Ok(r);
            }
            if let Ok(r) = (|i| parse_while_statement(i))
                .context("while statement")
                .parse(input)
            {
                return Ok(r);
            }
            if let Ok(r) = (|i| parse_do_while_statement(i))
                .context("do-while statement")
                .parse(input)
            {
                return Ok(r);
            }
            if let Ok(r) = (|i| parse_foreach_statement(i))
                .context("foreach statement")
                .parse(input)
            {
                return Ok(r);
            }
            if let Ok(r) = (|i| parse_switch_statement(i))
                .context("switch statement")
                .parse(input)
            {
                return Ok(r);
            }
            (|i| parse_try_statement(i))
                .context("try statement")
                .parse(input)
        } else {
            if let Ok(r) = (|i| parse_if_statement(i))
                .context("if statement inside block")
                .parse(input)
            {
                return Ok(r);
            }
            if let Ok(r) = (|i| parse_for_statement(i))
                .context("for statement inside block")
                .parse(input)
            {
                return Ok(r);
            }
            if let Ok(r) = (|i| parse_while_statement(i))
                .context("while statement inside block")
                .parse(input)
            {
                return Ok(r);
            }
            if let Ok(r) = (|i| parse_do_while_statement(i))
                .context("do-while statement inside block")
                .parse(input)
            {
                return Ok(r);
            }
            if let Ok(r) = (|i| parse_foreach_statement(i))
                .context("foreach statement inside block")
                .parse(input)
            {
                return Ok(r);
            }
            if let Ok(r) = (|i| parse_switch_statement(i))
                .context("switch statement inside block")
                .parse(input)
            {
                return Ok(r);
            }
            (|i| parse_try_statement(i))
                .context("try statement inside block")
                .parse(input)
        }
    }
}

/// Group 1 with block statements allowed
fn parse_group1_with_block(input: Span) -> BResult<Statement> {
    build_group1_parser(true)(input)
}

/// Group 1 without block statements (for use inside blocks to prevent recursion)
fn parse_group1_without_block(input: Span) -> BResult<Statement> {
    build_group1_parser(false)(input)
}

/// Group 2: Special statements that need early parsing
fn parse_group2_special(input: Span) -> BResult<Statement> {
    if let Ok(r) = (|i| parse_checked_unchecked_statement(i))
        .context("checked/unchecked statement")
        .parse(input)
    {
        return Ok(r);
    }
    if let Ok(r) = (|i| parse_lock_statement(i))
        .context("lock statement")
        .parse(input)
    {
        return Ok(r);
    }
    if let Ok(r) = (|i| parse_unsafe_statement(i))
        .context("unsafe statement")
        .parse(input)
    {
        return Ok(r);
    }
    if let Ok(r) = (|i| parse_fixed_statement(i))
        .context("fixed statement")
        .parse(input)
    {
        return Ok(r);
    }
    if let Ok(r) = (|i| parse_using_statement(i))
        .context("using statement")
        .parse(input)
    {
        return Ok(r);
    }
    if let Ok(r) = (|i| parse_local_function_statement(i))
        .context("local function statement")
        .parse(input)
    {
        return Ok(r);
    }
    (|i| parse_local_variable_declaration_statement(i))
        .context("variable declaration (expected type followed by variable name and optional initializer)")
        .parse(input)
}

/// Group 3: Jump statements
fn parse_group3_jump(input: Span) -> BResult<Statement> {
    if let Ok(r) = (|i| parse_return_statement(i))
        .context("return statement")
        .parse(input)
    {
        return Ok(r);
    }
    if let Ok(r) = (|i| parse_throw_statement(i))
        .context("throw statement")
        .parse(input)
    {
        return Ok(r);
    }
    if let Ok(r) = (|i| parse_break_statement(i))
        .context("break statement")
        .parse(input)
    {
        return Ok(r);
    }
    if let Ok(r) = (|i| parse_continue_statement(i))
        .context("continue statement")
        .parse(input)
    {
        return Ok(r);
    }
    if let Ok(r) = (|i| parse_goto_statement(i))
        .context("goto statement")
        .parse(input)
    {
        return Ok(r);
    }
    if let Ok(r) = (|i| parse_goto_case_statement(i))
        .context("goto case statement")
        .parse(input)
    {
        return Ok(r);
    }
    (|i| parse_yield_statement(i))
        .context("yield statement")
        .parse(input)
}

/// Group 4: Label, empty, and expression statements
fn parse_group4_misc(input: Span) -> BResult<Statement> {
    if let Ok(r) = (|i| parse_label_statement(i))
        .context("label statement")
        .parse(input)
    {
        return Ok(r);
    }
    if let Ok(r) = (|i| parse_empty_statement(i))
        .context("empty statement")
        .parse(input)
    {
        return Ok(r);
    }
    (|i| parse_expression_statement(i))
        .context("expression statement")
        .parse(input)
}

/// Main statement syntax - Enhanced VerboseError with specific diagnostics
/// Following Microsoft's Roslyn approach but using Nom's VerboseError for detailed error reporting
pub fn parse_statement(input: Span) -> BResult<Statement> {
    if let Ok(r) = parse_group1_with_block(input) {
        return Ok(r);
    }
    if let Ok(r) = parse_group2_special(input) {
        return Ok(r);
    }
    if let Ok(r) = parse_group3_jump(input) {
        return Ok(r);
    }
    parse_group4_misc(input)
}

/// Statement syntax for use inside blocks - EXCLUDES block statements to prevent recursion
/// Enhanced with VerboseError diagnostics explaining the recursive exclusion
pub fn parse_statement_for_block(input: Span) -> BResult<Statement> {
    if let Ok(r) = parse_group1_without_block(input) {
        return Ok(r);
    }
    if let Ok(r) = parse_group2_special(input) {
        return Ok(r);
    }
    if let Ok(r) = parse_group3_jump(input) {
        return Ok(r);
    }
    parse_group4_misc(input)
}

/// Parse a statement for use inside blocks, consuming any leading whitespace or comments.
/// This version excludes block statements to prevent recursion.
pub fn parse_statement_for_block_ws(input: Span) -> BResult<Statement> {
    let (input, _) = parse_whitespace_or_comments(input)?;
    parse_statement_for_block(input)
}

/// Parse a statement, consuming any leading whitespace or comments.
pub fn parse_statement_ws(input: Span) -> BResult<Statement> {
    let (input, _) = parse_whitespace_or_comments(input)?;
    parse_statement(input)
}

pub fn parse_statement_spanned(input: Span) -> BResult<Spanned<Statement>> {
    let start_abs = input.location_offset();
    let start_lo = LineOffset { line: input.location_line(), offset: input.get_utf8_column().saturating_sub(1) };
    let (rest, node) = parse_statement(input)?;
    let end_abs = rest.location_offset();
    let end_lo = LineOffset { line: rest.location_line(), offset: rest.get_utf8_column().saturating_sub(1) };
    let abs = ByteRange { start: start_abs, end: end_abs };
    let rel = TextRange { start: start_lo, end: end_lo };
    Ok((rest, Spanned { node, abs, rel }))
}

pub fn parse_statement_for_block_spanned(input: Span) -> BResult<Spanned<Statement>> {
    let start_abs = input.location_offset();
    let start_lo = LineOffset { line: input.location_line(), offset: input.get_utf8_column().saturating_sub(1) };
    let (rest, node) = parse_statement_for_block(input)?;
    let end_abs = rest.location_offset();
    let end_lo = LineOffset { line: rest.location_line(), offset: rest.get_utf8_column().saturating_sub(1) };
    let abs = ByteRange { start: start_abs, end: end_abs };
    let rel = TextRange { start: start_lo, end: end_lo };
    Ok((rest, Spanned { node, abs, rel }))
}

pub fn parse_statement_ws_spanned(input: Span) -> BResult<Spanned<Statement>> {
    let (input_core, _) = parse_whitespace_or_comments(input)?;
    let start_abs = input_core.location_offset();
    let start_lo = LineOffset { line: input_core.location_line(), offset: input_core.get_utf8_column().saturating_sub(1) };
    let (rest_after_core, node) = parse_statement(input_core)?;
    let end_abs = rest_after_core.location_offset();
    let end_lo = LineOffset { line: rest_after_core.location_line(), offset: rest_after_core.get_utf8_column().saturating_sub(1) };
    let abs = ByteRange { start: start_abs, end: end_abs };
    let rel = TextRange { start: start_lo, end: end_lo };
    Ok((rest_after_core, Spanned { node, abs, rel }))
}

use syntax::span::Span;

