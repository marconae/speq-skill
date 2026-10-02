//! Integration tests for the MCP server handling in `install.sh`:
//! `ask_yes_no` and `offer_mcp_servers`.
//!
//! These tests source `install.sh` and invoke the functions directly, with
//! `uv`, `serena`, `claude`, and `codex` replaced by fake scripts on a
//! restricted `PATH` (a temp dir of fakes, plus `/usr/bin:/bin` only, never
//! the real `PATH`, so a real tool on the host can never leak in and silently
//! change the outcome). `SPEQ_TTY` points the prompts at a file, so a test
//! answers `y` or `n` without a terminal.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

const CLAUDE_SERENA: &str = "mcp add --scope user serena -- serena start-mcp-server --context claude-code --project-from-cwd";
const CODEX_SERENA: &str =
    "mcp add serena -- serena start-mcp-server --project-from-cwd --context=codex";

/// Absolute path to the repository's `install.sh`.
fn install_script() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("install.sh")
}

fn write_executable(path: &Path, contents: &str) {
    fs::write(path, contents).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
    let mut perms = fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).unwrap_or_else(|e| panic!("chmod {}: {e}", path.display()));
}

/// A restricted PATH: `dir` first, then only `/usr/bin:/bin`.
fn restricted_path(dir: &Path) -> String {
    format!("{}:/usr/bin:/bin", dir.display())
}

/// Guard against a stale assumption: panics if `cmd` is resolvable on the
/// bare `/usr/bin:/bin` PATH, since that would invalidate any test that
/// relies on `cmd` being absent.
fn assert_absent_from_restricted_path(cmd: &str) {
    let output = Command::new("bash")
        .args(["-c", &format!("command -v {cmd}")])
        .env("PATH", "/usr/bin:/bin")
        .output()
        .expect("run command -v");
    if output.status.success() {
        panic!(
            "'{cmd}' is unexpectedly resolvable on /usr/bin:/bin ({}); \
             this test's absence assumption is invalid on this machine",
            String::from_utf8_lossy(&output.stdout).trim()
        );
    }
}

/// Fake `uv` that appends its arguments to a log file and exits with
/// `$FAKE_UV_EXIT` (default 0).
fn install_fake_uv(dir: &Path) -> PathBuf {
    let log = dir.join("uv.log");
    let script = format!(
        r#"#!/usr/bin/env bash
echo "$@" >> "{log}"
exit "${{FAKE_UV_EXIT:-0}}"
"#,
        log = log.display()
    );
    write_executable(&dir.join("uv"), &script);
    log
}

/// Fake `serena` that exists on PATH and exits 0 for anything.
fn install_fake_serena(dir: &Path) {
    write_executable(&dir.join("serena"), "#!/usr/bin/env bash\nexit 0\n");
}

/// Fake `claude`. `plugin list` prints one block per entry of
/// `$FAKE_CLAUDE_PLUGINS` in the real format (space separated). An entry is
/// `<name>@<marketplace>` (enabled) or `<name>@<marketplace>:disabled`.
/// `mcp get <name>` exits 0 when `<name>` is a word of `$FAKE_CLAUDE_MCP`,
/// otherwise 1. `mcp add` appends its arguments to a log file and exits with
/// `$FAKE_CLAUDE_ADD_EXIT` (default 0).
fn install_fake_claude(dir: &Path) -> PathBuf {
    let log = dir.join("claude.log");
    let script = format!(
        r#"#!/usr/bin/env bash
if [[ "$1" == "plugin" && "$2" == "list" ]]; then
    echo "Installed plugins:"
    echo ""
    for p in ${{FAKE_CLAUDE_PLUGINS:-}}; do
        name="${{p%%:*}}"
        if [[ "$p" == *:disabled ]]; then status="✘ disabled"; else status="✔ enabled"; fi
        echo "  ❯ $name"
        echo "    Version: 1.0.0"
        echo "    Scope: user"
        echo "    Status: $status"
        echo ""
    done
    exit 0
fi
if [[ "$1" == "mcp" ]]; then
    case "$2" in
        get)
            for r in ${{FAKE_CLAUDE_MCP:-}}; do
                [[ "$r" == "$3" ]] && exit 0
            done
            exit 1
            ;;
        add)
            echo "mcp add ${{*:3}}" >> "{log}"
            exit "${{FAKE_CLAUDE_ADD_EXIT:-0}}"
            ;;
    esac
fi
exit 1
"#,
        log = log.display()
    );
    write_executable(&dir.join("claude"), &script);
    log
}

/// Fake `codex`. `mcp get <name>` exits 0 when `<name>` is a word of
/// `$FAKE_CODEX_REGISTERED`, otherwise 1. `mcp add` and `mcp remove` append
/// their arguments to a log file and exit with `$FAKE_CODEX_ADD_EXIT` or
/// `$FAKE_CODEX_REMOVE_EXIT` (default 0).
fn install_fake_codex(dir: &Path) -> PathBuf {
    let log = dir.join("codex.log");
    let script = format!(
        r#"#!/usr/bin/env bash
if [[ "$1" == "mcp" ]]; then
    case "$2" in
        get)
            for r in ${{FAKE_CODEX_REGISTERED:-}}; do
                [[ "$r" == "$3" ]] && exit 0
            done
            exit 1
            ;;
        add)
            echo "mcp add ${{*:3}}" >> "{log}"
            exit "${{FAKE_CODEX_ADD_EXIT:-0}}"
            ;;
        remove)
            echo "mcp remove ${{*:3}}" >> "{log}"
            exit "${{FAKE_CODEX_REMOVE_EXIT:-0}}"
            ;;
    esac
fi
exit 1
"#,
        log = log.display()
    );
    write_executable(&dir.join("codex"), &script);
    log
}

/// Write a file that answers the prompts with `answer` and return its path.
fn answer_file(dir: &Path, answer: &str) -> PathBuf {
    let path = dir.join("tty");
    fs::write(&path, answer).unwrap();
    path
}

fn run_function(function: &str, path: &str, envs: &[(&str, &str)]) -> Output {
    let command = format!("source {} && {function}", install_script().display());
    let mut cmd = Command::new("bash");
    cmd.args(["-c", &command]).env("PATH", path);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    cmd.output().expect("run install.sh function")
}

fn read_log(log: &Path) -> String {
    fs::read_to_string(log).unwrap_or_default()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn assert_success(output: &Output, function: &str) {
    assert!(
        output.status.success(),
        "{function} failed: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

// ---------------------------------------------------------------------
// ask_yes_no
// ---------------------------------------------------------------------

#[test]
fn ask_yes_no_accepts_y_and_rejects_everything_else() {
    let fake_bin = TempDir::new().unwrap();
    let path = restricted_path(fake_bin.path());

    for (answer, expect_yes) in [("y", true), ("Y", true), ("n", false), ("\n", false)] {
        let tty = answer_file(fake_bin.path(), answer);
        let output = run_function(
            "ask_yes_no 'Install?'",
            &path,
            &[("SPEQ_TTY", tty.to_str().unwrap())],
        );
        assert_eq!(
            output.status.success(),
            expect_yes,
            "answer {answer:?} gave status {:?}",
            output.status
        );
    }
}

#[test]
fn ask_yes_no_says_no_without_a_terminal() {
    let fake_bin = TempDir::new().unwrap();
    let path = restricted_path(fake_bin.path());
    let missing = fake_bin.path().join("no-such-tty");

    let output = run_function(
        "ask_yes_no 'Install?'",
        &path,
        &[("SPEQ_TTY", missing.to_str().unwrap())],
    );

    assert!(!output.status.success());
}

// ---------------------------------------------------------------------
// offer_mcp_servers: Claude Code
// ---------------------------------------------------------------------

#[test]
fn offer_adds_serena_to_the_user_scope_on_yes() {
    let fake_bin = TempDir::new().unwrap();
    install_fake_serena(fake_bin.path());
    let claude_log = install_fake_claude(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "y");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[("SPEQ_TTY", tty.to_str().unwrap())],
    );

    assert_success(&output, "offer_mcp_servers");
    let log = read_log(&claude_log);
    assert!(log.contains(CLAUDE_SERENA), "log: {log}");
    assert!(
        !log.contains("plugin"),
        "no plugin or marketplace use: {log}"
    );
    assert!(
        !log.contains("context7"),
        "speq does not manage context7: {log}"
    );
}

#[test]
fn offer_installs_the_serena_cli_with_uv_before_the_claude_registration() {
    let fake_bin = TempDir::new().unwrap();
    let uv_log = install_fake_uv(fake_bin.path());
    assert_absent_from_restricted_path("serena");
    let claude_log = install_fake_claude(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "y");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[("SPEQ_TTY", tty.to_str().unwrap())],
    );

    assert_success(&output, "offer_mcp_servers");
    assert!(read_log(&uv_log).contains("tool install"), "uv must run");
    assert!(read_log(&claude_log).contains(CLAUDE_SERENA));
}

#[test]
fn offer_skips_claude_when_serena_is_registered() {
    let fake_bin = TempDir::new().unwrap();
    let claude_log = install_fake_claude(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "y");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[
            ("SPEQ_TTY", tty.to_str().unwrap()),
            ("FAKE_CLAUDE_MCP", "serena context7"),
        ],
    );

    assert_success(&output, "offer_mcp_servers");
    assert_eq!(read_log(&claude_log), "", "nothing must be added");
    assert_eq!(stdout(&output).trim(), "");
}

#[test]
fn offer_skips_claude_when_the_serena_plugin_is_enabled() {
    let fake_bin = TempDir::new().unwrap();
    let claude_log = install_fake_claude(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "y");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[
            ("SPEQ_TTY", tty.to_str().unwrap()),
            ("FAKE_CLAUDE_PLUGINS", "serena@my-marketplace"),
        ],
    );

    assert_success(&output, "offer_mcp_servers");
    assert_eq!(read_log(&claude_log), "");
    assert_eq!(stdout(&output).trim(), "");
}

#[test]
fn offer_adds_serena_when_only_a_disabled_plugin_exists() {
    let fake_bin = TempDir::new().unwrap();
    install_fake_serena(fake_bin.path());
    let claude_log = install_fake_claude(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "y");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[
            ("SPEQ_TTY", tty.to_str().unwrap()),
            (
                "FAKE_CLAUDE_PLUGINS",
                "serena@claude-plugins-official:disabled",
            ),
        ],
    );

    assert_success(&output, "offer_mcp_servers");
    assert!(read_log(&claude_log).contains(CLAUDE_SERENA));
}

#[test]
fn offer_does_not_mistake_a_similar_plugin_name_for_serena() {
    let fake_bin = TempDir::new().unwrap();
    install_fake_serena(fake_bin.path());
    let claude_log = install_fake_claude(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "y");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[
            ("SPEQ_TTY", tty.to_str().unwrap()),
            ("FAKE_CLAUDE_PLUGINS", "my-serena@x"),
        ],
    );

    assert_success(&output, "offer_mcp_servers");
    assert!(read_log(&claude_log).contains(CLAUDE_SERENA));
}

#[test]
fn offer_installs_nothing_on_no() {
    let fake_bin = TempDir::new().unwrap();
    let claude_log = install_fake_claude(fake_bin.path());
    let codex_log = install_fake_codex(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "n");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[("SPEQ_TTY", tty.to_str().unwrap())],
    );

    assert_success(&output, "offer_mcp_servers");
    assert_eq!(read_log(&claude_log), "");
    assert_eq!(read_log(&codex_log), "");
}

#[test]
fn offer_prints_the_commands_without_a_terminal() {
    let fake_bin = TempDir::new().unwrap();
    let claude_log = install_fake_claude(fake_bin.path());
    let codex_log = install_fake_codex(fake_bin.path());
    let path = restricted_path(fake_bin.path());
    let missing = fake_bin.path().join("no-such-tty");

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[("SPEQ_TTY", missing.to_str().unwrap())],
    );

    assert_success(&output, "offer_mcp_servers");
    assert_eq!(read_log(&claude_log), "");
    assert_eq!(read_log(&codex_log), "");
    let out = stdout(&output);
    assert!(
        out.contains(&format!("claude {CLAUDE_SERENA}")),
        "stdout: {out}"
    );
    assert!(
        out.contains(&format!("codex {CODEX_SERENA}")),
        "stdout: {out}"
    );
    assert!(!out.contains("context7"), "stdout: {out}");
    let uv = out.find("uv tool install").expect("uv line missing");
    assert!(
        uv < out.find(CLAUDE_SERENA).unwrap(),
        "uv install must come before the registration: {out}"
    );
}

#[test]
fn offer_omits_the_uv_line_without_a_terminal_when_serena_is_present() {
    let fake_bin = TempDir::new().unwrap();
    install_fake_serena(fake_bin.path());
    install_fake_claude(fake_bin.path());
    let path = restricted_path(fake_bin.path());
    let missing = fake_bin.path().join("no-such-tty");

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[("SPEQ_TTY", missing.to_str().unwrap())],
    );

    assert_success(&output, "offer_mcp_servers");
    let out = stdout(&output);
    assert!(out.contains(CLAUDE_SERENA), "stdout: {out}");
    assert!(!out.contains("uv tool install"), "stdout: {out}");
}

#[test]
fn offer_continues_when_claude_add_fails() {
    let fake_bin = TempDir::new().unwrap();
    install_fake_serena(fake_bin.path());
    install_fake_claude(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "y");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[
            ("SPEQ_TTY", tty.to_str().unwrap()),
            ("FAKE_CLAUDE_ADD_EXIT", "1"),
        ],
    );

    assert_success(&output, "offer_mcp_servers");
    let out = stdout(&output);
    assert!(out.contains("failed"), "expected a failure warning: {out}");
    assert!(
        out.contains(&format!("claude {CLAUDE_SERENA}")),
        "expected the manual command: {out}"
    );
}

// ---------------------------------------------------------------------
// offer_mcp_servers: Codex
// ---------------------------------------------------------------------

#[test]
fn offer_adds_missing_codex_servers_on_yes() {
    let fake_bin = TempDir::new().unwrap();
    let codex_log = install_fake_codex(fake_bin.path());
    install_fake_serena(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "y");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[("SPEQ_TTY", tty.to_str().unwrap())],
    );

    assert_success(&output, "offer_mcp_servers");
    let log = read_log(&codex_log);
    assert!(log.contains(CODEX_SERENA), "log: {log}");
    assert!(!log.contains("context7"), "log: {log}");
}

#[test]
fn offer_skips_codex_servers_that_are_registered() {
    let fake_bin = TempDir::new().unwrap();
    let codex_log = install_fake_codex(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "y");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[
            ("SPEQ_TTY", tty.to_str().unwrap()),
            ("FAKE_CODEX_REGISTERED", "serena context7"),
        ],
    );

    assert_success(&output, "offer_mcp_servers");
    assert_eq!(read_log(&codex_log), "");
    assert_eq!(stdout(&output).trim(), "");
}

#[test]
fn offer_installs_the_serena_cli_with_uv_before_registering() {
    assert_absent_from_restricted_path("serena");

    let fake_bin = TempDir::new().unwrap();
    let codex_log = install_fake_codex(fake_bin.path());
    let uv_log = install_fake_uv(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "y");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[
            ("SPEQ_TTY", tty.to_str().unwrap()),
            ("FAKE_CODEX_REGISTERED", "context7"),
        ],
    );

    assert_success(&output, "offer_mcp_servers");
    assert_eq!(
        read_log(&uv_log).trim(),
        "tool install -p 3.13 serena-agent"
    );
    assert!(read_log(&codex_log).contains(CODEX_SERENA));
}

#[test]
fn offer_skips_codex_serena_when_uv_is_missing() {
    assert_absent_from_restricted_path("serena");
    assert_absent_from_restricted_path("uv");

    let fake_bin = TempDir::new().unwrap();
    let codex_log = install_fake_codex(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "y");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[("SPEQ_TTY", tty.to_str().unwrap())],
    );

    assert_success(&output, "offer_mcp_servers must not fail without uv");
    let log = read_log(&codex_log);
    assert!(
        !log.contains("add serena"),
        "serena needs its CLI first: {log}"
    );
    assert!(!log.contains("context7"), "log: {log}");
    let out = stdout(&output);
    assert!(out.contains("uv not found"), "stdout: {out}");
    assert!(
        out.contains("uv tool install -p 3.13 serena-agent"),
        "stdout: {out}"
    );
}

#[test]
fn offer_continues_when_uv_tool_install_fails() {
    assert_absent_from_restricted_path("serena");

    let fake_bin = TempDir::new().unwrap();
    let codex_log = install_fake_codex(fake_bin.path());
    install_fake_uv(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "y");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[("SPEQ_TTY", tty.to_str().unwrap()), ("FAKE_UV_EXIT", "1")],
    );

    assert_success(&output, "offer_mcp_servers");
    assert!(!read_log(&codex_log).contains("add serena"));
    assert!(stdout(&output).contains("uv tool install -p 3.13 serena-agent"));
}

#[test]
fn offer_continues_when_codex_add_fails() {
    let fake_bin = TempDir::new().unwrap();
    install_fake_codex(fake_bin.path());
    install_fake_serena(fake_bin.path());
    let tty = answer_file(fake_bin.path(), "y");
    let path = restricted_path(fake_bin.path());

    let output = run_function(
        "offer_mcp_servers",
        &path,
        &[
            ("SPEQ_TTY", tty.to_str().unwrap()),
            ("FAKE_CODEX_ADD_EXIT", "1"),
        ],
    );

    assert_success(&output, "offer_mcp_servers");
    let out = stdout(&output);
    assert!(out.contains("failed"), "expected a failure warning: {out}");
    assert!(
        out.contains(&format!("codex {CODEX_SERENA}")),
        "expected the manual command: {out}"
    );
}

#[test]
fn offer_does_nothing_when_no_host_is_installed() {
    let fake_bin = TempDir::new().unwrap();
    assert_absent_from_restricted_path("claude");
    assert_absent_from_restricted_path("codex");
    let path = restricted_path(fake_bin.path());

    let output = run_function("offer_mcp_servers", &path, &[]);

    assert_success(&output, "offer_mcp_servers");
    assert_eq!(stdout(&output).trim(), "");
}
