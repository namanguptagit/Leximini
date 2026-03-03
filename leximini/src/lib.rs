use pyo3::prelude::*;
use pyo3::types::{PyModule, PySet};
use pyo3::Bound;
use std::collections::HashMap;

#[pyclass]
pub struct Tokenizer {
    vocab: HashMap<u32, Vec<u8>>,
    merges: HashMap<(u32, u32), u32>,
}

#[pymethods]
impl Tokenizer {
    #[new]
    fn new() -> Self {
        let mut vocab = HashMap::new();
        for i in 0..=255 {
            vocab.insert(i as u32, vec![i as u8]);
        }
        Tokenizer {
            vocab,
            merges: HashMap::new(),
        }
    }

    #[pyo3(signature = (text, vocab_size))]
    fn train(&mut self, text: &str, vocab_size: usize) -> PyResult<()> {
        if vocab_size < 256 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "vocab_size must be >= 256",
            ));
        }

        let num_merges = vocab_size - 256;
        let mut tokens: Vec<u32> = text.bytes().map(|b| b as u32).collect();

        for i in 0..num_merges {
            let mut stats: HashMap<(u32, u32), usize> = HashMap::new();
            for window in tokens.windows(2) {
                let pair = (window[0], window[1]);
                *stats.entry(pair).or_insert(0) += 1;
            }

            if stats.is_empty() {
                break;
            }

            let best_pair = stats.into_iter().max_by_key(|&(_, count)| count).unwrap().0;

            let new_token_id = 256 + i as u32;

            self.merges.insert(best_pair, new_token_id);

            let mut new_token_bytes = self.vocab.get(&best_pair.0).unwrap().clone();
            new_token_bytes.extend(self.vocab.get(&best_pair.1).unwrap().iter());
            self.vocab.insert(new_token_id, new_token_bytes);

            tokens = Self::apply_merge(&tokens, best_pair, new_token_id);
        }

        Ok(())
    }

    #[allow(unused_variables)]
    #[pyo3(signature = (text, allowed_special=None))]
    fn encode(&self, text: &str, allowed_special: Option<&Bound<'_, PySet>>) -> PyResult<Vec<u32>> {
        let mut tokens: Vec<u32> = text.bytes().map(|b| b as u32).collect();

        loop {
            let mut stats: HashMap<(u32, u32), usize> = HashMap::new();
            for window in tokens.windows(2) {
                let pair = (window[0], window[1]);
                *stats.entry(pair).or_insert(0) += 1;
            }

            if stats.is_empty() {
                break;
            }

            let mut best_pair_opt = None;
            let mut lowest_merge_id = u32::MAX;

            for &pair in stats.keys() {
                if let Some(&merge_id) = self.merges.get(&pair) {
                    if merge_id < lowest_merge_id {
                        lowest_merge_id = merge_id;
                        best_pair_opt = Some(pair);
                    }
                }
            }

            if let Some(best_pair) = best_pair_opt {
                tokens = Self::apply_merge(&tokens, best_pair, lowest_merge_id);
            } else {
                break;
            }
        }

        Ok(tokens)
    }

    fn decode(&self, tokens: Vec<u32>) -> PyResult<String> {
        let mut bytes: Vec<u8> = Vec::new();
        for t in tokens {
            if let Some(token_bytes) = self.vocab.get(&t) {
                bytes.extend(token_bytes);
            } else {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Token {} not in vocabulary",
                    t
                )));
            }
        }

        match String::from_utf8(bytes) {
            Ok(s) => Ok(s),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Invalid UTF-8 sequence: {}",
                e
            ))),
        }
    }
}

impl Tokenizer {
    fn apply_merge(tokens: &[u32], pair: (u32, u32), new_token_id: u32) -> Vec<u32> {
        let mut new_tokens = Vec::with_capacity(tokens.len());
        let mut i = 0;

        while i < tokens.len() {
            if i < tokens.len() - 1 && tokens[i] == pair.0 && tokens[i + 1] == pair.1 {
                new_tokens.push(new_token_id);
                i += 2;
            } else {
                new_tokens.push(tokens[i]);
                i += 1;
            }
        }
        new_tokens
    }
}

#[pyfunction]
fn get_encoding(_name: &str) -> PyResult<Tokenizer> {
    Ok(Tokenizer::new())
}

#[pymodule]
fn leximini(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Tokenizer>()?;
    m.add_function(wrap_pyfunction!(get_encoding, m)?)?;
    Ok(())
}
