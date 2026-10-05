use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;

mod common;

/// Fixture with three features whose scenario content is byte-identical, so
/// their embeddings — and thus their cosine-similarity scores against any
/// query — are exactly equal. Equal scores are where a non-order-preserving
/// parallel collect would reorder results nondeterministically.
const FIXTURE_DIR: &str = "tests/fixtures/search_determinism";
const QUERY: &str = "identical setup action responds identically";

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

/// Resolve the system cache path as a `String` for passing via `SPEQ_CACHE_DIR`.
fn system_cache_dir() -> String {
    speq_skill::search::get_cache_path()
        .to_string_lossy()
        .into_owned()
}

/// Parallelizing the scoring loop must not change result ordering. With three
/// exactly-tied scores, a stable sort over a source-order collect yields one
/// fixed ordering; an unordered parallel collect would permute the tied lines
/// run-to-run. Building the index once and querying twice against that same
/// on-disk index isolates the scoring/sort/collect path, so any drift between
/// the two runs is the scoring loop's ordering, not index-build order.
#[test]
#[serial]
fn search_ranking_is_deterministic() {
    common::ensure_model_cached();
    let cache_dir = system_cache_dir();

    cmd()
        .current_dir(FIXTURE_DIR)
        .env("SPEQ_CACHE_DIR", &cache_dir)
        .args(["search", "index"])
        .assert()
        .success();

    let first = cmd()
        .current_dir(FIXTURE_DIR)
        .env("SPEQ_CACHE_DIR", &cache_dir)
        .args(["search", "query", QUERY])
        .assert()
        .success()
        .stdout(predicate::str::contains("dup/alpha"))
        .stdout(predicate::str::contains("dup/bravo"))
        .stdout(predicate::str::contains("dup/charlie"));
    let first_stdout = first.get_output().stdout.clone();

    let second = cmd()
        .current_dir(FIXTURE_DIR)
        .env("SPEQ_CACHE_DIR", &cache_dir)
        .args(["search", "query", QUERY])
        .assert()
        .success();
    let second_stdout = second.get_output().stdout.clone();

    assert_eq!(
        first_stdout, second_stdout,
        "search query output must be byte-identical across runs (deterministic tie-break ordering)"
    );
}
