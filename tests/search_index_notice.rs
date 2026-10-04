use assert_cmd::Command;
use serial_test::serial;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use tempfile::TempDir;

const FIXTURE_SPECS: &str = "tests/fixtures/search_index_notice/specs";
const QUERY: &str = "document validation";
const NOTICE: &str = "Info: no search index found. Building it now, this may take a while.";
const RESULT_LINE: &str = "notice/sample/Valid document passes validation";

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

static MODEL_CACHED: OnceLock<()> = OnceLock::new();

/// Provision the embedding-model files into the system model cache once per
/// test process, mirroring `tests/search_determinism.rs::ensure_model_cached`.
fn ensure_model_cached() {
    MODEL_CACHED.get_or_init(|| {
        let model_dir = speq_skill::search::get_model_dir();
        std::fs::create_dir_all(&model_dir).expect("create model dir");
        let files = [
            (
                "https://huggingface.co/Snowflake/snowflake-arctic-embed-xs/resolve/main/onnx/model.onnx",
                "model.onnx",
            ),
            (
                "https://huggingface.co/Snowflake/snowflake-arctic-embed-xs/resolve/main/tokenizer.json",
                "tokenizer.json",
            ),
        ];
        for (url, filename) in files {
            let dest = model_dir.join(filename);
            if dest.exists() {
                continue;
            }
            let tmp = format!("{}.tmp", dest.display());
            let status = std::process::Command::new("curl")
                .args(["-fsSL", url, "-o", &tmp])
                .status()
                .expect("invoke curl");
            assert!(status.success(), "Failed to download {filename}");
            std::fs::rename(&tmp, &dest).expect("rename model file into place");
        }
    });
}

fn system_cache_dir() -> String {
    speq_skill::search::get_cache_path()
        .to_string_lossy()
        .into_owned()
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let target = dst.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir_recursive(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// A fresh project directory holding the fixture specs. Its index file name
/// derives from the project path, so no index exists for it in any cache.
/// Dropping it removes the index file it may have created in the cache.
struct Project {
    dir: TempDir,
    cache_dir: String,
}

impl Project {
    fn new() -> Self {
        ensure_model_cached();
        let dir = TempDir::new().unwrap();
        copy_dir_recursive(Path::new(FIXTURE_SPECS), &dir.path().join("specs"));
        Project {
            dir,
            cache_dir: system_cache_dir(),
        }
    }

    fn run(&self, args: &[&str]) -> assert_cmd::assert::Assert {
        cmd()
            .current_dir(self.dir.path())
            .env("SPEQ_CACHE_DIR", &self.cache_dir)
            .args(args)
            .assert()
    }

    fn index_path(&self) -> PathBuf {
        let slug = self
            .dir
            .path()
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .replace('/', "-");
        Path::new(&self.cache_dir)
            .join("indexes")
            .join(format!("{slug}.idx"))
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_file(self.index_path());
    }
}

#[test]
#[serial]
fn query_without_index_prints_build_notice_on_stderr() {
    let project = Project::new();

    let first = project.run(&["search", "query", QUERY]).success();
    let first_output = first.get_output();
    let first_stdout = String::from_utf8_lossy(&first_output.stdout).into_owned();
    let first_stderr = String::from_utf8_lossy(&first_output.stderr).into_owned();

    assert!(first_stderr.contains(NOTICE), "stderr was: {first_stderr}");
    assert!(
        first_stdout.contains(RESULT_LINE),
        "stdout was: {first_stdout}"
    );
    assert!(
        !first_stdout.contains("Info:"),
        "stdout was: {first_stdout}"
    );

    let second = project.run(&["search", "query", QUERY]).success();
    assert_eq!(first_output.stdout, second.get_output().stdout);
}

#[test]
#[serial]
fn query_with_existing_index_prints_no_build_notice() {
    let project = Project::new();
    project.run(&["search", "index"]).success();

    let query = project.run(&["search", "query", QUERY]).success();
    let output = query.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    assert!(stdout.contains(RESULT_LINE), "stdout was: {stdout}");
    assert!(!stdout.contains("no search index found"));
    assert!(!stderr.contains("no search index found"));
}
