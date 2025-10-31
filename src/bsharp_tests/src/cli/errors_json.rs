use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::NamedTempFile;

fn write_temp(source: &str) -> NamedTempFile {
    let mut file = tempfile::Builder::new()
        .prefix("bsharp_cli_test_")
        .suffix(".cs")
        .tempfile()
        .expect("tempfile");
    write!(file, "{}", source).expect("write");
    file
}

#[test]
fn errors_json_without_spans() {
    let input = write_temp("public clas Program { }"); // typo 'clas' to force error
    let mut cmd = Command::cargo_bin("bsharp_cli").unwrap();
    let assert = cmd
        .args(["parse", "--input", input.path().to_str().unwrap(), "--errors-json"]) // no --emit-spans
        .assert();

    // Non-zero exit and JSON payload with required keys present, and no spans key
    assert
        .failure()
        .stdout(
            predicate::str::contains("\"error\"")
                .and(predicate::str::contains("\"kind\""))
                .and(predicate::str::contains("\"file\""))
                .and(predicate::str::contains("\"line\""))
                .and(predicate::str::contains("\"column\""))
                .and(predicate::str::contains("\"message\""))
                .and(predicate::str::contains("\"spans\"").not()),
        );
}

#[test]
fn errors_json_with_spans() {
    let input = write_temp("public clas Program { }");
    let mut cmd = Command::cargo_bin("bsharp_cli").unwrap();
    let assert = cmd
        .args(["parse", "--input", input.path().to_str().unwrap(), "--errors-json", "--emit-spans"]) // spans enabled
        .assert();

    assert
        .failure()
        .stdout(
            predicate::str::contains("\"error\"")
                .and(predicate::str::contains("\"spans\"")),
        );
}

#[test]
fn strict_vs_lenient_failure_mode() {
    // An input likely to produce trailing input or general parse failure
    let input = write_temp("class C { void M() { if (true) { } else } }");

    // Strict
    let mut cmd_strict = Command::cargo_bin("bsharp_cli").unwrap();
    let assert_strict = cmd_strict
        .args(["parse", "--input", input.path().to_str().unwrap(), "--errors-json"]) // strict default
        .assert();
    assert_strict.failure();

    // Lenient
    let mut cmd_lenient = Command::cargo_bin("bsharp_cli").unwrap();
    let assert_lenient = cmd_lenient
        .args(["parse", "--input", input.path().to_str().unwrap(), "--errors-json", "--lenient"]) // lenient mode
        .assert();
    // Lenient may succeed; if it fails, ensure JSON payload is present
    if assert_lenient.get_output().status.success() {
        // ok
    } else {
        assert_lenient.stdout(predicate::str::contains("\"error\""));
    }
}
