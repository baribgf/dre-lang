use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(rel)
}

fn run_compiler(args: &[PathBuf]) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_drec"));
    for arg in args {
        cmd.arg(arg);
    }

    cmd.output().expect("failed to run compiler binary")
}

fn stderr_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

#[test]
fn parses_minimal_program_successfully() {
    let output = run_compiler(&[fixture("valid/minimal.dr")]);

    assert!(
        output.status.success(),
        "compiler should succeed, stderr: {}",
        stderr_text(&output)
    );
}

#[test]
fn parses_feature_rich_program_successfully() {
    let output = run_compiler(&[fixture("valid/construct_mix.dr")]);

    assert!(
        output.status.success(),
        "compiler should succeed, stderr: {}",
        stderr_text(&output)
    );
}

#[test]
fn parses_all_valid_fixtures_successfully() {
    let valid_dir = fixture("valid");
    let mut fixtures: Vec<PathBuf> = fs::read_dir(&valid_dir)
        .expect("failed to read valid fixtures directory")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == "dr")
                    .unwrap_or(false)
        })
        .collect();

    fixtures.sort();
    assert!(!fixtures.is_empty(), "no valid fixtures discovered");

    let output = run_compiler(&fixtures);

    assert!(
        output.status.success(),
        "compiler should succeed for all valid fixtures, stderr: {}",
        stderr_text(&output)
    );
}

#[test]
fn rejects_protocol_inheritance_cycle() {
    let output = run_compiler(&[fixture("invalid/semantic_protocol_cycle.dr")]);

    assert!(
        !output.status.success(),
        "compiler should fail on protocol inheritance cycle"
    );
}

#[test]
fn rejects_struct_conformance_when_imposed_protocols_cycle() {
    let output = run_compiler(&[fixture("invalid/semantic_struct_protocol_cycle.dr")]);

    assert!(
        !output.status.success(),
        "compiler should fail when a struct imposes a protocol graph with a cycle"
    );
}
