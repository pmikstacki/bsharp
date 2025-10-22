use crate::emitters::emit_trait::{EmitCtx, EmitError};
use crate::statements::statement::Statement;
use std::fmt::Write;

/// Minimal formatting policy helpers.
/// These are intentionally simple; FmtWriter later collapses duplicate blank lines.
pub(crate) fn between_header_and_body_of_file<W: Write>(
    cx: &mut EmitCtx,
    w: &mut W,
) -> Result<(), EmitError> {
    // Ensure exactly one blank line separation
    cx.nl(w)?;
    cx.nl(w)
}

pub(crate) fn after_file_scoped_namespace_header<W: Write>(
    cx: &mut EmitCtx,
    w: &mut W,
) -> Result<(), EmitError> {
    // Ensure a blank line after file-scoped namespace header
    cx.nl(w)?;
    cx.nl(w)
}

pub(crate) fn between_using_blocks_and_declarations<W: Write>(
    cx: &mut EmitCtx,
    w: &mut W,
) -> Result<(), EmitError> {
    // Ensure exactly one blank line separation
    cx.nl(w)?;
    cx.nl(w)
}

pub(crate) fn between_top_level_declarations<W: Write>(
    cx: &mut EmitCtx,
    w: &mut W,
) -> Result<(), EmitError> {
    // Insert one separating newline (together with prior trailing newline -> one blank line)
    cx.nl(w)
}

pub(crate) fn between_members<W: Write>(cx: &mut EmitCtx, w: &mut W) -> Result<(), EmitError> {
    cx.nl(w)
}

// Insert an extra separator inside block bodies between certain kinds of statements.
// CSharpier inserts a blank line when a control-flow block (if/for/while/do/switch/inner block)
// is followed by a declaration statement in the same block scope.
pub(crate) fn between_block_items<W: Write>(
    cx: &mut EmitCtx,
    w: &mut W,
    prev: &Statement,
    next: &Statement,
) -> Result<(), EmitError> {
    let prev_is_block_like = matches!(
        prev,
        Statement::If(_)
            | Statement::For(_)
            | Statement::While(_)
            | Statement::DoWhile(_)
            | Statement::Switch(_)
            | Statement::Block(_)
    );
    if prev_is_block_like {
        if let Statement::Declaration(_) = next {
            // One additional newline -> results in a visible blank line given the prior nl
            return cx.nl(w);
        }
    }
    Ok(())
}
