use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use tokenizers::Tokenizer;

const WEIGHTS_FILE: &str = "model.safetensors";
const TOKENIZER_FILE: &str = "tokenizer.json";
const CONFIG_FILE: &str = "config.json";

/// Maximum sequence length the BERT encoder accepts.
const MAX_SEQUENCE_LENGTH: usize = 512;

/// Output dimensionality of the embedding model.
const EMBEDDING_DIM: usize = 384;

/// Pure-Rust BERT embedder backed by the candle inference stack.
pub struct Embedder {
    pub model: BertModel,
    pub tokenizer: Tokenizer,
    pub device: Device,
}

impl Embedder {
    /// Load the embedding model from the speq model cache.
    ///
    /// Returns a fail-fast error naming the cache directory and the
    /// provisioning remedy when any required model file is missing.
    pub fn load_model() -> Result<Embedder, String> {
        let (weights_path, tokenizer_path, config_path) = crate::search::get_model_file_paths();

        let missing: Vec<&str> = [
            (&weights_path, WEIGHTS_FILE),
            (&tokenizer_path, TOKENIZER_FILE),
            (&config_path, CONFIG_FILE),
        ]
        .into_iter()
        .filter(|(path, _)| !path.exists())
        .map(|(_, name)| name)
        .collect();

        if !missing.is_empty() {
            let dir = crate::search::get_model_dir();
            return Err(format!(
                "Embedding model not found in {}. Run the speq-skill installer to provision the model files ({}, {}, {}).",
                dir.display(),
                WEIGHTS_FILE,
                TOKENIZER_FILE,
                CONFIG_FILE,
            ));
        }

        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {e}"))?;

        let config_text = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read model config: {e}"))?;
        let config: Config = serde_json::from_str(&config_text)
            .map_err(|e| format!("Failed to parse model config: {e}"))?;

        let device = Device::Cpu;

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_path], DType::F32, &device)
                .map_err(|e| format!("Failed to load model weights: {e}"))?
        };
        let model =
            BertModel::load(vb, &config).map_err(|e| format!("Failed to build BERT model: {e}"))?;

        Ok(Embedder {
            model,
            tokenizer,
            device,
        })
    }

    /// Embed each input text into a 384-dimensional, L2-normalized vector.
    ///
    /// Texts are tokenized individually with special tokens, truncated to 512
    /// tokens, then padded to the longest sequence in the batch. The batched
    /// tensors run through the BERT encoder once; the CLS token of each row is
    /// pooled and normalized so cosine similarity reduces to a dot product.
    pub fn embed(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, String> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let mut id_rows: Vec<Vec<u32>> = Vec::with_capacity(texts.len());
        let mut mask_rows: Vec<Vec<u32>> = Vec::with_capacity(texts.len());
        let mut type_rows: Vec<Vec<u32>> = Vec::with_capacity(texts.len());

        for text in texts {
            let encoding = self
                .tokenizer
                .encode(*text, true)
                .map_err(|e| format!("Tokenization error: {e}"))?;
            let limit = MAX_SEQUENCE_LENGTH.min(encoding.get_ids().len());
            id_rows.push(encoding.get_ids()[..limit].to_vec());
            mask_rows.push(encoding.get_attention_mask()[..limit].to_vec());
            type_rows.push(encoding.get_type_ids()[..limit].to_vec());
        }

        let seq_len = id_rows.iter().map(Vec::len).max().unwrap_or(0).max(1);
        let input_ids = self.batch_tensor(&id_rows, seq_len)?;
        let attention_mask_ids = self.batch_tensor(&mask_rows, seq_len)?;
        let token_type_ids = self.batch_tensor(&type_rows, seq_len)?;

        let attention_mask = attention_mask_ids
            .to_dtype(DType::F32)
            .map_err(infer_error)?;

        let output = self
            .model
            .forward(&input_ids, &token_type_ids, Some(&attention_mask))
            .map_err(infer_error)?;

        let cls = output
            .narrow(1, 0, 1)
            .map_err(infer_error)?
            .squeeze(1)
            .map_err(infer_error)?;

        let normalized = l2_normalize(&cls)?;
        let rows: Vec<Vec<f32>> = normalized
            .contiguous()
            .map_err(infer_error)?
            .to_vec2::<f32>()
            .map_err(infer_error)?;

        for row in &rows {
            if row.len() != EMBEDDING_DIM {
                return Err(format!(
                    "Unexpected embedding dimensionality: got {}, expected {EMBEDDING_DIM}",
                    row.len()
                ));
            }
        }

        Ok(rows)
    }

    /// Pad each row to `seq_len` with zeros and stack into a `[batch, seq_len]`
    /// `u32` tensor on the embedder's device.
    fn batch_tensor(&self, rows: &[Vec<u32>], seq_len: usize) -> Result<Tensor, String> {
        let mut flat: Vec<u32> = Vec::with_capacity(rows.len() * seq_len);
        for row in rows {
            flat.extend_from_slice(row);
            flat.extend(std::iter::repeat_n(0u32, seq_len - row.len()));
        }
        Tensor::from_vec(flat, (rows.len(), seq_len), &self.device).map_err(infer_error)
    }
}

/// Map a candle inference error into an actionable string.
fn infer_error(e: candle_core::Error) -> String {
    format!("Inference error: {e}")
}

/// L2-normalize each row of a `[batch, dim]` tensor to unit length.
///
/// A zero-magnitude row would divide by zero; norms are floored to a small
/// epsilon so the result is finite rather than NaN.
fn l2_normalize(vectors: &Tensor) -> Result<Tensor, String> {
    let norms = vectors
        .sqr()
        .map_err(infer_error)?
        .sum_keepdim(1)
        .map_err(infer_error)?
        .sqrt()
        .map_err(infer_error)?;
    let safe_norms = norms
        .clamp(f32::EPSILON, f32::INFINITY)
        .map_err(infer_error)?;
    vectors.broadcast_div(&safe_norms).map_err(infer_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row_norm(row: &[f32]) -> f32 {
        row.iter().map(|x| x * x).sum::<f32>().sqrt()
    }

    #[test]
    fn l2_normalize_scales_each_row_to_unit_length() {
        let device = Device::Cpu;
        let input =
            Tensor::from_vec(vec![3.0f32, 4.0, 0.0, 6.0, 0.0, 8.0], (2, 3), &device).unwrap();

        let normalized = l2_normalize(&input).unwrap();
        let rows = normalized.to_vec2::<f32>().unwrap();

        assert!((row_norm(&rows[0]) - 1.0).abs() < 1e-5);
        assert!((row_norm(&rows[1]) - 1.0).abs() < 1e-5);
        assert!((rows[0][0] - 0.6).abs() < 1e-5);
        assert!((rows[0][1] - 0.8).abs() < 1e-5);
    }

    #[test]
    fn l2_normalize_zero_row_stays_finite() {
        let device = Device::Cpu;
        let input = Tensor::from_vec(vec![0.0f32, 0.0, 0.0], (1, 3), &device).unwrap();

        let normalized = l2_normalize(&input).unwrap();
        let rows = normalized.to_vec2::<f32>().unwrap();

        assert!(rows[0].iter().all(|v| v.is_finite()));
    }
}
