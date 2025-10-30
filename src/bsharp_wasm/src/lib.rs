use wasm_bindgen::prelude::*;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_syntax::span::Span;
use bsharp_syntax::{FormatOptions, Formatter};
use bsharp_syntax::node::render::to_mermaid;
use bsharp_parser::facade::Parser;
use bsharp_analysis::context::AnalysisContext;
use bsharp_analysis::framework::pipeline::AnalyzerPipeline;
use bsharp_analysis::framework::session::AnalysisSession;
use bsharp_analysis::report::AnalysisReport;
use bsharp_parser::errors as perr;
use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};

#[wasm_bindgen]
pub fn format_code(input: &str) -> Result<String, JsValue> {
    let parsed = parse_csharp_source_strict(Span::new(input));
    let Ok((_rest, cu)) = parsed else {
        // Pretty-format parse error with caret and contexts (miette-like)
        let pretty = match parsed {
            Err(nom::Err::Error(t)) | Err(nom::Err::Failure(t)) => perr::format_error_tree(input, &t),
            Err(nom::Err::Incomplete(_)) => {
                let tree: ErrorTree<Span> = ErrorTree::Base {
                    location: Span::new(""),
                    kind: BaseErrorKind::Expected(Expectation::Eof),
                };
                perr::format_error_tree(input, &tree)
            }
            Ok(_) => unreachable!(),
        };
        return Err(JsValue::from_str(&pretty));
    };

    let opts = FormatOptions { ..Default::default() };
    let fmt = Formatter::new(opts);
    fmt
        .format_compilation_unit(&cu)
        .map_err(|_e| JsValue::from_str("Format failed"))
}

#[wasm_bindgen]
pub fn ast_to_mermaid(input: &str) -> Result<String, JsValue> {
    let parsed = parse_csharp_source_strict(Span::new(input));
    let Ok((_rest, cu)) = parsed else {
        let pretty = match parsed {
            Err(nom::Err::Error(t)) | Err(nom::Err::Failure(t)) => perr::format_error_tree(input, &t),
            Err(nom::Err::Incomplete(_)) => {
                let tree: ErrorTree<Span> = ErrorTree::Base {
                    location: Span::new(""),
                    kind: BaseErrorKind::Expected(Expectation::Eof),
                };
                perr::format_error_tree(input, &tree)
            }
            Ok(_) => unreachable!(),
        };
        return Err(JsValue::from_str(&pretty));
    };
    Ok(to_mermaid(&cu))
}

#[wasm_bindgen]
pub fn analyze_code(input: &str) -> Result<String, JsValue> {
    let parser = Parser::new();
    let (cu, spans) = parser
        .parse_with_spans(Span::new(input))
        .map_err(|e| JsValue::from_str(&format!("Parse error: {e}")))?;

    let ctx = AnalysisContext::new("inline.cs".to_string(), input.to_string());
    let mut session = AnalysisSession::new(ctx, spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let report = session
        .artifacts
        .get::<AnalysisReport>()
        .map(|a| (*a).clone())
        .unwrap_or_else(|| AnalysisReport::from_session(&session));
    serde_json::to_string_pretty(&report)
        .map_err(|_e| JsValue::from_str("Failed to serialize analysis report"))
}
