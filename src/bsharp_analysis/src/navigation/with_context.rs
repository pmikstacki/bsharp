use super::traits::{DeclarationInfo, DeclarationType, FindDeclarations};
use crate::syntax::ast::CompilationUnit;
use crate::AnalysisContext;
use bsharp_parser::SpanTable;

/// Find declarations by name and populate SourceLocation when spans are available.
///
/// Notes:
/// - Currently uses a simple key format (e.g., "class::Name") to look up spans
///   collected at parse time. This may collide across namespaces; Milestone B/C
///   will introduce fully-qualified keys to disambiguate.
pub fn find_by_name_with_context(
    cu: &CompilationUnit,
    name: &str,
    ctx: &AnalysisContext,
    spans: &SpanTable,
) -> Vec<DeclarationInfo> {
    let mut items = cu.find_by_name(name);
    for item in &mut items {
        if let Some((start, len)) = lookup_span(item, spans) {
            let loc = ctx.location_from_span(start, len);
            item.location = Some(loc);
        }
    }
    items
}

fn lookup_span(info: &DeclarationInfo, spans: &SpanTable) -> Option<(usize, usize)> {
    let key = match info.declaration_type {
        DeclarationType::Class => format!("class::{}", info.name),
        DeclarationType::Interface => format!("interface::{}", info.name),
        DeclarationType::Struct => format!("struct::{}", info.name),
        DeclarationType::Enum => format!("enum::{}", info.name),
        DeclarationType::Record => format!("record::{}", info.name),
        DeclarationType::Delegate => format!("delegate::{}", info.name),
        DeclarationType::Method
        | DeclarationType::Property
        | DeclarationType::Field
        | DeclarationType::Event
        | DeclarationType::Constructor => {
            // Top-level span table currently does not include members.
            return None;
        }
    };

    spans
        .get(&key)
        .map(|r| (r.start, r.end.saturating_sub(r.start)))
}
