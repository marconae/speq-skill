use assert_cmd::Command;
use serial_test::serial;
use std::path::{Path, PathBuf};

mod common;

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

fn system_cache_dir() -> String {
    speq_skill::search::get_cache_path()
        .to_string_lossy()
        .into_owned()
}

/// Absolute, symlink-resolved path to a fixture root (the directory that
/// contains a `specs/` tree). Canonicalizing keeps the child process's cwd — and
/// thus the derived index-cache slug — identical across runs.
fn fixture_root(name: &str) -> PathBuf {
    std::fs::canonicalize(PathBuf::from("tests/fixtures/search_index_errors").join(name))
        .expect("canonicalize fixture root")
}

/// Run `speq search index` in `root` against `cache_dir`, returning
/// (exit_code, stdout).
fn run_index(root: &PathBuf, cache_dir: &str) -> (Option<i32>, String) {
    let output = cmd()
        .current_dir(root)
        .env("SPEQ_CACHE_DIR", cache_dir)
        .args(["search", "index"])
        .output()
        .expect("run speq search index");
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

fn remove_index_file(index_path: &Path) {
    match std::fs::remove_file(index_path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => panic!("remove index file {}: {e}", index_path.display()),
    }
}

/// The parallel index build must fail fast on the FIRST unparseable spec in
/// discovery order (domain then feature, sorted), surfacing that spec's error
/// deterministically across runs — never an arbitrary one of the failing specs.
/// Fixture `broken/a-first` sorts before `broken/z-second`, so `a-first` must
/// always be the surfaced error.
#[test]
#[serial]
fn index_fails_on_first_unparseable_spec() {
    common::ensure_model_cached();
    let cache_dir = system_cache_dir();
    let root = fixture_root("invalid_utf8");

    let (code1, out1) = run_index(&root, &cache_dir);
    let (code2, out2) = run_index(&root, &cache_dir);

    assert_eq!(code1, Some(1), "first run must exit non-zero");
    assert_eq!(code2, Some(1), "second run must exit non-zero");

    assert!(
        out1.contains("a-first"),
        "first (discovery-order) failing spec must surface; got: {out1}"
    );
    assert!(
        !out1.contains("z-second"),
        "the second failing spec must NOT surface (fail-fast on the first); got: {out1}"
    );

    assert_eq!(
        out1, out2,
        "the surfaced error must be identical across repeated runs (deterministic first-error-in-order)"
    );
}

/// Scenario ordering in the built index is a deterministic function of
/// discovery order. Two full builds of the same specs must produce a
/// byte-identical index file: the ordered parallel collect must not reorder
/// scenarios run to run. The index file is deleted before each build, so the
/// second build embeds every scenario instead of reusing the stored vectors.
#[test]
#[serial]
fn index_scenario_ordering_is_deterministic_across_rebuilds() {
    common::ensure_model_cached();
    let cache_dir = system_cache_dir();
    let root = fixture_root("valid_ordering");

    let slug = root.to_string_lossy().replace('/', "-");
    let index_path = PathBuf::from(&cache_dir)
        .join("indexes")
        .join(format!("{slug}.idx"));

    remove_index_file(&index_path);
    let (code1, _) = run_index(&root, &cache_dir);
    assert_eq!(code1, Some(0), "first index build must succeed");
    let bytes1 = std::fs::read(&index_path).expect("read index after first build");

    remove_index_file(&index_path);
    let (code2, _) = run_index(&root, &cache_dir);
    assert_eq!(code2, Some(0), "second index build must succeed");
    let bytes2 = std::fs::read(&index_path).expect("read index after second build");

    assert!(!bytes1.is_empty(), "built index must be non-empty");
    assert_eq!(
        bytes1, bytes2,
        "rebuilt index must be byte-identical (deterministic scenario ordering)"
    );
}
