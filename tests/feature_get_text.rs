use assert_cmd::Command;
use predicates::prelude::*;

const FIXTURE_DIR: &str = "tests/fixtures/spec_text";

const DESCRIPTION: &str = "The `CHAR` type pads stored values with spaces up to its declared length.\n\nPushdown keeps the padding of `CHAR(n)` columns intact.\n";

const BACKGROUND_PARAGRAPH: &str = "The connector maps `CHAR` columns to fixed-length strings.";

const BACKGROUND_BLOCK: &str = "## Background\n\nThe connector maps `CHAR` columns to fixed-length strings.\n\n* Supported string functions:\n  * `substr` with a constant length\n";

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

fn feature_get(path: &str) -> String {
    let output = cmd()
        .current_dir(FIXTURE_DIR)
        .args(["feature", "get", path])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    String::from_utf8(output).unwrap()
}

fn position_of(haystack: &str, needle: &str) -> usize {
    haystack
        .find(needle)
        .unwrap_or_else(|| panic!("expected {needle:?} in output:\n{haystack}"))
}

#[test]
fn displays_full_description_before_background() {
    let stdout = feature_get("demo/char-type");

    let description = position_of(&stdout, DESCRIPTION);
    let background_heading = position_of(&stdout, "## Background");
    let background_paragraph = position_of(&stdout, BACKGROUND_PARAGRAPH);

    assert!(description < background_heading);
    assert!(background_heading < background_paragraph);
    assert_eq!(stdout.matches(BACKGROUND_PARAGRAPH).count(), 1);
}

#[test]
fn displays_background_section_as_written() {
    let stdout = feature_get("demo/char-type");

    let background = position_of(&stdout, BACKGROUND_BLOCK);
    let first_scenario = position_of(&stdout, "### ");

    assert!(position_of(&stdout, DESCRIPTION) < background);
    assert!(background < first_scenario);
}

#[test]
fn omits_empty_background_section() {
    let stdout = feature_get("demo/empty-background");

    assert!(stdout.contains("The Background heading of this spec is followed directly"));
    assert!(!stdout.contains("## Background"));
}

#[test]
fn displays_inline_code_in_steps() {
    let stdout = feature_get("demo/char-type/Pad short values");

    assert!(stdout.contains("Given a column of type `CHAR(10)`\n"));
    assert!(stdout.contains("When the value `abc` is stored\n"));
}

#[test]
fn displays_inline_code_in_feature_and_scenario_names() {
    let stdout = feature_get("demo/char-type");

    assert_eq!(stdout.lines().next(), Some("`CHAR` type pushdown"));
    assert!(stdout.contains("### Pushdown of `substr`\n"));
}

#[test]
fn gets_scenario_by_name_with_inline_code() {
    cmd()
        .current_dir(FIXTURE_DIR)
        .args(["feature", "get", "demo/char-type/Pushdown of `substr`"])
        .assert()
        .code(0)
        .stdout(predicate::str::contains(
            "demo/char-type/Pushdown of `substr`",
        ))
        .stdout(predicate::str::contains(
            "When a query calls `substr` on the column",
        ))
        .stdout(predicate::str::contains("Pad short values").not());
}
