use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;

/// Edge-case specs whose `speq feature validate` output was captured with the
/// binary built before the parser kept inline code, the description source
/// text, and the Background source text. The output must not change.
const CORPUS_DIR: &str = "tests/fixtures/validation_compat";

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

#[test]
fn validate_output_unchanged_for_compat_corpus() {
    let expected = fs::read(Path::new(CORPUS_DIR).join("expected-validate.txt")).unwrap();

    let output = cmd()
        .current_dir(CORPUS_DIR)
        .args(["feature", "validate"])
        .assert()
        .code(1)
        .get_output()
        .stdout
        .clone();

    assert_eq!(
        String::from_utf8_lossy(&output),
        String::from_utf8_lossy(&expected)
    );
}

#[test]
fn keyword_inside_inline_code_is_not_normative() {
    cmd()
        .current_dir(CORPUS_DIR)
        .args(["feature", "validate", "compat/keyword-in-code-span"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains(
            "Step in scenario 'Keyword only in code' is missing RFC 2119 keyword",
        ));
}
