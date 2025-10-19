use crate::emitters::emit_trait::{EmitCtx, EmitError};
use std::fmt::Write;

/// Minimal formatting policy helpers.
/// These are intentionally simple; FmtWriter later collapses duplicate blank lines.
pub(crate) fn between_header_and_body_of_file<W: Write>(cx: &mut EmitCtx, w: &mut W) -> Result<(), EmitError> {
    cx.nl(w)
}

pub(crate) fn after_file_scoped_namespace_header<W: Write>(cx: &mut EmitCtx, w: &mut W) -> Result<(), EmitError> {
    // Ensure a blank line after file-scoped namespace header
    cx.nl(w)?;
    cx.nl(w)
}

pub(crate) fn between_using_blocks_and_declarations<W: Write>(cx: &mut EmitCtx, w: &mut W) -> Result<(), EmitError> {
    cx.nl(w)
}

pub(crate) fn between_top_level_declarations<W: Write>(cx: &mut EmitCtx, w: &mut W) -> Result<(), EmitError> {
    cx.nl(w)
}

pub(crate) fn between_members<W: Write>(cx: &mut EmitCtx, w: &mut W) -> Result<(), EmitError> {
    cx.nl(w)
}
