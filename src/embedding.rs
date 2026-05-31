use tokenizers::Tokenizer;
use tract_onnx::prelude::*;

const MODEL_FILE: &str = "model.onnx";
const TOKENIZER_FILE: &str = "tokenizer.json";

/// Maximum sequence length the encoder accepts.
const MAX_SEQUENCE_LENGTH: usize = 512;

/// Output dimensionality of the embedding model.
const EMBEDDING_DIM: usize = 384;

/// ONNX embedder backed by the tract inference stack.
pub struct Embedder {
    model: TypedRunnableModel<TypedModel>,
    tokenizer: Tokenizer,
}

impl Embedder {
    /// Load the embedding model from the speq model cache.
    ///
    /// Returns a fail-fast error naming the cache directory and the
    /// provisioning remedy when any required model file is missing.
    pub fn load_model() -> Result<Embedder, String> {
        let (model_path, tokenizer_path) = crate::search::get_model_file_paths();

        let missing: Vec<&str> = [(&model_path, MODEL_FILE), (&tokenizer_path, TOKENIZER_FILE)]
            .into_iter()
            .filter(|(path, _)| !path.exists())
            .map(|(_, name)| name)
            .collect();

        if !missing.is_empty() {
            let dir = crate::search::get_model_dir();
            return Err(format!(
                "Embedding model not found in {}. Run the speq-skill installer to provision the model files ({}, {}).",
                dir.display(),
                MODEL_FILE,
                TOKENIZER_FILE,
            ));
        }

        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {e}"))?;

        let model = tract_onnx::onnx()
            .model_for_path(&model_path)
            .map_err(|e| format!("Failed to load ONNX model: {e}"))?
            .into_optimized()
            .map_err(|e| format!("Failed to optimize ONNX model: {e}"))?
            .into_runnable()
            .map_err(|e| format!("Failed to make ONNX model runnable: {e}"))?;

        Ok(Embedder { model, tokenizer })
    }

    /// Embed each input text into a 384-dimensional, L2-normalized vector.
    ///
    /// Texts are tokenized individually with special tokens, truncated to 512
    /// tokens, then padded to the longest sequence in the batch. The batched
    /// tensors run through the encoder once; the CLS token of each row is
    /// pooled and normalized so cosine similarity reduces to a dot product.
    pub fn embed(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, String> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let mut id_rows: Vec<Vec<i64>> = Vec::with_capacity(texts.len());
        let mut mask_rows: Vec<Vec<i64>> = Vec::with_capacity(texts.len());
        let mut type_rows: Vec<Vec<i64>> = Vec::with_capacity(texts.len());

        for text in texts {
            let encoding = self
                .tokenizer
                .encode(*text, true)
                .map_err(|e| format!("Tokenization error: {e}"))?;
            let limit = MAX_SEQUENCE_LENGTH.min(encoding.get_ids().len());
            id_rows.push(to_i64(&encoding.get_ids()[..limit]));
            mask_rows.push(to_i64(&encoding.get_attention_mask()[..limit]));
            type_rows.push(to_i64(&encoding.get_type_ids()[..limit]));
        }

        let batch = texts.len();
        let seq_len = id_rows.iter().map(Vec::len).max().unwrap_or(0).max(1);

        let input_ids = build_tensor(&id_rows, batch, seq_len)?;
        let attention_mask = build_tensor(&mask_rows, batch, seq_len)?;
        let token_type_ids = build_tensor(&type_rows, batch, seq_len)?;

        let outputs = self
            .model
            .run(tvec![
                input_ids.into(),
                attention_mask.into(),
                token_type_ids.into()
            ])
            .map_err(|e| format!("Inference error: {e}"))?;

        let last_hidden_state = outputs[0]
            .to_array_view::<f32>()
            .map_err(|e| format!("Inference error: {e}"))?;

        let shape = last_hidden_state.shape();
        if shape.len() != 3 || shape[2] != EMBEDDING_DIM {
            return Err(format!(
                "Unexpected model output shape: got {:?}, expected [{batch}, {seq_len}, {EMBEDDING_DIM}]",
                shape
            ));
        }

        let mut rows: Vec<Vec<f32>> = Vec::with_capacity(batch);
        for item in 0..batch {
            let cls: Vec<f32> = (0..EMBEDDING_DIM)
                .map(|dim| last_hidden_state[[item, 0, dim]])
                .collect();
            rows.push(l2_normalize(&cls));
        }

        Ok(rows)
    }
}

/// Widen tokenizer `u32` ids into the `i64` the ONNX graph expects.
fn to_i64(values: &[u32]) -> Vec<i64> {
    values.iter().map(|&v| i64::from(v)).collect()
}

/// Pad each row to `seq_len` with zeros and stack into a `[batch, seq_len]`
/// `i64` tensor.
fn build_tensor(rows: &[Vec<i64>], batch: usize, seq_len: usize) -> Result<Tensor, String> {
    let mut flat: Vec<i64> = Vec::with_capacity(batch * seq_len);
    for row in rows {
        flat.extend_from_slice(row);
        flat.extend(std::iter::repeat_n(0i64, seq_len - row.len()));
    }
    let array = tract_ndarray::Array2::from_shape_vec((batch, seq_len), flat)
        .map_err(|e| format!("Failed to build input tensor: {e}"))?;
    Ok(array.into())
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
}
