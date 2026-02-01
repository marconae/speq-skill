use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

mod version {
    use super::*;

    #[test]
    fn prints_version_with_long_flag() {
        cmd()
            .arg("--version")
            .assert()
            .success()
            .stdout(predicate::str::is_match(r"^speq \d+\.\d+\.\d+\n$").unwrap());
    }

    #[test]
    fn prints_version_with_short_flag() {
        cmd()
            .arg("-V")
            .assert()
            .success()
            .stdout(predicate::str::is_match(r"^speq \d+\.\d+\.\d+\n$").unwrap());
    }
}
