use assert_cmd::prelude::*;
use std::process::Command;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

fn write_temp(contents: &str, name: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    let pid = std::process::id();
    let nanos = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    p.push(format!("bsharp_cli_test_{}_{}_{}.cs", name, pid, nanos));
    fs::write(&p, contents).expect("write temp");
    p
}

#[test]
fn pretty_miette_reports_expected_eof_for_trailing_tokens() {
    // Minimal program with trailing garbage to trigger remaining-input path (expected EOF)
    let src = r#"using System; class C {} trailing"#;
    let file = write_temp(src, "eof");

    let mut cmd = Command::cargo_bin("bsharp_cli").unwrap();
    let assert = cmd.args(["parse", "--input"]).arg(&file)
        .assert()
        .failure();

    let out = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(out.contains("parse error"), "stderr: {}", out);
    assert!(out.contains(&file.display().to_string()), "stderr: {}", out);
    assert!(out.contains("using System; class C {} trailing"), "stderr: {}", out);
}

#[test]
fn json_error_includes_location_and_message() {
    let src = r#"using System; class C {} trailing"#;
    let file = write_temp(src, "json");

    let mut cmd = Command::cargo_bin("bsharp_cli").unwrap();
    let assert = cmd.args(["parse", "--input"]).arg(&file)
        .arg("--errors-json")
        .assert()
        .failure();

    let out = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(out.contains("\"parse_error\""), "stdout: {}", out);
    assert!(out.contains("\"line\":"), "stdout: {}", out);
    assert!(out.contains("\"column\":"), "stdout: {}", out);
    assert!(out.contains("\"message\":"), "stdout: {}", out);
}
