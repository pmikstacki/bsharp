use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::report::AnalysisReport;
use parser::facade::Parser;

fn analyze(source: &str) -> AnalysisReport {
    let (cu, spans) = Parser::new().parse_with_spans(source).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("test.cs", source), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    AnalysisReport::from_session(&session)
}

#[test]
fn ctor_no_async_emits_bse01001() {
    let src = r#"
public class C {
  public async C() { }
}
"#;
    let report = analyze(src);
    assert!(
        report
            .diagnostics
            .diagnostics
            .iter()
            .any(|d| d.code.as_str() == "BSE01001"),
        "expected ctor no async error BSE01001"
    );
}

#[test]
fn ctor_name_mismatch_emits_bse01005() {
    let src = r#"
public class C {
  public D() { }
}
"#;
    let report = analyze(src);
    assert!(
        report
            .diagnostics
            .diagnostics
            .iter()
            .any(|d| d.code.as_str() == "BSE01005"),
        "expected ctor name mismatch error BSE01005"
    );
}

#[test]
fn abstract_method_with_body_emits_bse02001() {
    let src = r#"
public abstract class C {
  public abstract void M() { }
}
"#;
    let report = analyze(src);
    assert!(
        report
            .diagnostics
            .diagnostics
            .iter()
            .any(|d| d.code.as_str() == "BSE02001"),
        "expected abstract method with body error BSE02001"
    );
}

#[test]
fn static_override_method_emits_bse02006() {
    let src = r#"
public class B { public virtual void M() {} }
public class C : B { public static override void M() {} }
"#;
    let report = analyze(src);
    assert!(
        report
            .diagnostics
            .diagnostics
            .iter()
            .any(|d| d.code.as_str() == "BSE02006"),
        "expected static override method error BSE02006"
    );
}

#[test]
fn async_method_non_task_return_emits_bse02009() {
    let src = r#"
using System.Threading.Tasks;
public class C {
  public async int M() { return 1; }
}
"#;
    let report = analyze(src);
    assert!(
        report
            .diagnostics
            .diagnostics
            .iter()
            .any(|d| d.code.as_str() == "BSE02009"),
        "expected async returns non-Task error BSE02009"
    );
}
