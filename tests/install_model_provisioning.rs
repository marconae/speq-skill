//! Integration tests for `install.sh`'s `provision_embedding_model` step.
//!
//! These tests source `install.sh` and invoke `provision_embedding_model`
//! directly with `curl` replaced by a fake script on `PATH` that copies from
//! `tests/fixtures/model-stub/` instead of downloading. No live HuggingFace or
//! GitHub network calls are made.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

/// Absolute path to the repository's `install.sh`.
fn install_script() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("install.sh")
}

/// Absolute path to the model-stub fixture directory.
fn model_stub_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("model-stub")
}

/// Write a fake `curl` executable into `dir` that copies fixture files instead
/// of downloading.
///
/// The fake honors the exact argument shape `provision_embedding_model` uses
/// (`curl -fsSL <url> -o <output>`). It resolves the requested file by URL
/// basename inside `$FAKE_CURL_FIXTURE_DIR`. When the fixture file is absent it
/// exits non-zero without creating the output path, mirroring `curl -f`.
fn install_fake_curl(dir: &Path) {
    let script = r#"#!/usr/bin/env bash
url=""
output=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        -o)
            output="$2"
            shift 2
            ;;
        -*)
            shift
            ;;
        *)
            url="$1"
            shift
            ;;
    esac
done
filename=$(basename "$url")
src="${FAKE_CURL_FIXTURE_DIR}/${filename}"
if [[ -f "$src" ]]; then
    cp "$src" "$output"
    exit 0
fi
echo "curl: (22) The requested URL returned error: 404" >&2
exit 22
"#;
    let curl_path = dir.join("curl");
    fs::write(&curl_path, script).expect("write fake curl");
    let mut perms = fs::metadata(&curl_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&curl_path, perms).expect("chmod fake curl");
}

/// Run `provision_embedding_model` from `install.sh` with `curl` faked.
///
/// `fixture_dir` is the directory the fake curl copies from; pointing it at a
/// directory without the model files makes downloads fail.
fn run_provisioning(cache_dir: &Path, fixture_dir: &Path) -> std::process::Output {
    let fake_bin = TempDir::new().unwrap();
    install_fake_curl(fake_bin.path());

    let original_path = std::env::var("PATH").unwrap_or_default();
    let patched_path = format!("{}:{}", fake_bin.path().display(), original_path);

    let command = format!(
        "source {} && provision_embedding_model",
        install_script().display()
    );

    Command::new("bash")
        .args(["-c", &command])
        .env("PATH", patched_path)
        .env("FAKE_CURL_FIXTURE_DIR", fixture_dir)
        .env("SPEQ_CACHE_DIR", cache_dir)
        .output()
        .expect("run provisioning")
}

#[test]
fn installer_provisions_model_on_clean_machine() {
    let cache = TempDir::new().unwrap();

    let output = run_provisioning(cache.path(), &model_stub_dir());

    assert!(
        output.status.success(),
        "provisioning failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let model_dir = cache.path().join("models");
    assert!(model_dir.join("model.onnx").exists());
    assert!(model_dir.join("tokenizer.json").exists());
}

#[test]
fn installer_skips_provisioning_when_model_cached() {
    let cache = TempDir::new().unwrap();
    let model_dir = cache.path().join("models");
    fs::create_dir_all(&model_dir).unwrap();
    for filename in ["model.onnx", "tokenizer.json"] {
        fs::copy(model_stub_dir().join(filename), model_dir.join(filename)).unwrap();
    }

    // Point the fake curl at an empty dir so any download attempt would fail —
    // the test passes only if provisioning is skipped entirely.
    let empty_source = TempDir::new().unwrap();
    let output = run_provisioning(cache.path(), empty_source.path());

    assert!(
        output.status.success(),
        "provisioning should succeed when already cached: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("already provisioned"),
        "expected 'already provisioned' message, got: {stdout}"
    );
}

#[test]
fn installer_reports_error_on_model_download_failure() {
    let cache = TempDir::new().unwrap();

    // Empty fixture source — the fake curl exits non-zero for every file.
    let empty_source = TempDir::new().unwrap();
    let output = run_provisioning(cache.path(), empty_source.path());

    assert!(
        !output.status.success(),
        "provisioning should fail when downloads are unreachable"
    );

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        combined.to_lowercase().contains("error"),
        "expected a clear error message, got: {combined}"
    );

    // No partial `.tmp` file may remain behind.
    let model_dir = cache.path().join("models");
    if model_dir.exists() {
        let leftover: Vec<_> = fs::read_dir(&model_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().ends_with(".tmp"))
            .collect();
        assert!(
            leftover.is_empty(),
            "partial .tmp file left behind: {leftover:?}"
        );
    }
}

#[test]
fn installer_provisions_model_into_custom_cache_dir() {
    let custom_cache = TempDir::new().unwrap();

    let output = run_provisioning(custom_cache.path(), &model_stub_dir());

    assert!(
        output.status.success(),
        "provisioning failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Files must land under the SPEQ_CACHE_DIR-derived models/ directory.
    let model_dir = custom_cache.path().join("models");
    assert!(model_dir.join("model.onnx").exists());
    assert!(model_dir.join("tokenizer.json").exists());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(&model_dir.display().to_string()),
        "expected provisioning message to name the custom model dir, got: {stdout}"
    );
}
