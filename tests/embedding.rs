use std::sync::OnceLock;

use speq_skill::embedding::Embedder;

mod common;

const SHORT_TEXT_COUNT: usize = 19;
const REPEATS_PER_LENGTH_STEP: usize = 3;
const OVERLONG_TEXT_REPEATS: usize = 200;

static EMBEDDER: OnceLock<Embedder> = OnceLock::new();

fn embedder() -> &'static Embedder {
    EMBEDDER.get_or_init(|| {
        common::ensure_model_cached();
        Embedder::load_model().expect("load the provisioned embedding model")
    })
}

/// Twenty distinct texts from a few tokens up to several hundred, plus one
/// text far over the 512-token limit, so the texts of one call differ in
/// length the way scenarios do.
fn texts_of_mixed_lengths() -> Vec<String> {
    let mut texts: Vec<String> = (0..SHORT_TEXT_COUNT)
        .map(|step| {
            format!(
                "Scenario {step}: {}",
                "the user validates a spec file ".repeat(step * REPEATS_PER_LENGTH_STEP)
            )
        })
        .collect();
    texts.push("an overlong scenario step ".repeat(OVERLONG_TEXT_REPEATS));
    texts
}

fn as_strs(texts: &[String]) -> Vec<&str> {
    texts.iter().map(String::as_str).collect()
}

fn bits(vector: &[f32]) -> Vec<u32> {
    vector.iter().map(|value| value.to_bits()).collect()
}

#[test]
fn load_model_reports_the_model_revision_recorded_in_the_cache() {
    let embedder = embedder();
    let stamp = std::fs::read_to_string(speq_skill::search::get_model_dir().join("revision"))
        .expect("read the model revision stamp the installer wrote");

    assert_eq!(embedder.model_revision(), stamp.trim());
}

#[test]
fn embed_returns_each_vector_at_its_input_position() {
    let embedder = embedder();
    let texts = texts_of_mixed_lengths();
    let inputs = as_strs(&texts);

    let together = embedder
        .embed(&inputs)
        .expect("embed all texts in one call");

    assert_eq!(together.len(), inputs.len());
    for (position, text) in inputs.iter().enumerate() {
        let alone = embedder.embed(&[text]).expect("embed one text alone");
        assert_eq!(
            bits(&together[position]),
            bits(&alone[0]),
            "the vector at position {position} differs from the vector of its text embedded alone"
        );
    }
}

#[test]
fn embed_is_deterministic_across_calls() {
    let embedder = embedder();
    let texts = texts_of_mixed_lengths();
    let inputs = as_strs(&texts);

    let first = embedder.embed(&inputs).expect("embed in the first call");
    let second = embedder.embed(&inputs).expect("embed in the second call");

    let first_bits: Vec<Vec<u32>> = first.iter().map(|vector| bits(vector)).collect();
    let second_bits: Vec<Vec<u32>> = second.iter().map(|vector| bits(vector)).collect();
    assert_eq!(first_bits, second_bits);
}
