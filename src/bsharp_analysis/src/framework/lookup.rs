use crate::artifacts::symbols::{Symbol, SymbolIndex};
use crate::framework::AnalysisSession;
use crate::SourceLocation;

/// Return all symbols with the given local name using the SymbolIndex artifact.
pub fn find_symbols_by_name(session: &AnalysisSession, name: &str) -> Vec<Symbol> {
    let Some(index) = session.artifacts.get::<SymbolIndex>() else { return Vec::new(); };
    let Some(ids) = index.get_ids_by_name(name) else { return Vec::new(); };
    ids.iter()
        .filter_map(|id| index.get(*id).cloned())
        .collect()
}

/// Return all symbols with the given local name, together with best-effort source locations.
/// Location is computed from the session context and the stored span offsets when available.
pub fn find_symbols_with_locations(
    session: &AnalysisSession,
    name: &str,
) -> Vec<(Symbol, Option<SourceLocation>)> {
    let Some(index) = session.artifacts.get::<SymbolIndex>() else { return Vec::new(); };
    let Some(ids) = index.get_ids_by_name(name) else { return Vec::new(); };
    ids.iter()
        .filter_map(|id| index.get(*id))
        .map(|sym| {
            let loc = match (sym.span_start, sym.span_end) {
                (Some(s), Some(e)) if e >= s => Some(session.ctx.location_from_span(s, e - s)),
                _ => None,
            };
            (sym.clone(), loc)
        })
        .collect()
}
