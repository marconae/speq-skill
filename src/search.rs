use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::embedding::{Embedder, INDEX_FORMAT};
use crate::feature::discover_features;
use crate::validate::parser;

/// A searchable scenario with its embedding
#[derive(Serialize, Deserialize, Debug)]
pub struct IndexedScenario {
    pub domain: String,
    pub feature: String,
    pub scenario: String,
    /// The exact text passed to the embedder: the scenario name followed by
    /// its steps. A build looks up stored vectors by this text, so it is the
    /// reuse key and MUST stay identical to the embedded text.
    pub content: String,
    pub embedding: Vec<f32>,
}

/// The search index stored on disk. It doubles as the embedding cache.
///
/// A build reuses the stored vector of every scenario whose embedded text is
/// unchanged, but only from an index whose `format` equals
/// `crate::embedding::INDEX_FORMAT` and whose `model_revision` equals the
/// revision of the loaded model. Any other index is rebuilt in full. Any
/// change to the encoded form of this type or of `IndexedScenario` MUST raise
/// `INDEX_FORMAT`, so a file in the old form counts as outdated.
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchIndex {
    /// The index format of the speq release that wrote the file.
    pub format: u32,
    /// The model revision the stored vectors were computed with.
    pub model_revision: String,
    pub scenarios: Vec<IndexedScenario>,
}

/// Why a search builds the index before it ranks scenarios.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildReason {
    /// No index file exists for the project.
    MissingIndex,
    /// An index file exists but is not reusable.
    OutdatedIndex,
}

enum StoredIndex {
    Reusable(SearchIndex),
    NeedsBuild(BuildReason),
}

struct ParsedScenario {
    domain: String,
    feature: String,
    scenario: String,
    content: String,
}

/// Search result with similarity score
#[derive(Debug)]
pub struct SearchResult {
    pub domain: String,
    pub feature: String,
    pub scenario: String,
    pub content: String,
    pub score: f32,
}

/// Get the cache directory path for speq
pub fn get_cache_path() -> PathBuf {
    let local_cache = PathBuf::from(".cache").join("speq");

    if let Ok(path) = std::env::var("SPEQ_CACHE_DIR") {
        return PathBuf::from(path);
    }

    if let Some(system_cache) = dirs::cache_dir() {
        let system_cache = system_cache.join("speq");
        if cache_path_is_writable(&system_cache) {
            return system_cache;
        }
    }

    local_cache
}

fn cache_path_is_writable(path: &Path) -> bool {
    if std::fs::create_dir_all(path).is_err() {
        return false;
    }

    let probe = path.join(".write-test");
    match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&probe)
    {
        Ok(_) => {
            let _ = std::fs::remove_file(probe);
            true
        }
        Err(_) => false,
    }
}

/// Get the project slug from the current working directory
pub fn get_project_slug() -> String {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .to_string_lossy()
        .replace('/', "-")
}

/// Get the index file path for the current project
pub fn get_index_path() -> PathBuf {
    let cache = get_cache_path();
    let slug = get_project_slug();
    cache.join("indexes").join(format!("{}.idx", slug))
}

/// Get the model directory path for speq's embedding model files
pub fn get_model_dir() -> PathBuf {
    get_cache_path().join("models")
}

/// Get the two expected model file paths as (model.onnx, tokenizer.json)
pub fn get_model_file_paths() -> (PathBuf, PathBuf) {
    let model_dir = get_model_dir();
    (
        model_dir.join("model.onnx"),
        model_dir.join("tokenizer.json"),
    )
}

fn read_stored_index(index_path: &Path, model_revision: &str) -> StoredIndex {
    match std::fs::read(index_path) {
        Ok(bytes) => decode_reusable_index(&bytes, model_revision).map_or(
            StoredIndex::NeedsBuild(BuildReason::OutdatedIndex),
            StoredIndex::Reusable,
        ),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            StoredIndex::NeedsBuild(BuildReason::MissingIndex)
        }
        Err(_) => StoredIndex::NeedsBuild(BuildReason::OutdatedIndex),
    }
}

fn decode_reusable_index(bytes: &[u8], model_revision: &str) -> Option<SearchIndex> {
    let index: SearchIndex = postcard::from_bytes(bytes).ok()?;
    let reusable = index.format == INDEX_FORMAT && index.model_revision == model_revision;
    reusable.then_some(index)
}

/// Build the search index from all specs and write it.
///
/// The build reuses the stored vector of every scenario whose embedded text
/// appears in a reusable index and embeds only the other texts, so a rebuild
/// after a small spec change runs few model calls. Deleting the index file
/// forces a full build. Returns the total number of scenarios in the index.
pub fn index_specs(base: &Path) -> Result<usize, String> {
    let embedder = Embedder::load_model()?;
    Ok(build_index(base, &embedder)?.scenarios.len())
}

fn build_index(base: &Path, embedder: &Embedder) -> Result<SearchIndex, String> {
    if !base.is_dir() {
        return Err(format!("Specs directory not found: {}", base.display()));
    }

    let parsed = parse_scenarios(base)?;
    let index_path = get_index_path();
    let mut vectors = stored_vectors(read_stored_index(&index_path, embedder.model_revision()));
    let new_texts = texts_without_vector(&parsed, &vectors);
    let new_vectors = embedder.embed(&new_texts)?;
    vectors.extend(new_texts.into_iter().map(str::to_string).zip(new_vectors));

    let index = SearchIndex {
        format: INDEX_FORMAT,
        model_revision: embedder.model_revision().to_string(),
        scenarios: assemble_scenarios(parsed, &vectors)?,
    };
    write_index(&index_path, &index)?;
    Ok(index)
}

fn parse_scenarios(base: &Path) -> Result<Vec<ParsedScenario>, String> {
    // Discover all features (deterministically ordered: domains then features).
    let features = discover_features(base);

    // Parse each spec in parallel. Stage 1 collects an ORDERED
    // `Vec<Result<_, String>>`: rayon preserves index order for an
    // `IndexedParallelIterator`, so batches stay in discovery order and a
    // missing spec maps to an empty batch (the serial `continue`).
    let batches: Vec<Result<Vec<ParsedScenario>, String>> = features
        .par_iter()
        .map(|fp| {
            let spec_path = fp.spec_path(base);
            if !spec_path.exists() {
                return Ok(Vec::new());
            }

            let content = std::fs::read_to_string(&spec_path)
                .map_err(|e| format!("Failed to read {}: {}", spec_path.display(), e))?;

            let parsed = parser::parse(&content).map_err(|e| format!("Failed to parse: {}", e))?;

            let scenarios = parsed
                .spec
                .scenarios
                .iter()
                .map(|scenario| {
                    let steps_text: String = scenario
                        .steps
                        .iter()
                        .map(|s| format!("{:?} {}", s.kind, s.display_text))
                        .collect::<Vec<_>>()
                        .join("\n");

                    ParsedScenario {
                        domain: fp.domain.clone(),
                        feature: fp.feature.clone(),
                        scenario: scenario.name.clone(),
                        content: format!("{}\n{}", scenario.name, steps_text),
                    }
                })
                .collect();

            Ok(scenarios)
        })
        .collect();

    // Stage 2: fold the ordered Vec SEQUENTIALLY so the FIRST discovery-order
    // `Err` surfaces (matching the serial `?`-on-first-error), then flatten in
    // order. A direct parallel `collect::<Result<Vec<_>>>()` would surface an
    // arbitrary error when multiple specs fail.
    Ok(batches
        .into_iter()
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .flatten()
        .collect())
}

fn stored_vectors(stored: StoredIndex) -> HashMap<String, Vec<f32>> {
    match stored {
        StoredIndex::Reusable(index) => index
            .scenarios
            .into_iter()
            .map(|scenario| (scenario.content, scenario.embedding))
            .collect(),
        StoredIndex::NeedsBuild(_) => HashMap::new(),
    }
}

fn texts_without_vector<'a>(
    scenarios: &'a [ParsedScenario],
    vectors: &HashMap<String, Vec<f32>>,
) -> Vec<&'a str> {
    let mut listed = HashSet::new();
    scenarios
        .iter()
        .map(|scenario| scenario.content.as_str())
        .filter(|text| !vectors.contains_key(*text) && listed.insert(*text))
        .collect()
}

fn assemble_scenarios(
    scenarios: Vec<ParsedScenario>,
    vectors: &HashMap<String, Vec<f32>>,
) -> Result<Vec<IndexedScenario>, String> {
    scenarios
        .into_iter()
        .map(|parsed| {
            let embedding = vectors.get(&parsed.content).cloned().ok_or_else(|| {
                format!(
                    "Failed to assemble the search index: no embedding for scenario {}/{}/{}",
                    parsed.domain, parsed.feature, parsed.scenario
                )
            })?;
            Ok(IndexedScenario {
                domain: parsed.domain,
                feature: parsed.feature,
                scenario: parsed.scenario,
                content: parsed.content,
                embedding,
            })
        })
        .collect()
}

fn write_index(index_path: &Path, index: &SearchIndex) -> Result<(), String> {
    let encoded = postcard::to_allocvec(index)
        .map_err(|e| format!("Failed to serialize the search index: {e}"))?;
    if let Some(index_dir) = index_path.parent() {
        std::fs::create_dir_all(index_dir).map_err(|e| {
            format!(
                "Failed to create index directory {}: {e}",
                index_dir.display()
            )
        })?;
    }

    let temporary_path = temporary_index_path(index_path);
    let replaced = std::fs::write(&temporary_path, encoded)
        .and_then(|()| std::fs::rename(&temporary_path, index_path));
    let Err(e) = replaced else {
        return Ok(());
    };
    let mut message = format!("Failed to write index {}: {e}", index_path.display());
    match std::fs::remove_file(&temporary_path) {
        Ok(()) => {}
        Err(cleanup_error) if cleanup_error.kind() == std::io::ErrorKind::NotFound => {}
        Err(cleanup_error) => message.push_str(&format!(
            "; the temporary file {} was not removed: {cleanup_error}",
            temporary_path.display()
        )),
    }
    Err(message)
}

fn temporary_index_path(index_path: &Path) -> PathBuf {
    let mut file_name = index_path.file_name().unwrap_or_default().to_os_string();
    file_name.push(format!(".{}.tmp", std::process::id()));
    index_path.with_file_name(file_name)
}

/// Search for scenarios matching a query.
///
/// Loads the embedding model first, then checks the project's index with the
/// same reuse rule as a build. That check runs no model call. When the index
/// is missing or outdated, `on_build` runs once with the reason, the index is
/// rebuilt with the loaded model, and the query runs against the rebuilt
/// index. `on_build` never runs for a reusable index, so the caller decides
/// how to tell the user about the wait.
pub fn search_specs(
    query: &str,
    limit: usize,
    on_build: impl FnOnce(BuildReason),
) -> Result<Vec<SearchResult>, String> {
    let embedder = Embedder::load_model()?;
    let index = match read_stored_index(&get_index_path(), embedder.model_revision()) {
        StoredIndex::Reusable(index) => index,
        StoredIndex::NeedsBuild(reason) => {
            on_build(reason);
            build_index(Path::new("specs"), &embedder)?
        }
    };

    if index.scenarios.is_empty() {
        return Ok(Vec::new());
    }

    // Generate query embedding
    let query_embeddings = embedder.embed(&[query])?;
    let query_embedding = query_embeddings
        .first()
        .ok_or_else(|| "Embedding produced no vector".to_string())?;

    // Calculate cosine similarity and rank results
    let mut scored: Vec<(f32, &IndexedScenario)> = index
        .scenarios
        .par_iter()
        .map(|s| (cosine_similarity(query_embedding, &s.embedding), s))
        .collect();

    // Sort by score descending
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    // Take top results
    let results: Vec<SearchResult> = scored
        .into_iter()
        .take(limit)
        .filter(|(score, _)| *score > 0.0) // Filter out zero similarity
        .map(|(score, s)| SearchResult {
            domain: s.domain.clone(),
            feature: s.feature.clone(),
            scenario: s.scenario.clone(),
            content: s.content.clone(),
            score,
        })
        .collect();

    Ok(results)
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_model_dir_is_under_cache() {
        let cache = get_cache_path();
        let model_dir = get_model_dir();
        assert_eq!(model_dir, cache.join("models"));
    }

    #[test]
    fn test_get_model_file_paths_are_under_model_dir() {
        let model_dir = get_model_dir();
        let (onnx, tokenizer) = get_model_file_paths();
        assert_eq!(onnx, model_dir.join("model.onnx"));
        assert_eq!(tokenizer, model_dir.join("tokenizer.json"));
    }

    #[test]
    fn test_get_cache_path() {
        let path = get_cache_path();
        assert!(path.to_string_lossy().contains("speq"));
    }

    #[test]
    fn test_get_project_slug() {
        let slug = get_project_slug();
        assert!(!slug.is_empty());
        assert!(!slug.contains('/'));
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 0.0001);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![-1.0, 0.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim + 1.0).abs() < 0.0001);
    }

    const CURRENT_MODEL_REVISION: &str = "d8c86521";

    fn index_with(format: u32, model_revision: &str) -> SearchIndex {
        SearchIndex {
            format,
            model_revision: model_revision.to_string(),
            scenarios: vec![IndexedScenario {
                domain: "library".to_string(),
                feature: "catalog".to_string(),
                scenario: "Add a book".to_string(),
                content: "Add a book\nGiven a new book".to_string(),
                embedding: vec![0.25; 3],
            }],
        }
    }

    fn encode(index: &SearchIndex) -> Vec<u8> {
        postcard::to_allocvec(index).unwrap()
    }

    #[test]
    fn garbage_bytes_are_not_a_reusable_index() {
        assert!(decode_reusable_index(b"garbage", CURRENT_MODEL_REVISION).is_none());
    }

    #[test]
    fn index_of_another_format_is_not_reusable() {
        let bytes = encode(&index_with(INDEX_FORMAT + 1, CURRENT_MODEL_REVISION));

        assert!(decode_reusable_index(&bytes, CURRENT_MODEL_REVISION).is_none());
    }

    #[test]
    fn index_of_another_model_revision_is_not_reusable() {
        let bytes = encode(&index_with(INDEX_FORMAT, "other-revision"));

        assert!(decode_reusable_index(&bytes, CURRENT_MODEL_REVISION).is_none());
    }

    #[test]
    fn index_of_current_format_and_model_revision_is_reusable() {
        let bytes = encode(&index_with(INDEX_FORMAT, CURRENT_MODEL_REVISION));

        let index =
            decode_reusable_index(&bytes, CURRENT_MODEL_REVISION).expect("the index is reusable");

        assert_eq!(index.scenarios.len(), 1);
        assert_eq!(index.scenarios[0].content, "Add a book\nGiven a new book");
        assert_eq!(index.scenarios[0].embedding, vec![0.25; 3]);
    }

    #[test]
    fn missing_index_file_needs_a_build_for_a_missing_index() {
        let dir = tempfile::tempdir().unwrap();

        let stored = read_stored_index(&dir.path().join("absent.idx"), CURRENT_MODEL_REVISION);

        assert!(matches!(
            stored,
            StoredIndex::NeedsBuild(BuildReason::MissingIndex)
        ));
    }

    #[test]
    fn unreadable_index_file_needs_a_build_for_an_outdated_index() {
        let dir = tempfile::tempdir().unwrap();

        let stored = read_stored_index(dir.path(), CURRENT_MODEL_REVISION);

        assert!(matches!(
            stored,
            StoredIndex::NeedsBuild(BuildReason::OutdatedIndex)
        ));
    }

    #[test]
    fn index_file_of_another_model_revision_needs_a_build_for_an_outdated_index() {
        let dir = tempfile::tempdir().unwrap();
        let index_path = dir.path().join("project.idx");
        std::fs::write(&index_path, encode(&index_with(INDEX_FORMAT, "other"))).unwrap();

        let stored = read_stored_index(&index_path, CURRENT_MODEL_REVISION);

        assert!(matches!(
            stored,
            StoredIndex::NeedsBuild(BuildReason::OutdatedIndex)
        ));
    }

    #[test]
    fn reusable_index_file_is_read_as_reusable() {
        let dir = tempfile::tempdir().unwrap();
        let index_path = dir.path().join("project.idx");
        let bytes = encode(&index_with(INDEX_FORMAT, CURRENT_MODEL_REVISION));
        std::fs::write(&index_path, bytes).unwrap();

        let stored = read_stored_index(&index_path, CURRENT_MODEL_REVISION);

        assert!(matches!(stored, StoredIndex::Reusable(index) if index.scenarios.len() == 1));
    }

    fn parsed(scenario: &str, content: &str) -> ParsedScenario {
        ParsedScenario {
            domain: "library".to_string(),
            feature: "catalog".to_string(),
            scenario: scenario.to_string(),
            content: content.to_string(),
        }
    }

    fn vectors_of(entries: &[(&str, f32)]) -> HashMap<String, Vec<f32>> {
        entries
            .iter()
            .map(|(text, value)| ((*text).to_string(), vec![*value; 3]))
            .collect()
    }

    #[test]
    fn texts_without_vector_lists_each_new_text_once_in_first_occurrence_order() {
        let scenarios = [
            parsed("one", "b"),
            parsed("two", "a"),
            parsed("three", "b"),
            parsed("four", "stored"),
            parsed("five", "a"),
            parsed("six", "c"),
        ];
        let vectors = vectors_of(&[("stored", 0.5)]);

        assert_eq!(texts_without_vector(&scenarios, &vectors), ["b", "a", "c"]);
    }

    #[test]
    fn texts_without_vector_is_empty_when_every_text_has_a_vector() {
        let scenarios = [parsed("one", "a"), parsed("two", "b")];
        let vectors = vectors_of(&[("a", 0.1), ("b", 0.2)]);

        assert!(texts_without_vector(&scenarios, &vectors).is_empty());
    }

    #[test]
    fn assemble_scenarios_keeps_discovery_order_and_shares_vectors_of_identical_texts() {
        let scenarios = vec![parsed("one", "b"), parsed("two", "a"), parsed("three", "b")];
        let vectors = vectors_of(&[("a", 0.1), ("b", 0.2)]);

        let assembled = assemble_scenarios(scenarios, &vectors).unwrap();

        let names: Vec<&str> = assembled.iter().map(|s| s.scenario.as_str()).collect();
        assert_eq!(names, ["one", "two", "three"]);
        assert_eq!(assembled[0].embedding, vec![0.2; 3]);
        assert_eq!(assembled[1].embedding, vec![0.1; 3]);
        assert_eq!(assembled[2].embedding, vec![0.2; 3]);
        assert_eq!(assembled[1].content, "a");
    }

    #[test]
    fn assemble_scenarios_fails_for_a_text_without_vector() {
        let scenarios = vec![parsed("one", "a"), parsed("Lost scenario", "b")];
        let vectors = vectors_of(&[("a", 0.1)]);

        let error = assemble_scenarios(scenarios, &vectors).unwrap_err();

        assert!(error.contains("library/catalog/Lost scenario"), "{error}");
    }

    fn file_names_in(dir: &Path) -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(dir)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }

    #[test]
    fn write_index_creates_the_index_directory() {
        let cache = tempfile::tempdir().unwrap();
        let index_path = cache.path().join("indexes").join("project.idx");

        write_index(
            &index_path,
            &index_with(INDEX_FORMAT, CURRENT_MODEL_REVISION),
        )
        .unwrap();

        assert_eq!(
            file_names_in(&cache.path().join("indexes")),
            ["project.idx"]
        );
    }

    #[test]
    fn write_index_replaces_the_index_file_and_leaves_no_temporary_file() {
        let dir = tempfile::tempdir().unwrap();
        let index_path = dir.path().join("project.idx");
        std::fs::write(&index_path, b"previous index").unwrap();

        write_index(
            &index_path,
            &index_with(INDEX_FORMAT, CURRENT_MODEL_REVISION),
        )
        .unwrap();

        let written = std::fs::read(&index_path).unwrap();
        assert!(decode_reusable_index(&written, CURRENT_MODEL_REVISION).is_some());
        assert_eq!(file_names_in(dir.path()), ["project.idx"]);
    }

    #[test]
    fn failed_index_rename_removes_the_temporary_file_and_names_the_index() {
        let dir = tempfile::tempdir().unwrap();
        let index_path = dir.path().join("project.idx");
        std::fs::create_dir(&index_path).unwrap();
        std::fs::write(index_path.join("occupied"), b"").unwrap();

        let error = write_index(
            &index_path,
            &index_with(INDEX_FORMAT, CURRENT_MODEL_REVISION),
        )
        .unwrap_err();

        assert!(error.contains(&index_path.display().to_string()), "{error}");
        assert_eq!(file_names_in(dir.path()), ["project.idx"]);
    }

    #[test]
    fn failed_index_write_names_a_temporary_file_it_cannot_remove() {
        let dir = tempfile::tempdir().unwrap();
        let index_path = dir.path().join("project.idx");
        let temporary_path = temporary_index_path(&index_path);
        std::fs::create_dir(&temporary_path).unwrap();
        std::fs::write(temporary_path.join("occupied"), b"").unwrap();

        let error = write_index(
            &index_path,
            &index_with(INDEX_FORMAT, CURRENT_MODEL_REVISION),
        )
        .unwrap_err();

        assert!(
            error.contains(&temporary_path.display().to_string()),
            "{error}"
        );
    }
}
