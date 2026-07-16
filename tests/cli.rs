use std::path::PathBuf;
use std::process::Command;

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_miyagi"))
}

#[test]
fn help_lists_the_operational_commands() {
    let output = binary().arg("--help").output().unwrap();
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    for command in [
        "inspect",
        "info",
        "compose",
        "eval",
        "apply",
        "search",
        "benchmark",
    ] {
        assert!(text.contains(command), "missing {command} in help: {text}");
    }
}

#[test]
fn info_reads_checked_in_legacy_patch_without_a_model() {
    let patch = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../python/bankai/patches/patch_math_v1.json");
    let output = binary()
        .args(["--json", "info", "--patch"])
        .arg(patch)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["patch"]["format"], "bankai_row_xor_v1");
    assert_eq!(report["patch"]["flips"].as_array().unwrap().len(), 72);
}
