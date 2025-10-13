use analysis::diagnostics::diagnostic::Diagnostic;
use analysis::diagnostics::diagnostic_code::DiagnosticCode;
use analysis::diagnostics::diagnostic_collection::DiagnosticCollection;

#[test]
fn diagnostic_collection_add_and_extend_counts() {
    let mut c1 = DiagnosticCollection::new();
    c1.add(Diagnostic::with_default_message(DiagnosticCode::BSW02002));
    c1.add_warning(DiagnosticCode::BSW02003, "camelCase expected".into());

    assert!(c1.has_warnings());
    assert!(!c1.has_errors());
    assert_eq!(c1.warning_count(), 2); // both BSW codes are warnings

    let mut c2 = DiagnosticCollection::new();
    c2.add(Diagnostic::with_default_message(DiagnosticCode::BSW02004));
    c1.extend(c2);
    assert_eq!(c1.warning_count(), 3);
}
