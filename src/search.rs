use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::feature::discover_features;
use crate::validate::parser;

/// A searchable scenario with its embedding
#[derive(Serialize, Deserialize, Debug)]
pub struct IndexedScenario {
    pub domain: String,
    pub feature: String,
    pub scenario: String,
    pub content: String,
    pub embedding: Vec<f32>,
}

/// The search index stored on disk
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchIndex {
    pub scenarios: Vec<IndexedScenario>,
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
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from(".cache"))
        .join("speq")
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

/// Configure fastembed to use our cache directory
fn configure_fastembed_cache() {
    if std::env::var("FASTEMBED_CACHE_DIR").is_err() {
        let cache_dir = get_cache_path().join("fastembed");
        // SAFETY: We only set this once at startup before any concurrent access
        unsafe {
            std::env::set_var("FASTEMBED_CACHE_DIR", cache_dir);
        }
    }
}

/// Build the search index from all specs
pub fn index_specs(base: &Path) -> Result<usize, String> {
    use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

    // Configure fastembed cache before model initialization
    configure_fastembed_cache();

    // Initialize the embedding model
    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
    )
    .map_err(|e| format!("Failed to initialize embedding model: {}", e))?;

    // Discover all features
    let features = discover_features(base);
    let mut indexed_scenarios = Vec::new();

    for fp in features {
        let spec_path = fp.spec_path(base);
        if !spec_path.exists() {
            continue;
        }

        let content = std::fs::read_to_string(&spec_path)
            .map_err(|e| format!("Failed to read {}: {}", spec_path.display(), e))?;

        let parsed = parser::parse(&content).map_err(|e| format!("Failed to parse: {}", e))?;

        for scenario in &parsed.scenarios {
            // Build scenario content for embedding
            let steps_text: String = scenario
                .steps
                .iter()
                .map(|s| format!("{:?} {}", s.kind, s.text))
                .collect::<Vec<_>>()
                .join("\n");

            let scenario_content = format!("{}\n{}", scenario.name, steps_text);

            indexed_scenarios.push((
                fp.domain.clone(),
                fp.feature.clone(),
                scenario.name.clone(),
                scenario_content,
            ));
        }
    }

    if indexed_scenarios.is_empty() {
        return Ok(0);
    }

    // Generate embeddings for all scenarios
    let texts: Vec<&str> = indexed_scenarios.iter().map(|s| s.3.as_str()).collect();
    let embeddings = model
        .embed(texts, None)
        .map_err(|e| format!("Failed to generate embeddings: {}", e))?;

    // Build the index
    let scenarios: Vec<IndexedScenario> = indexed_scenarios
        .into_iter()
        .zip(embeddings)
        .map(
            |((domain, feature, scenario, content), embedding)| IndexedScenario {
                domain,
                feature,
                scenario,
                content,
                embedding,
            },
        )
        .collect();

    let count = scenarios.len();
    let index = SearchIndex { scenarios };

    // Save to disk
    let index_path = get_index_path();
    if let Some(parent) = index_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create index directory: {}", e))?;
    }

    let encoded = bincode::serialize(&index).map_err(|e| format!("Failed to serialize: {}", e))?;
    std::fs::write(&index_path, encoded).map_err(|e| format!("Failed to write index: {}", e))?;

    Ok(count)
}

/// Search for scenarios matching a query
pub fn search_specs(query: &str, limit: usize) -> Result<Vec<SearchResult>, String> {
    use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

    // Configure fastembed cache before model initialization
    configure_fastembed_cache();

    // Load the index
    let index_path = get_index_path();
    if !index_path.exists() {
        return Err("No search index found. Run 'speq search index' first.".to_string());
    }

    let data = std::fs::read(&index_path).map_err(|e| format!("Failed to read index: {}", e))?;
    let index: SearchIndex =
        bincode::deserialize(&data).map_err(|e| format!("Failed to deserialize index: {}", e))?;

    if index.scenarios.is_empty() {
        return Ok(Vec::new());
    }

    // Initialize the embedding model
    let model = TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2))
        .map_err(|e| format!("Failed to initialize embedding model: {}", e))?;

    // Generate query embedding
    let query_embeddings = model
        .embed(vec![query], None)
        .map_err(|e| format!("Failed to generate query embedding: {}", e))?;
    let query_embedding = &query_embeddings[0];

    // Calculate cosine similarity and rank results
    let mut scored: Vec<(f32, &IndexedScenario)> = index
        .scenarios
        .iter()
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
}
