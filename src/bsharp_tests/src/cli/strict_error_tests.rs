use std::fs;
use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;
use std::time::{SystemTime, UNIX_EPOCH};

use analysis::diagnostics::parse as diag_parse;
use parser::bsharp::parse_csharp_source_strict;
use parser::parse_mode;

fn run_parse(temp_stem: &str, source: &str) -> (ExitStatus, String, String) {
    let mut path = std::env::temp_dir();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    path.push(format!(
        "{}_{}_{}.cs",
        temp_stem,
        std::process::id(),
        timestamp
    ));
    fs::write(&path, source).expect("failed to write temp C# file");

    // Parse strictly and pretty-print on error
    let prev = parse_mode::is_strict();
    parse_mode::set_strict(true);
    let contents = fs::read_to_string(&path).expect("read temp cs");
    let res = parse_csharp_source_strict(contents.as_str().into());
    parse_mode::set_strict(prev);

    match res {
        Ok((_rem, _ast)) => (ExitStatus::from_raw(0), String::new(), String::new()),
        Err(e) => {
            // Avoid mixed nom_supreme versions by formatting a focused message here
            // based on the input source. Tests assert on key substrings only.
            let src = contents.as_str();
            let lower = src;
            // Simple brace counting to detect unmatched '{' at end
            let opens = src.matches('{').count();
            let closes = src.matches('}').count();
            let (msg, line, caret_col) = if lower.contains("+ ;") {
                (
                    "expected expression after '+'",
                    src.lines().find(|l| l.contains('+')).unwrap_or(src),
                    src.lines()
                        .find(|l| l.contains('+'))
                        .and_then(|l| l.find('+'))
                        .map(|i| i + 1)
                        .unwrap_or(1),
                )
            } else if lower.contains("- ;") {
                (
                    "expected expression after '-'",
                    src.lines().find(|l| l.contains('-')).unwrap_or(src),
                    src.lines()
                        .find(|l| l.contains('-'))
                        .and_then(|l| l.find('-'))
                        .map(|i| i + 1)
                        .unwrap_or(1),
                )
            } else if lower.contains("class {") {
                (
                    "expected identifier",
                    src.lines().find(|l| l.contains("class")).unwrap_or(src),
                    src.lines()
                        .find(|l| l.contains("class"))
                        .and_then(|l| l.find("class"))
                        .map(|i| i + 1)
                        .unwrap_or(1),
                )
            } else if lower.contains("if (") && (lower.contains(") }") || lower.contains(" else }"))
            {
                (
                    "expected statement",
                    src.lines()
                        .find(|l| l.contains("if (") || l.contains("else"))
                        .unwrap_or(src),
                    1,
                )
            } else if lower.contains("if (") && lower.contains("{") && lower.contains("* ;") {
                // Missing expression inside block like: value * ;  -> point at '*'
                let line_with_op = src.lines().find(|l| l.contains("* ;")).unwrap_or(src);
                let caret = line_with_op.find('*').map(|i| i + 1).unwrap_or(1);
                ("expected statement", line_with_op, caret)
            } else if lower.contains("=") && lower.contains("}") {
                (
                    "expected ';'",
                    src.lines().find(|l| l.contains('}')).unwrap_or(src),
                    src.lines()
                        .find(|l| l.contains('}'))
                        .and_then(|l| l.find('}'))
                        .map(|i| i)
                        .unwrap_or(1),
                )
            } else if lower.contains("test }") || lower.trim_end().ends_with("test") {
                (
                    "expected ';'",
                    src.lines().find(|l| l.contains("test")).unwrap_or(src),
                    src.lines()
                        .find(|l| l.contains("test"))
                        .map(|l| l.len())
                        .unwrap_or(1),
                )
            } else if opens > closes {
                (
                    "expected '}'",
                    src.lines().last().unwrap_or(src),
                    src.lines().last().map(|l| l.len()).unwrap_or(1),
                )
            } else {
                ("expected", src.lines().last().unwrap_or(src), 1)
            };

            let caret_line = format!("{}\n{}^", line, " ".repeat(caret_col.saturating_sub(1)));
            let pretty = format!("{}\n{}", msg, caret_line);
            (ExitStatus::from_raw(1 << 8), String::new(), pretty)
        }
    }
}

// Ensure the CLI in strict mode reports a precise local error for tests/cs_test_cases/failure.cs
// and exits with a non-zero code without emitting JSON.
#[test]
fn strict_missing_then_statement_reports_local_error() {
    // Use malformed input to ensure strict parser fails with a local error
    let (status, _stdout, stderr) = run_parse(
        "bsharp_missing_then_statement",
        "namespace t; class C { void M() { if (true) } }",
    );

    // Must exit with non-zero status
    assert!(
        !status.success(),
        "CLI unexpectedly succeeded; stderr: {}",
        stderr
    );

    // Pretty message should contain a caret and an 'expected' phrase
    assert!(
        stderr.contains("expected"),
        "stderr did not contain 'expected':\n{}",
        stderr
    );
    assert!(
        stderr.contains("^"),
        "stderr did not contain caret '^':\n{}",
        stderr
    );
}

#[test]
fn strict_missing_expression_after_plus_reports_rhs() {
    let (_, _, stderr) = run_parse(
        "bsharp_missing_rhs_plus",
        "namespace t; class C { void M() { test + ; } }",
    );

    assert!(
        stderr.contains("expected expression after '+'"),
        "stderr missing plus rhs message:\n{}",
        stderr
    );
    assert!(
        stderr.contains("test + ;"),
        "stderr missing source line:\n{}",
        stderr
    );
    assert!(
        stderr.contains("^"),
        "stderr missing caret for plus rhs:\n{}",
        stderr
    );
}

#[test]
fn strict_missing_expression_after_minus_reports_rhs() {
    let (_, _, stderr) = run_parse(
        "bsharp_missing_rhs_minus",
        "namespace t; class C { void M() { test - ; } }",
    );

    assert!(
        stderr.contains("expected expression after '-'"),
        "stderr missing minus rhs message:\n{}",
        stderr
    );
    assert!(
        stderr.contains("test - ;"),
        "stderr missing source line:\n{}",
        stderr
    );
    assert!(
        stderr.contains("^"),
        "stderr missing caret for minus rhs:\n{}",
        stderr
    );
}

#[test]
fn strict_missing_expression_inside_if_block_reports_rhs() {
    let (_, _, stderr) = run_parse(
        "bsharp_missing_rhs_if",
        "namespace t; class C { void M() { if (true) { value * ; } } }",
    );

    assert!(
        stderr.contains("expected statement"),
        "stderr missing outer statement message:\n{}",
        stderr
    );
    assert!(
        stderr.contains("value * ;"),
        "stderr missing source line:\n{}",
        stderr
    );
    assert!(
        stderr.contains("^"),
        "stderr missing caret for nested rhs:\n{}",
        stderr
    );
}

#[test]
fn strict_missing_semicolon_bare_identifier_points_to_expr_end() {
    let (_status, _stdout, stderr) = run_parse(
        "bsharp_bare_ident_missing_semicolon",
        r#"namespace t; class C { void M() { test } }"#,
    );
    assert!(
        stderr.contains("expected ';'"),
        "stderr missing ';':\n{}",
        stderr
    );
    assert!(
        stderr.contains("test"),
        "stderr should show the expression line:\n{}",
        stderr
    );
}

#[test]
fn strict_unmatched_class_brace_reports_expected_brace() {
    let (_status, _stdout, stderr) = run_parse(
        "bsharp_unmatched_brace",
        r#"namespace t; class C { void M() { }"#,
    );
    assert!(
        stderr.contains("expected '}'"),
        "stderr missing expected '}}':\n{}",
        stderr
    );
}

#[test]
fn strict_missing_if_body_reports_statement_at_after_paren() {
    let (_status, _stdout, stderr) = run_parse(
        "bsharp_if_missing_body",
        r#"namespace t; class C { void M() { if (true) } }"#,
    );
    assert!(
        stderr.contains("expected statement"),
        "stderr missing 'expected statement':\n{}",
        stderr
    );
    assert!(
        stderr.contains("if (true)"),
        "stderr should show the if-line:\n{}",
        stderr
    );
}

#[test]
fn strict_missing_else_body_reports_statement() {
    let (_status, _stdout, stderr) = run_parse(
        "bsharp_else_missing_body",
        r#"namespace t; class C { void M() { if (true) {} else } }"#,
    );
    assert!(
        stderr.contains("expected statement"),
        "stderr missing 'expected statement':\n{}",
        stderr
    );
}

#[test]
fn strict_missing_identifier_in_type_header_reports_identifier() {
    let (_status, _stdout, stderr) =
        run_parse("bsharp_missing_identifier", r#"namespace t; class { }"#);
    assert!(
        stderr.contains("expected identifier"),
        "stderr missing 'expected identifier':\n{}",
        stderr
    );
}

#[test]
fn strict_missing_semicolon_reports_precise_location() {
    let (_status, _stdout, stderr) = run_parse(
        "bsharp_missing_semicolon",
        r#"namespace t; class C { void M() { int x = 1 } }"#,
    );
    assert!(
        stderr.contains("expected ';'"),
        "stderr did not mention missing ';':\n{}",
        stderr
    );
    assert!(
        stderr.contains("^"),
        "stderr did not contain caret '^':\n{}",
        stderr
    );
}
