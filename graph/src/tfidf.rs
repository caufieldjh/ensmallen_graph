use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use super::*;

/// Return vocabulary and TFIDF matrix of given documents.
///
/// # Arguments
/// * `documents`: &[Vec<String>] - The documents to parse
/// * `k1`: Option<f64> - The default parameter for k1, tipically between 1.2 and 2.0.
/// * `b`: Option<f64> - The default parameter for b, tipically equal to 0.75.
/// * `verbose`: Option<bool> - Whether to show a loading bar.
///
pub fn okapi_bm25_tfidf(
    documents: &[Vec<String>],
    k1: Option<f64>,
    b: Option<f64>,
    verbose: Option<bool>,
) -> Result<(Vocabulary<usize>, Vec<Vec<f64>>), String> {
    if documents.is_empty() {
        return Err("The given documents set is empty!".to_string());
    }
    let verbose = verbose.unwrap_or(true);
    let k1 = k1.unwrap_or(1.5);
    let b = b.unwrap_or(0.75);
    let number_of_documents = documents.len();
    let mut total_documents_length = 0;
    let mut vocabulary = Vocabulary::<usize>::default();
    let mut word_counts: Vec<usize> = Vec::new();
    let pb = get_loading_bar(verbose, "Building vocabulary", number_of_documents);
    for document in documents.iter().progress_with(pb) {
        total_documents_length += document.len();
        for word in document.iter() {
            let (index, not_new) = vocabulary.insert(word)?;
            if not_new {
                word_counts[index] += 1;
            } else {
                word_counts.push(1);
            }
        }
    }
    // Build the reverse mapping
    vocabulary.build_reverse_mapping()?;
    // Computing vocabulary size
    let vocabulary_size = word_counts.len();
    // Computing average document size
    let average_document_len = total_documents_length as f64 / number_of_documents as f64;
    // Computing inverse document frequencies
    let inverse_document_frequencies = word_counts
        .into_par_iter()
        .map(|counts| {
            ((number_of_documents as f64 - counts as f64 + 0.5) / (counts as f64 + 0.5)).ln_1p()
        })
        .collect::<Vec<f64>>();
    // Creating loading bar for actually computing TFIDF
    let pb = get_loading_bar(verbose, "Building TFIDF", number_of_documents);
    // Computing TFIDF of provided words and documents
    let tfidf = documents
        .par_iter()
        .progress_with(pb)
        .map(|document| {
            let document_len = document.len() as f64;
            let mut frequencies = vec![0.0; vocabulary_size];
            document.iter().for_each(|word| {
                frequencies[*vocabulary.get(word.as_ref()).unwrap()] += 1.0;
            });
            frequencies
                .iter_mut()
                .enumerate()
                .for_each(|(word_id, frequency)| {
                    *frequency = inverse_document_frequencies[word_id] * (*frequency * (k1 + 1.0))
                        / (*frequency + k1 * (1.0 - b + b * document_len / average_document_len));
                });
            frequencies
        })
        .collect::<Vec<Vec<f64>>>();
    Ok((vocabulary, tfidf))
}
