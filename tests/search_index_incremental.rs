use assert_cmd::Command;
use predicates::prelude::*;
use serde::Serialize;
use serial_test::serial;
use speq_skill::embedding::INDEX_FORMAT;
use speq_skill::search::{IndexedScenario, SearchIndex};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

mod common;

const BASE_SPECS: &str = "tests/fixtures/search_index_incremental/base/specs";
const EDITED_SPECS: &str = "tests/fixtures/search_index_incremental/edited/specs";
const RECORD_SPECS: &str = "tests/fixtures/search_index_incremental/record/specs";
const PLAN: &str = "incremental-plan";

const SCENARIO_COUNT: usize = 6;
const CHANGED_SCENARIO: &str = "library/catalog/Find a book by its author";
const ADDED_SCENARIO: &str = "library/lending/Renew a loan online";
const REMOVED_SCENARIO: &str = "library/catalog/Retire a damaged book";
const UNCHANGED_SCENARIO: &str = "library/lending/Charge a fee for a late return";
const UNCHANGED_SCENARIOS: [&str; 4] = [
    "library/catalog/Add a book to the catalog",
    "library/lending/Borrow a book with a library card",
    "library/lending/Charge a fee for a late return",
    "library/lending/Send a reminder before the due date",
];

const EMBEDDING_DIM: usize = 384;
const MARKER_VALUE: f32 = 0.5;
const UNKNOWN_MODEL_REVISION: &str = "unknown";
const FOREIGN_MODEL_REVISION: &str = "other-revision";
const MISSING_SPECS_ERROR: &str = "Specs directory not found: specs";

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

fn marker() -> Vec<f32> {
    vec![MARKER_VALUE; EMBEDDING_DIM]
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

fn remove_spec_files(dir: &Path) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            remove_spec_files(&path);
        } else if path.file_name().is_some_and(|name| name == "spec.md") {
            fs::remove_file(path).unwrap();
        }
    }
}

fn scenario_path(scenario: &IndexedScenario) -> String {
    format!(
        "{}/{}/{}",
        scenario.domain, scenario.feature, scenario.scenario
    )
}

fn scenario_paths(index: &SearchIndex) -> Vec<String> {
    index.scenarios.iter().map(scenario_path).collect()
}

fn scenarios_with_marker(index: &SearchIndex) -> Vec<String> {
    index
        .scenarios
        .iter()
        .filter(|scenario| scenario.embedding == marker())
        .map(scenario_path)
        .collect()
}

fn find_scenario<'a>(index: &'a SearchIndex, path: &str) -> &'a IndexedScenario {
    index
        .scenarios
        .iter()
        .find(|scenario| scenario_path(scenario) == path)
        .unwrap_or_else(|| panic!("the index holds {path}"))
}

fn recorded_model_revision(cache_dir: &Path) -> String {
    fs::read_to_string(cache_dir.join("models").join("revision"))
        .expect("the provisioned model cache records a model revision")
        .trim()
        .to_string()
}

/// A temporary project directory and the cache directory its commands use.
/// Its index file name derives from the project path, so no index exists for
/// it in any cache. Dropping it removes the index file it may have created.
struct Project {
    dir: TempDir,
    cache_dir: PathBuf,
    _private_cache: Option<TempDir>,
}

impl Project {
    fn without_specs() -> Self {
        common::ensure_model_cached();
        Project {
            dir: TempDir::new().unwrap(),
            cache_dir: speq_skill::search::get_cache_path(),
            _private_cache: None,
        }
    }

    fn with_specs(fixture: &str) -> Self {
        let project = Self::without_specs();
        project.add_specs(fixture);
        project
    }

    /// A project whose cache holds links to the provisioned model files and
    /// no recorded model revision.
    fn with_specs_and_unstamped_cache(fixture: &str) -> Self {
        common::ensure_model_cached();
        let cache = TempDir::new().unwrap();
        let models = cache.path().join("models");
        fs::create_dir_all(&models).unwrap();
        let provisioned = speq_skill::search::get_model_dir();
        for file in ["model.onnx", "tokenizer.json"] {
            std::os::unix::fs::symlink(provisioned.join(file), models.join(file)).unwrap();
        }

        let project = Project {
            dir: TempDir::new().unwrap(),
            cache_dir: cache.path().to_path_buf(),
            _private_cache: Some(cache),
        };
        project.add_specs(fixture);
        project
    }

    fn specs_dir(&self) -> PathBuf {
        self.dir.path().join("specs")
    }

    fn add_specs(&self, fixture: &str) {
        copy_dir_recursive(Path::new(fixture), &self.specs_dir());
    }

    fn replace_specs(&self, fixture: &str) {
        fs::remove_dir_all(self.specs_dir()).unwrap();
        self.add_specs(fixture);
    }

    fn run(&self, args: &[&str]) -> assert_cmd::assert::Assert {
        cmd()
            .current_dir(self.dir.path())
            .env("SPEQ_CACHE_DIR", &self.cache_dir)
            .args(args)
            .assert()
    }

    fn build_index(&self) -> assert_cmd::assert::Assert {
        self.run(&["search", "index"]).success()
    }

    fn index_path(&self) -> PathBuf {
        let slug = self
            .dir
            .path()
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .replace('/', "-");
        self.cache_dir.join("indexes").join(format!("{slug}.idx"))
    }

    fn read_index(&self) -> SearchIndex {
        let bytes = fs::read(self.index_path()).expect("read the index file");
        postcard::from_bytes(&bytes).expect("decode the index file")
    }

    fn write_index(&self, index: &SearchIndex) {
        fs::write(self.index_path(), postcard::to_allocvec(index).unwrap()).unwrap();
    }

    fn plant_markers(&self) {
        let mut index = self.read_index();
        for scenario in &mut index.scenarios {
            scenario.embedding = marker();
        }
        self.write_index(&index);
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_file(self.index_path());
    }
}

fn indexed_line(count: usize) -> String {
    format!("Indexed {count} scenarios.")
}

fn assert_only_changed_and_added_were_embedded(index: &SearchIndex) {
    let paths = scenario_paths(index);
    assert!(paths.contains(&CHANGED_SCENARIO.to_string()), "{paths:?}");
    assert!(paths.contains(&ADDED_SCENARIO.to_string()), "{paths:?}");
    assert!(!paths.contains(&REMOVED_SCENARIO.to_string()), "{paths:?}");
    assert_eq!(scenarios_with_marker(index), UNCHANGED_SCENARIOS);
    for path in [CHANGED_SCENARIO, ADDED_SCENARIO] {
        assert_eq!(find_scenario(index, path).embedding.len(), EMBEDDING_DIM);
    }
}

fn assert_rebuilt_in_full(project: &Project) {
    project
        .build_index()
        .stdout(predicate::str::contains(indexed_line(SCENARIO_COUNT)));

    let rebuilt = project.read_index();
    assert_eq!(rebuilt.scenarios.len(), SCENARIO_COUNT);
    assert!(scenarios_with_marker(&rebuilt).is_empty());
    assert!(
        rebuilt
            .scenarios
            .iter()
            .all(|scenario| scenario.embedding.len() == EMBEDDING_DIM)
    );
    assert_eq!(rebuilt.format, INDEX_FORMAT);
    assert_eq!(
        rebuilt.model_revision,
        recorded_model_revision(&project.cache_dir)
    );
}

#[test]
#[serial]
fn rebuild_reuses_embeddings_of_unchanged_scenarios() {
    let project = Project::with_specs(BASE_SPECS);
    project.build_index();
    project.plant_markers();
    project.replace_specs(EDITED_SPECS);

    project
        .build_index()
        .stdout(predicate::str::contains(indexed_line(SCENARIO_COUNT)));

    assert_only_changed_and_added_were_embedded(&project.read_index());
}

#[test]
#[serial]
fn rebuilt_index_stays_searchable() {
    let project = Project::with_specs(BASE_SPECS);
    project.build_index();
    project.replace_specs(EDITED_SPECS);

    project.build_index();

    let incremental = project.read_index();
    let incremental_paths = scenario_paths(&incremental);
    let distinct: HashSet<&String> = incremental_paths.iter().collect();
    assert_eq!(
        distinct.len(),
        incremental_paths.len(),
        "{incremental_paths:?}"
    );
    assert!(!incremental_paths.contains(&REMOVED_SCENARIO.to_string()));
    for path in [CHANGED_SCENARIO, ADDED_SCENARIO, UNCHANGED_SCENARIO] {
        let text = &find_scenario(&incremental, path).content;
        let query = project.run(&["search", "query", text]).success();
        let stdout = String::from_utf8_lossy(&query.get_output().stdout).into_owned();
        let first_line = stdout.lines().next().unwrap_or_default();
        assert!(
            first_line.starts_with(&format!("{path} (score: ")),
            "query for {path} listed first: {stdout}"
        );
    }

    fs::remove_file(project.index_path()).unwrap();
    project.build_index();
    assert_eq!(incremental_paths, scenario_paths(&project.read_index()));
}

#[test]
#[serial]
fn model_revision_change_reembeds_every_scenario() {
    let project = Project::with_specs(BASE_SPECS);
    project.build_index();
    project.plant_markers();
    let mut index = project.read_index();
    index.model_revision = FOREIGN_MODEL_REVISION.to_string();
    project.write_index(&index);

    assert_rebuilt_in_full(&project);
}

#[test]
#[serial]
fn index_format_change_reembeds_every_scenario() {
    let project = Project::with_specs(BASE_SPECS);
    project.build_index();
    project.plant_markers();
    let mut index = project.read_index();
    index.format = INDEX_FORMAT + 1;
    project.write_index(&index);

    assert_rebuilt_in_full(&project);
}

#[test]
#[serial]
fn index_without_model_revision_is_reused() {
    let project = Project::with_specs_and_unstamped_cache(BASE_SPECS);
    project.build_index();
    assert_eq!(project.read_index().model_revision, UNKNOWN_MODEL_REVISION);
    project.plant_markers();

    project
        .build_index()
        .stdout(predicate::str::contains(indexed_line(SCENARIO_COUNT)));

    let rebuilt = project.read_index();
    assert_eq!(scenarios_with_marker(&rebuilt).len(), SCENARIO_COUNT);
    assert_eq!(rebuilt.model_revision, UNKNOWN_MODEL_REVISION);
}

#[derive(Serialize)]
struct OlderReleaseIndex {
    scenarios: Vec<OlderReleaseScenario>,
}

#[derive(Serialize)]
struct OlderReleaseScenario {
    domain: String,
    feature: String,
    scenario: String,
    content: String,
    embedding: Vec<f32>,
}

#[test]
#[serial]
fn index_from_older_release_is_rebuilt_in_full() {
    let project = Project::with_specs(BASE_SPECS);
    project.build_index();
    let older = OlderReleaseIndex {
        scenarios: project
            .read_index()
            .scenarios
            .into_iter()
            .map(|scenario| OlderReleaseScenario {
                domain: scenario.domain,
                feature: scenario.feature,
                scenario: scenario.scenario,
                content: scenario.content,
                embedding: marker(),
            })
            .collect(),
    };
    fs::write(project.index_path(), postcard::to_allocvec(&older).unwrap()).unwrap();

    assert_rebuilt_in_full(&project);
}

#[test]
#[serial]
fn undecodable_index_is_rebuilt_in_full() {
    let project = Project::with_specs(BASE_SPECS);
    fs::create_dir_all(project.index_path().parent().unwrap()).unwrap();
    fs::write(project.index_path(), b"garbage").unwrap();

    assert_rebuilt_in_full(&project);
}

#[test]
#[serial]
fn record_reembeds_only_changed_scenarios() {
    let project = Project::with_specs(BASE_SPECS);
    project.add_specs(RECORD_SPECS);
    project.build_index();
    project.plant_markers();

    project
        .run(&["record", PLAN])
        .success()
        .stdout(predicate::str::contains(indexed_line(SCENARIO_COUNT)));

    assert_only_changed_and_added_were_embedded(&project.read_index());
}

#[test]
#[serial]
fn rebuild_after_removing_every_scenario_writes_empty_index() {
    let project = Project::with_specs(BASE_SPECS);
    project.build_index();
    remove_spec_files(&project.specs_dir());

    project
        .build_index()
        .stdout(predicate::str::contains(indexed_line(0)));

    assert!(project.read_index().scenarios.is_empty());
}

#[test]
#[serial]
fn query_without_specs_directory_fails() {
    let project = Project::without_specs();

    project
        .run(&["search", "query", "validation"])
        .code(1)
        .stdout(predicate::str::contains(MISSING_SPECS_ERROR));

    assert!(!project.index_path().exists());
}

#[test]
#[serial]
fn index_without_specs_directory_fails() {
    let project = Project::without_specs();

    project
        .run(&["search", "index"])
        .code(1)
        .stdout(predicate::str::contains(MISSING_SPECS_ERROR));

    assert!(!project.index_path().exists());
}

#[test]
#[serial]
fn query_in_project_without_scenarios_prints_no_matches() {
    let project = Project::without_specs();
    fs::create_dir(project.specs_dir()).unwrap();

    project
        .run(&["search", "query", "validation"])
        .success()
        .stdout(predicate::str::contains("No matches found."));
}
