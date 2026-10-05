use std::num::NonZeroUsize;
use std::path::Path;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokenizers::Tokenizer;
use tract_onnx::prelude::*;

const MODEL_FILE: &str = "model.onnx";
const TOKENIZER_FILE: &str = "tokenizer.json";
const REVISION_FILE: &str = "revision";
const UNKNOWN_MODEL_REVISION: &str = "unknown";

/// Maximum sequence length the encoder accepts.
const MAX_SEQUENCE_LENGTH: usize = 512;

/// Output dimensionality of the embedding model.
const EMBEDDING_DIM: usize = 384;

/// Most model calls that run at once. Each call holds the activations of one
/// text, so the cap keeps the memory of inference flat on machines with many
/// cores.
const MAX_PARALLEL_MODEL_CALLS: usize = 8;

/// ONNX embedder backed by the tract inference stack.
pub struct Embedder {
    model: TypedRunnableModel<TypedModel>,
    tokenizer: Tokenizer,
    model_revision: String,
}

struct EncodedText {
    ids: Vec<i64>,
    attention_mask: Vec<i64>,
    type_ids: Vec<i64>,
}

impl Embedder {
    /// Load the embedding model from the speq model cache.
    ///
    /// Returns a fail-fast error naming the cache directory and the
    /// provisioning remedy when any required model file is missing.
    ///
    /// Reads the model revision from the `revision` stamp in the model
    /// directory before it loads the tokenizer and the model. The installer
    /// writes the stamp after it moves the model files into place, so a stamp
    /// read first is never newer than the loaded files. A missing or
    /// unreadable stamp gives the revision `unknown`.
    pub fn load_model() -> Result<Embedder, String> {
        let model_dir = crate::search::get_model_dir();
        let (model_path, tokenizer_path) = crate::search::get_model_file_paths();

        let missing: Vec<&str> = [(&model_path, MODEL_FILE), (&tokenizer_path, TOKENIZER_FILE)]
            .into_iter()
            .filter(|(path, _)| !path.exists())
            .map(|(_, name)| name)
            .collect();

        if !missing.is_empty() {
            return Err(format!(
                "Embedding model not found in {}. Run the speq-skill installer to provision the model files ({}, {}).",
                model_dir.display(),
                MODEL_FILE,
                TOKENIZER_FILE,
            ));
        }

        let model_revision = read_model_revision(&model_dir);

        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {e}"))?;

        let model = tract_onnx::onnx()
            .model_for_path(&model_path)
            .map_err(|e| format!("Failed to load ONNX model: {e}"))?
            .into_optimized()
            .map_err(|e| format!("Failed to optimize ONNX model: {e}"))?
            .into_runnable()
            .map_err(|e| format!("Failed to make ONNX model runnable: {e}"))?;

        Ok(Embedder {
            model,
            tokenizer,
            model_revision,
        })
    }

    /// The model revision recorded next to the loaded model files, or
    /// `unknown` when none is recorded. The search index stores it and
    /// reuses stored vectors only while it equals the stored text.
    pub fn model_revision(&self) -> &str {
        &self.model_revision
    }

    /// Embed each input text into a 384-dimensional, L2-normalized vector.
    ///
    /// The vector at each output position belongs to the text at the same
    /// input position. Each text is tokenized with special tokens and
    /// truncated to 512 tokens, then the model runs on that text alone, with
    /// no padding. A vector therefore depends only on its own text, not on
    /// the other texts of the call or on the number of threads. At most 8
    /// model calls run at once, so the memory of inference does not grow with
    /// the number of texts. The CLS token of each text is pooled and
    /// normalized so cosine similarity reduces to a dot product. When several
    /// texts fail, the error of the first failing text in input order is
    /// returned.
    pub fn embed(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, String> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let encoded = texts
            .iter()
            .map(|text| self.encode(text))
            .collect::<Result<Vec<_>, _>>()?;

        let available = std::thread::available_parallelism().map_or(1, NonZeroUsize::get);
        let threads = model_call_threads(available, encoded.len());
        map_in_input_order(encoded, threads, |text| embed_one(&self.model, text))
    }

    fn encode(&self, text: &str) -> Result<EncodedText, String> {
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| format!("Tokenization error: {e}"))?;
        let limit = MAX_SEQUENCE_LENGTH.min(encoding.get_ids().len());
        Ok(EncodedText {
            ids: to_i64(&encoding.get_ids()[..limit]),
            attention_mask: to_i64(&encoding.get_attention_mask()[..limit]),
            type_ids: to_i64(&encoding.get_type_ids()[..limit]),
        })
    }
}

/// Version of the search index encoding and of the way `embed` turns text
/// into vectors. An index stored under another format is rebuilt in full.
///
/// Raise this constant whenever the encoded form of the search index or the
/// vectors `embed` returns change, for example through tokenization,
/// truncation, pooling, or normalization.
pub const INDEX_FORMAT: u32 = 1;

fn read_model_revision(model_dir: &Path) -> String {
    std::fs::read_to_string(model_dir.join(REVISION_FILE)).map_or_else(
        |_| UNKNOWN_MODEL_REVISION.to_string(),
        |stamp| stamp.trim().to_string(),
    )
}

/// Widen tokenizer `u32` ids into the `i64` the ONNX graph expects.
fn to_i64(values: &[u32]) -> Vec<i64> {
    values.iter().map(|&v| i64::from(v)).collect()
}

fn single_row_tensor(row: Vec<i64>) -> Tensor {
    tract_ndarray::Array1::from(row)
        .insert_axis(tract_ndarray::Axis(0))
        .into()
}

fn embed_one(
    model: &TypedRunnableModel<TypedModel>,
    text: EncodedText,
) -> Result<Vec<f32>, String> {
    let token_count = text.ids.len();

    let outputs = model
        .run(tvec![
            single_row_tensor(text.ids).into(),
            single_row_tensor(text.attention_mask).into(),
            single_row_tensor(text.type_ids).into()
        ])
        .map_err(|e| format!("Inference error: {e}"))?;

    let last_hidden_state = outputs[0]
        .to_array_view::<f32>()
        .map_err(|e| format!("Inference error: {e}"))?;

    let shape = last_hidden_state.shape();
    if !matches!(shape, [1, tokens, EMBEDDING_DIM] if *tokens > 0) {
        return Err(format!(
            "Unexpected model output shape: got {shape:?}, expected [1, {token_count}, {EMBEDDING_DIM}]"
        ));
    }

    let cls: Vec<f32> = (0..EMBEDDING_DIM)
        .map(|dim| last_hidden_state[[0, 0, dim]])
        .collect();
    Ok(l2_normalize(&cls))
}

fn model_call_threads(available: usize, text_count: usize) -> usize {
    available
        .min(MAX_PARALLEL_MODEL_CALLS)
        .min(text_count)
        .max(1)
}

fn map_in_input_order<T, R, F>(items: Vec<T>, threads: usize, map: F) -> Result<Vec<R>, String>
where
    T: Send,
    R: Send,
    F: Fn(T) -> Result<R, String> + Send + Sync,
{
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .map_err(|e| format!("Failed to start a pool of {threads} threads for model calls: {e}"))?;
    let results: Vec<Result<R, String>> = pool.install(|| items.into_par_iter().map(map).collect());
    results.into_iter().collect()
}

/// L2-normalize a vector to unit length.
///
/// A zero-magnitude vector would divide by zero; the norm is floored to a
/// small epsilon so the result is finite rather than NaN.
fn l2_normalize(v: &[f32]) -> Vec<f32> {
    let norm = v
        .iter()
        .map(|x| x * x)
        .sum::<f32>()
        .sqrt()
        .max(f32::EPSILON);
    v.iter().map(|x| x / norm).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l2_normalize_scales_each_row_to_unit_length() {
        let row0 = l2_normalize(&[3.0f32, 4.0, 0.0]);
        let row1 = l2_normalize(&[0.0f32, 6.0, 8.0]);
        let norm0: f32 = row0.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm1: f32 = row1.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm0 - 1.0).abs() < 1e-5);
        assert!((norm1 - 1.0).abs() < 1e-5);
        assert!((row0[0] - 0.6).abs() < 1e-5);
        assert!((row0[1] - 0.8).abs() < 1e-5);
    }

    #[test]
    fn l2_normalize_zero_row_stays_finite() {
        let row = l2_normalize(&[0.0f32, 0.0, 0.0]);
        assert!(row.iter().all(|v| v.is_finite()));
    }

    #[test]
    fn model_revision_is_the_trimmed_stamp_text() {
        let model_dir = tempfile::tempdir().expect("create a model directory");
        std::fs::write(model_dir.path().join("revision"), "  d8c86521\n")
            .expect("write the revision stamp");

        assert_eq!(read_model_revision(model_dir.path()), "d8c86521");
    }

    #[test]
    fn model_revision_is_unknown_without_a_stamp() {
        let model_dir = tempfile::tempdir().expect("create a model directory");

        assert_eq!(read_model_revision(model_dir.path()), "unknown");
    }

    #[test]
    fn model_revision_is_unknown_when_the_stamp_is_unreadable() {
        let model_dir = tempfile::tempdir().expect("create a model directory");
        std::fs::create_dir(model_dir.path().join("revision"))
            .expect("create a directory in place of the stamp");

        assert_eq!(read_model_revision(model_dir.path()), "unknown");
    }

    #[test]
    fn model_call_threads_is_capped_at_eight() {
        assert_eq!(model_call_threads(64, 1000), 8);
        assert_eq!(model_call_threads(9, 1000), 8);
        assert_eq!(model_call_threads(8, 1000), 8);
    }

    #[test]
    fn model_call_threads_follows_available_parallelism_below_the_cap() {
        assert_eq!(model_call_threads(7, 1000), 7);
        assert_eq!(model_call_threads(1, 1000), 1);
    }

    #[test]
    fn model_call_threads_never_exceeds_the_text_count() {
        assert_eq!(model_call_threads(8, 3), 3);
        assert_eq!(model_call_threads(8, 1), 1);
    }

    #[test]
    fn model_call_threads_is_one_without_texts() {
        assert_eq!(model_call_threads(8, 0), 1);
        assert_eq!(model_call_threads(64, 0), 1);
    }

    #[test]
    fn map_in_input_order_returns_results_in_input_order() {
        let items: Vec<usize> = (0..1000).collect();

        let mapped = map_in_input_order(items, 8, |item| Ok(item * 2));

        assert_eq!(mapped, Ok((0..1000).map(|item| item * 2).collect()));
    }

    #[test]
    fn map_in_input_order_returns_the_first_error_in_input_order() {
        let first_failing_item = 60;
        let items: Vec<u64> = (0..1000).collect();

        let mapped: Result<Vec<u64>, String> = map_in_input_order(items, 8, |item| {
            if item == first_failing_item || item >= 500 {
                return Err(format!("item {item} failed"));
            }
            Ok((0..20_000u64).fold(item, |sum, step| {
                std::hint::black_box(sum.wrapping_add(step))
            }))
        });

        assert_eq!(mapped, Err(format!("item {first_failing_item} failed")));
    }

    #[test]
    fn map_in_input_order_runs_at_most_the_given_number_of_calls_at_once() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let running = AtomicUsize::new(0);
        let most_running = AtomicUsize::new(0);
        let items: Vec<u64> = (0..64).collect();

        let mapped = map_in_input_order(items, 2, |item| {
            let now_running = running.fetch_add(1, Ordering::SeqCst) + 1;
            most_running.fetch_max(now_running, Ordering::SeqCst);
            let work = (0..200_000u64).fold(item, |sum, step| {
                std::hint::black_box(sum.wrapping_add(step))
            });
            running.fetch_sub(1, Ordering::SeqCst);
            Ok(work)
        });

        assert!(mapped.is_ok());
        assert!(most_running.load(Ordering::SeqCst) <= 2);
    }
}
