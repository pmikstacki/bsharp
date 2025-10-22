use crate::AnalysisContext;
use crate::diagnostics::diagnostic::Diagnostic;

/// Render a compact, caret-annotated diagnostic body used by CLI pretty printing.
/// Example:
/// at 3:15: expected ';'
/// let x = 1
///               ^
pub fn render_body(ctx: &AnalysisContext, d: &Diagnostic) -> String {
    let mut out = String::new();
    let (line, col, len) = if let Some(loc) = d.location.as_ref() {
        (loc.line, loc.column, loc.length)
    } else {
        (1usize, 1usize, 0usize)
    };
    let line_text = ctx.line_text(line).to_string();
    out.push_str(&format!("at {}:{}: {}\n", line, col, d.message));
    out.push_str(&line_text);
    out.push('\n');
    let caret_count = if len == 0 { 1 } else { len };
    out.push_str(&" ".repeat(col.saturating_sub(1)));
    out.push_str(&"^".repeat(caret_count));
    out
}
