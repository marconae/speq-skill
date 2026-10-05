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
/// exits non-zero without creating the output path, mirroring `curl -f`. When
/// `$FAKE_CURL_LOG` is set, the fake appends each requested URL to that file.
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
if [[ -n "${FAKE_CURL_LOG:-}" ]]; then
    echo "$url" >> "$FAKE_CURL_LOG"
fi
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

/// Write a fake `uname` executable into `dir` that echoes `kernel_name` for
/// `-s`/`--kernel-name` args, matching the one call shape `install.sh` uses.
fn install_fake_uname(dir: &Path, kernel_name: &str) {
    let script = format!(
        r#"#!/usr/bin/env bash
case "$1" in
    -s|--kernel-name)
        echo "{kernel_name}"
        ;;
esac
"#
    );
    let uname_path = dir.join("uname");
    fs::write(&uname_path, script).expect("write fake uname");
    let mut perms = fs::metadata(&uname_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&uname_path, perms).expect("chmod fake uname");
}

/// Run `provision_embedding_model` from `install.sh` with `curl` faked.
///
/// `fixture_dir` is the directory the fake curl copies from; pointing it at a
/// directory without the model files makes downloads fail.
fn run_provisioning(cache_dir: &Path, fixture_dir: &Path) -> std::process::Output {
    run_provisioning_logged(cache_dir, fixture_dir, None)
}

/// Like [`run_provisioning`], and when `curl_log` is given the fake `curl`
/// appends every requested URL to that file.
fn run_provisioning_logged(
    cache_dir: &Path,
    fixture_dir: &Path,
    curl_log: Option<&Path>,
) -> std::process::Output {
    let fake_bin = TempDir::new().unwrap();
    install_fake_curl(fake_bin.path());

    let original_path = std::env::var("PATH").unwrap_or_default();
    let patched_path = format!("{}:{}", fake_bin.path().display(), original_path);

    let command = format!(
        "source {} && provision_embedding_model",
        install_script().display()
    );

    let mut cmd = Command::new("bash");
    cmd.args(["-c", &command])
        .env("PATH", patched_path)
        .env("FAKE_CURL_FIXTURE_DIR", fixture_dir)
        .env("SPEQ_CACHE_DIR", cache_dir);
    match curl_log {
        Some(log) => cmd.env("FAKE_CURL_LOG", log),
        None => cmd.env_remove("FAKE_CURL_LOG"),
    };
    cmd.output().expect("run provisioning")
}

/// Source `install.sh` and read the `EMBEDDING_MODEL_REVISION` it defines.
fn pinned_revision() -> String {
    let command = format!(
        "source {} && printf '%s' \"$EMBEDDING_MODEL_REVISION\"",
        install_script().display()
    );
    let output = Command::new("bash")
        .args(["-c", &command])
        .output()
        .expect("read pinned revision");
    assert!(output.status.success(), "sourcing install.sh failed");
    String::from_utf8(output.stdout).expect("revision is utf-8")
}

/// Copy the stub model files into `<cache>/models/` and write `stamp` (when
/// given) into `models/revision`. Returns the model directory.
fn cache_stub_model(cache: &Path, stamp: Option<&str>) -> PathBuf {
    let model_dir = cache.join("models");
    fs::create_dir_all(&model_dir).unwrap();
    for filename in ["model.onnx", "tokenizer.json"] {
        fs::copy(model_stub_dir().join(filename), model_dir.join(filename)).unwrap();
    }
    if let Some(stamp) = stamp {
        fs::write(model_dir.join("revision"), format!("{stamp}\n")).unwrap();
    }
    model_dir
}

fn read_stamp(model_dir: &Path) -> String {
    fs::read_to_string(model_dir.join("revision"))
        .expect("read revision stamp")
        .trim()
        .to_string()
}

fn requested_urls(log: &Path) -> Vec<String> {
    fs::read_to_string(log)
        .unwrap_or_default()
        .lines()
        .map(str::to_string)
        .collect()
}

const OTHER_REVISION: &str = "0123456789abcdef0123456789abcdef01234567";

/// Run `provision_embedding_model` with `curl` and `uname` faked, `$HOME`
/// pinned to `home_dir`, and `$SPEQ_CACHE_DIR` left unset so the installer's
/// default (platform-dependent) cache-directory resolution is exercised.
fn run_provisioning_with_platform(
    home_dir: &Path,
    fixture_dir: &Path,
    kernel_name: &str,
    clear_xdg_cache_home: bool,
) -> std::process::Output {
    let fake_bin = TempDir::new().unwrap();
    install_fake_curl(fake_bin.path());
    install_fake_uname(fake_bin.path(), kernel_name);

    let original_path = std::env::var("PATH").unwrap_or_default();
    let patched_path = format!("{}:{}", fake_bin.path().display(), original_path);

    let command = format!(
        "source {} && provision_embedding_model",
        install_script().display()
    );

    let mut cmd = Command::new("bash");
    cmd.args(["-c", &command])
        .env("PATH", patched_path)
        .env("FAKE_CURL_FIXTURE_DIR", fixture_dir)
        .env("HOME", home_dir)
        .env_remove("SPEQ_CACHE_DIR");

    if clear_xdg_cache_home {
        cmd.env_remove("XDG_CACHE_HOME");
    }

    cmd.output().expect("run provisioning")
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
    cache_stub_model(cache.path(), Some(&pinned_revision()));

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

#[test]
fn installer_provisions_model_into_macos_cache_dir() {
    let home = TempDir::new().unwrap();

    let output = run_provisioning_with_platform(home.path(), &model_stub_dir(), "Darwin", false);

    assert!(
        output.status.success(),
        "provisioning failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let model_dir = home
        .path()
        .join("Library")
        .join("Caches")
        .join("speq")
        .join("models");
    assert!(model_dir.join("model.onnx").exists());
    assert!(model_dir.join("tokenizer.json").exists());
}

#[test]
fn installer_provisions_model_into_linux_cache_dir() {
    let home = TempDir::new().unwrap();

    let output = run_provisioning_with_platform(home.path(), &model_stub_dir(), "Linux", true);

    assert!(
        output.status.success(),
        "provisioning failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let model_dir = home.path().join(".cache").join("speq").join("models");
    assert!(model_dir.join("model.onnx").exists());
    assert!(model_dir.join("tokenizer.json").exists());
}

#[test]
fn installer_downloads_model_from_pinned_revision() {
    let cache = TempDir::new().unwrap();
    let log_dir = TempDir::new().unwrap();
    let log = log_dir.path().join("curl.log");
    let pin = pinned_revision();

    let output = run_provisioning_logged(cache.path(), &model_stub_dir(), Some(&log));

    assert!(
        output.status.success(),
        "provisioning failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(pin.len(), 40, "pin must be a 40-character commit id: {pin}");
    assert!(
        pin.chars()
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c)),
        "pin must be lowercase hexadecimal: {pin}"
    );

    let urls = requested_urls(&log);
    assert_eq!(urls.len(), 2, "expected two downloads, got: {urls:?}");
    for url in &urls {
        assert!(
            url.contains(&format!("/resolve/{pin}/")),
            "URL is not pinned to the revision: {url}"
        );
    }
    assert_eq!(read_stamp(&cache.path().join("models")), pin);
}

#[test]
fn installer_redownloads_model_when_revision_stamp_is_missing_or_differs() {
    let pin = pinned_revision();

    for stamp in [None, Some(OTHER_REVISION)] {
        let cache = TempDir::new().unwrap();
        let log_dir = TempDir::new().unwrap();
        let log = log_dir.path().join("curl.log");
        let model_dir = cache_stub_model(cache.path(), stamp);

        let output = run_provisioning_logged(cache.path(), &model_stub_dir(), Some(&log));

        assert!(
            output.status.success(),
            "provisioning failed for stamp {stamp:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let urls = requested_urls(&log);
        assert!(
            urls.iter().any(|u| u.ends_with("/onnx/model.onnx")),
            "model.onnx was not requested for stamp {stamp:?}: {urls:?}"
        );
        assert!(
            urls.iter().any(|u| u.ends_with("/tokenizer.json")),
            "tokenizer.json was not requested for stamp {stamp:?}: {urls:?}"
        );
        assert_eq!(read_stamp(&model_dir), pin);
    }
}

#[test]
fn installer_keeps_cached_model_when_download_fails() {
    let cache = TempDir::new().unwrap();
    let model_dir = cache_stub_model(cache.path(), Some(OTHER_REVISION));
    fs::write(model_dir.join("model.onnx"), "cached model").unwrap();
    fs::write(model_dir.join("tokenizer.json"), "cached tokenizer").unwrap();

    let partial_source = TempDir::new().unwrap();
    fs::copy(
        model_stub_dir().join("model.onnx"),
        partial_source.path().join("model.onnx"),
    )
    .unwrap();

    let output = run_provisioning(cache.path(), partial_source.path());

    assert!(
        !output.status.success(),
        "provisioning should fail when tokenizer.json cannot be downloaded"
    );
    assert_eq!(
        fs::read_to_string(model_dir.join("model.onnx")).unwrap(),
        "cached model"
    );
    assert_eq!(
        fs::read_to_string(model_dir.join("tokenizer.json")).unwrap(),
        "cached tokenizer"
    );
    assert_eq!(read_stamp(&model_dir), OTHER_REVISION);

    let leftover: Vec<_> = fs::read_dir(&model_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".tmp"))
        .collect();
    assert!(
        leftover.is_empty(),
        "temporary file left behind: {leftover:?}"
    );

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        combined.contains("printf '%s\\n'") && combined.contains("/revision"),
        "expected manual stamp command, got: {combined}"
    );
}
