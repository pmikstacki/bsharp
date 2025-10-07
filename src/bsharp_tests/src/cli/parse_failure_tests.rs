use std::fs;
use std::process::Command;

use crate::utils::create_temp_dir;

fn cargo_run_parse(args: &[&str]) -> (i32, String, String) {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "-q", "--"]).args(args);
    // Ensure NO_COLOR to make output stable
    cmd.env("NO_COLOR", "1");
    let out = cmd.output().expect("failed to run cargo run parse");
    let code = out.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    (code, stdout, stderr)
}

#[test]
fn cli_strict_failure_pretty_and_no_output_file() {
    let temp = create_temp_dir().expect("temp dir");
    let out_path = temp.join("failure_out.json");

    let (code, _stdout, stderr) = cargo_run_parse(&[
        "parse",
        "tests/cs_test_cases/failure.cs",
        "-o",
        out_path.to_str().unwrap(),
        // strict by default
    ]);

    assert_ne!(code, 0, "strict parse should exit non-zero");
    assert!(stderr.contains("error:"), "stderr should contain error header: {}", stderr);
    assert!(!out_path.exists(), "no output file should be created on failure");
}

#[test]
fn cli_strict_failure_json_schema() {
    let temp = create_temp_dir().expect("temp dir");
    let out_path = temp.join("failure_out.json");

    let (code, stdout, _stderr) = cargo_run_parse(&[
        "parse",
        "tests/cs_test_cases/failure.cs",
        "--errors-json",
        "-o",
        out_path.to_str().unwrap(),
    ]);

    assert_ne!(code, 0, "strict parse should exit non-zero");
    // Should be a JSON object with error fields
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON on stdout");
    let err = v.get("error").expect("error root");
    for key in ["kind", "file", "line", "column", "message"] {
        assert!(err.get(key).is_some(), "missing key '{}' in error JSON: {}", key, stdout);
    }
    assert!(!out_path.exists(), "no output file should be created on failure");
}

#[test]
fn cli_lenient_succeeds_on_failure_fixture() {
    let temp = create_temp_dir().expect("temp dir");
    let out_path = temp.join("lenient_out.json");

    let (code, _stdout, stderr) = cargo_run_parse(&[
        "parse",
        "tests/cs_test_cases/failure.cs",
        "--lenient",
        "-o",
        out_path.to_str().unwrap(),
    ]);

    assert_eq!(code, 0, "lenient parse should succeed for exploratory usage; stderr: {}", stderr);
    let content = fs::read_to_string(&out_path).expect("lenient output json exists");
    let v: serde_json::Value = serde_json::from_str(&content).expect("valid json file written");
    assert!(v.is_object());
}
