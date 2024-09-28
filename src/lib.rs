use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::*;
use serde_json::Value;

mod algos;
mod io;
mod parsers;

fn convert_values(py: Python, values: Vec<Value>) -> PyResult<Vec<PyObject>> {
    values
        .into_iter()
        .map(|value| value_to_pyobject(py, &value))
        .collect()
}

fn value_to_pyobject(py: Python, value: &Value) -> PyResult<PyObject> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.to_object(py)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.to_object(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.to_object(py))
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Invalid number",
                ))
            }
        }
        Value::String(s) => Ok(s.to_object(py)),
        Value::Array(arr) => {
            let pylist = PyList::empty_bound(py);
            for item in arr {
                pylist.append(value_to_pyobject(py, item)?)?;
            }
            Ok(pylist.to_object(py))
        }
        Value::Object(obj) => {
            let dict = PyDict::new_bound(py);
            for (key, val) in obj {
                dict.set_item(key, value_to_pyobject(py, val)?)?;
            }
            Ok(dict.to_object(py))
        }
    }
}

// pyo3 declarative style for defining python module
#[pymodule]
mod alea_preprocess {
    use super::*;

    // submodule for algos
    #[pymodule]
    mod algos {
        use super::*;

        // submodule for hashing
        #[pymodule]
        mod hashing {
            use super::*;

            // submodule for blake2
            #[pymodule]
            mod blake2 {
                use super::*;

                // function to hash bytes using blake2
                #[pyfunction]
                fn hash_bytes(bytes: &[u8]) -> PyResult<String> {
                    Ok(crate::algos::hashing::blake2::hash_bytes(bytes))
                }

                // function to hash string using blake2
                #[pyfunction]
                fn hash_str(s: &str) -> PyResult<String> {
                    Ok(crate::algos::hashing::blake2::hash_str(s))
                }

                // function to hash file using blake2
                #[pyfunction]
                fn hash_file(path: &str) -> PyResult<String> {
                    Ok(crate::algos::hashing::blake2::hash_file(path))
                }

                // function to hash gz file using blake2
                #[pyfunction]
                fn hash_gz_file(path: &str) -> PyResult<String> {
                    Ok(crate::algos::hashing::blake2::hash_gz_file(path))
                }
            }

            // submodule for blake3
            #[pymodule]
            mod blake3 {
                use super::*;

                // function to hash bytes using blake3
                #[pyfunction]
                fn hash_bytes(bytes: &[u8]) -> PyResult<String> {
                    Ok(crate::algos::hashing::blake3::hash_bytes(bytes))
                }

                // function to hash string using blake3
                #[pyfunction]
                fn hash_str(s: &str) -> PyResult<String> {
                    Ok(crate::algos::hashing::blake3::hash_str(s))
                }

                // function to hash file using blake3
                #[pyfunction]
                fn hash_file(path: &str) -> PyResult<String> {
                    Ok(crate::algos::hashing::blake3::hash_file(path))
                }

                // function to hash gz file using blake3
                #[pyfunction]
                fn hash_gz_file(path: &str) -> PyResult<String> {
                    Ok(crate::algos::hashing::blake3::hash_gz_file(path))
                }
            }

            // submodule for rolling
            #[pymodule]
            mod rolling {
                use super::*;

                // function to hash bytes using rolling
                #[pyfunction]
                fn hash_bytes(bytes: &[u8], window_size: usize, precision: u8) -> PyResult<String> {
                    Ok(crate::algos::hashing::rolling::hash_bytes(
                        bytes,
                        window_size,
                        precision,
                    ))
                }

                // function to hash string using rolling
                #[pyfunction]
                fn hash_str(s: &str, window_size: usize, precision: u8) -> PyResult<String> {
                    Ok(crate::algos::hashing::rolling::hash_str(
                        s,
                        window_size,
                        precision,
                    ))
                }

                // function to hash file using rolling
                #[pyfunction]
                fn hash_file(path: &str, window_size: usize, precision: u8) -> PyResult<String> {
                    Ok(crate::algos::hashing::rolling::hash_file(
                        path,
                        window_size,
                        precision,
                    ))
                }

                // function to hash gz file using rolling
                #[pyfunction]
                fn hash_gz_file(path: &str, window_size: usize, precision: u8) -> PyResult<String> {
                    Ok(crate::algos::hashing::rolling::hash_gz_file(
                        path,
                        window_size,
                        precision,
                    ))
                }
            }

            // submodule for ctph
            #[pymodule]
            mod ctph {
                use super::*;

                // function to hash bytes using ctph
                #[pyfunction]
                fn hash_bytes(
                    bytes: &[u8],
                    window_size: usize,
                    digest_size: usize,
                    precision: u8,
                ) -> PyResult<String> {
                    Ok(crate::algos::hashing::ctph::hash_bytes(
                        bytes,
                        window_size,
                        digest_size,
                        precision,
                    ))
                }

                // function to hash string using ctph
                #[pyfunction]
                fn hash_str(
                    s: &str,
                    window_size: usize,
                    digest_size: usize,
                    precision: u8,
                ) -> PyResult<String> {
                    Ok(crate::algos::hashing::ctph::hash_str(
                        s,
                        window_size,
                        digest_size,
                        precision,
                    ))
                }

                // function to hash file using ctph
                #[pyfunction]
                fn hash_file(
                    path: &str,
                    window_size: usize,
                    digest_size: usize,
                    precision: u8,
                ) -> PyResult<String> {
                    Ok(crate::algos::hashing::ctph::hash_file(
                        path,
                        window_size,
                        digest_size,
                        precision,
                    ))
                }

                // function to hash gz file using ctph
                #[pyfunction]
                fn hash_gz_file(
                    path: &str,
                    window_size: usize,
                    digest_size: usize,
                    precision: u8,
                ) -> PyResult<String> {
                    Ok(crate::algos::hashing::ctph::hash_gz_file(
                        path,
                        window_size,
                        digest_size,
                        precision,
                    ))
                }

                // function to compare ctph hashes
                #[pyfunction]
                fn compare(hash1: &str, hash2: &str) -> PyResult<f64> {
                    Ok(crate::algos::hashing::ctph::similarity(hash1, hash2))
                }
            }
        }

        #[pymodule]
        mod ngrams {
            use super::*;

            // submodule for bytes
            #[pymodule]
            mod byte {
                use super::*;

                #[pyfunction]
                fn transform(input_data: &[u8], n: usize) -> PyResult<Vec<Vec<u8>>> {
                    match n {
                        1 => Ok(crate::algos::ngrams::binary::transform_1(input_data)),
                        2 => Ok(crate::algos::ngrams::binary::transform_2(input_data)),
                        3 => Ok(crate::algos::ngrams::binary::transform_3(input_data)),
                        4 => Ok(crate::algos::ngrams::binary::transform_4(input_data)),
                        5 => Ok(crate::algos::ngrams::binary::transform_5(input_data)),
                        6 => Ok(crate::algos::ngrams::binary::transform_6(input_data)),
                        7 => Ok(crate::algos::ngrams::binary::transform_7(input_data)),
                        8 => Ok(crate::algos::ngrams::binary::transform_8(input_data)),
                        9 => Ok(crate::algos::ngrams::binary::transform_9(input_data)),
                        _ => {
                            return Err(pyo3::exceptions::PyValueError::new_err(
                                "ngrams of size greater than 9 are not supported",
                            ))
                        }
                    }
                }

                // function to extract bytegrams from bytes
                #[pyfunction]
                fn extract(py: Python, input_data: &[u8], n: usize) -> PyResult<Py<PyDict>> {
                    let raw_result = match n {
                        1 => crate::algos::ngrams::binary::extract_1(input_data),
                        2 => crate::algos::ngrams::binary::extract_2(input_data),
                        3 => crate::algos::ngrams::binary::extract_3(input_data),
                        4 => crate::algos::ngrams::binary::extract_4(input_data),
                        5 => crate::algos::ngrams::binary::extract_5(input_data),
                        _ => {
                            return Err(pyo3::exceptions::PyValueError::new_err(
                                "ngrams of size greater than 5 are not supported",
                            ))
                        }
                    };

                    let dict = PyDict::new_bound(py);
                    for (k, v) in raw_result {
                        let py_bytes = PyBytes::new_bound(py, &k);
                        dict.set_item(py_bytes, v)?;
                    }

                    Ok(dict.into())
                }
            }

            #[pymodule]
            mod chars {
                use super::*;

                #[pyfunction]
                fn transform(input_data: &str, n: usize) -> PyResult<Vec<Vec<char>>> {
                    match n {
                        1 => Ok(crate::algos::ngrams::chars::transform_1(input_data)),
                        2 => Ok(crate::algos::ngrams::chars::transform_2(input_data)),
                        3 => Ok(crate::algos::ngrams::chars::transform_3(input_data)),
                        4 => Ok(crate::algos::ngrams::chars::transform_4(input_data)),
                        5 => Ok(crate::algos::ngrams::chars::transform_5(input_data)),
                        6 => Ok(crate::algos::ngrams::chars::transform_6(input_data)),
                        7 => Ok(crate::algos::ngrams::chars::transform_7(input_data)),
                        8 => Ok(crate::algos::ngrams::chars::transform_8(input_data)),
                        9 => Ok(crate::algos::ngrams::chars::transform_9(input_data)),
                        _ => {
                            return Err(pyo3::exceptions::PyValueError::new_err(
                                "ngrams of size greater than 9 are not supported",
                            ))
                        }
                    }
                }

                // function to extract chargrams from string
                #[pyfunction]
                fn extract(py: Python, input_data: &str, n: usize) -> PyResult<Py<PyDict>> {
                    let raw_result = match n {
                        1 => crate::algos::ngrams::chars::extract_1(input_data),
                        2 => crate::algos::ngrams::chars::extract_2(input_data),
                        3 => crate::algos::ngrams::chars::extract_3(input_data),
                        4 => crate::algos::ngrams::chars::extract_4(input_data),
                        5 => crate::algos::ngrams::chars::extract_5(input_data),
                        _ => {
                            return Err(pyo3::exceptions::PyValueError::new_err(
                                "ngrams of size greater than 5 are not supported",
                            ))
                        }
                    };

                    let dict = PyDict::new_bound(py);
                    for (k, v) in raw_result {
                        let ngram_string: String = k.into_iter().collect();
                        let py_str = PyString::new_bound(py, &ngram_string);
                        dict.set_item(py_str, v)?;
                    }

                    Ok(dict.into())
                }
            }

            #[pymodule]
            mod categories {
                use super::*;
                use crate::algos::unicode::categories::{
                    category_group_to_string, category_to_string,
                };

                #[pyfunction]
                fn transform_category(
                    input_data: &str,
                    n: usize,
                ) -> PyResult<Vec<Vec<String>>> {
                    // get the category for each character, then map with category_to_string
                    let raw_result = match n {
                        1 => crate::algos::ngrams::categories::transform_category_1(input_data),
                        2 => crate::algos::ngrams::categories::transform_category_2(input_data),
                        3 => crate::algos::ngrams::categories::transform_category_3(input_data),
                        4 => crate::algos::ngrams::categories::transform_category_4(input_data),
                        5 => crate::algos::ngrams::categories::transform_category_5(input_data),
                        6 => crate::algos::ngrams::categories::transform_category_6(input_data),
                        7 => crate::algos::ngrams::categories::transform_category_7(input_data),
                        8 => crate::algos::ngrams::categories::transform_category_8(input_data),
                        9 => crate::algos::ngrams::categories::transform_category_9(input_data),
                        _ => {
                            return Err(PyValueError::new_err(
                                "ngrams of size greater than 9 are not supported",
                            ))
                        }
                    };

                    // map each Vec to a Vec<String> with category_to_string
                    Ok(raw_result
                        .iter()
                        .map(|cat| {
                            cat.iter()
                                .map(|&c| category_to_string(c).to_string())
                                .collect()
                        })
                        .collect())
                }

                #[pyfunction]
                fn transform_category_group(
                    input_data: &str,
                    n: usize,
                ) -> PyResult<Vec<Vec<String>>> {
                    // get the category group for each character, then map with category_group_to_string
                    let raw_result = match n {
                        1 => {
                            crate::algos::ngrams::categories::transform_category_group_1(input_data)
                        }
                        2 => {
                            crate::algos::ngrams::categories::transform_category_group_2(input_data)
                        }
                        3 => {
                            crate::algos::ngrams::categories::transform_category_group_3(input_data)
                        }
                        4 => {
                            crate::algos::ngrams::categories::transform_category_group_4(input_data)
                        }
                        5 => {
                            crate::algos::ngrams::categories::transform_category_group_5(input_data)
                        }
                        6 => {
                            crate::algos::ngrams::categories::transform_category_group_6(input_data)
                        }
                        7 => {
                            crate::algos::ngrams::categories::transform_category_group_7(input_data)
                        }
                        8 => {
                            crate::algos::ngrams::categories::transform_category_group_8(input_data)
                        }
                        9 => {
                            crate::algos::ngrams::categories::transform_category_group_9(input_data)
                        }
                        _ => {
                            return Err(PyValueError::new_err(
                                "ngrams of size greater than 9 are not supported",
                            ))
                        }
                    };

                    // map each Vec to a Vec<String> with category_group_to_string
                    Ok(raw_result
                        .iter()
                        .map(|group| {
                            group
                                .iter()
                                .map(|&g| category_group_to_string(g).to_string())
                                .collect()
                        })
                        .collect())
                }

                #[pyfunction]
                fn extract_category(
                    py: Python,
                    input_data: &str,
                    n: usize,
                ) -> PyResult<Py<PyDict>> {
                    let raw_result = match n {
                        1 => crate::algos::ngrams::categories::extract_category_1(input_data),
                        2 => crate::algos::ngrams::categories::extract_category_2(input_data),
                        3 => crate::algos::ngrams::categories::extract_category_3(input_data),
                        4 => crate::algos::ngrams::categories::extract_category_4(input_data),
                        5 => crate::algos::ngrams::categories::extract_category_5(input_data),
                        _ => {
                            return Err(PyValueError::new_err(
                                "ngrams of size greater than 5 are not supported",
                            ))
                        }
                    };

                    let dict = PyDict::new_bound(py);

                    for (category_ngram, frequency) in raw_result {
                        // get string representation with category_to_string, then combine into a tuple
                        let py_tuple = PyTuple::new_bound(
                            py,
                            category_ngram
                                .iter()
                                .map(|&cat| PyString::new_bound(py, category_to_string(cat))),
                        );
                        dict.set_item(py_tuple, frequency)?;
                    }

                    Ok(dict.into())
                }
                #[pyfunction]
                fn extract_category_group(
                    py: Python,
                    input_data: &str,
                    n: usize,
                ) -> PyResult<Py<PyDict>> {
                    let raw_result = match n {
                        1 => crate::algos::ngrams::categories::extract_category_group_1(input_data),
                        2 => crate::algos::ngrams::categories::extract_category_group_2(input_data),
                        3 => crate::algos::ngrams::categories::extract_category_group_3(input_data),
                        4 => crate::algos::ngrams::categories::extract_category_group_4(input_data),
                        5 => crate::algos::ngrams::categories::extract_category_group_5(input_data),
                        _ => {
                            return Err(PyValueError::new_err(
                                "ngrams of size greater than 5 are not supported",
                            ))
                        }
                    };

                    let dict = PyDict::new_bound(py);

                    for (category_group_ngram, frequency) in raw_result {
                        // get string representation with category_group_to_string, then combine into a tuple
                        let py_tuple = PyTuple::new_bound(
                            py,
                            category_group_ngram.iter().map(|&group| {
                                PyString::new_bound(py, category_group_to_string(group))
                            }),
                        );
                        dict.set_item(py_tuple, frequency)?;
                    }

                    Ok(dict.into())
                }
            }

            #[pymodule]
            mod words {
                use super::*;

                // transform
                #[pyfunction]
                fn transform(input_data: &str, n: usize) -> PyResult<Vec<Vec<String>>> {
                    match n {
                        1 => Ok(crate::algos::ngrams::words::transform_1(input_data)),
                        2 => Ok(crate::algos::ngrams::words::transform_2(input_data)),
                        3 => Ok(crate::algos::ngrams::words::transform_3(input_data)),
                        4 => Ok(crate::algos::ngrams::words::transform_4(input_data)),
                        5 => Ok(crate::algos::ngrams::words::transform_5(input_data)),
                        6 => Ok(crate::algos::ngrams::words::transform_6(input_data)),
                        7 => Ok(crate::algos::ngrams::words::transform_7(input_data)),
                        8 => Ok(crate::algos::ngrams::words::transform_8(input_data)),
                        9 => Ok(crate::algos::ngrams::words::transform_9(input_data)),
                        _ => Err(PyValueError::new_err(
                            "ngrams of size greater than 9 are not supported",
                        )),
                    }
                }

                // function to extract wordgrams from string
                #[pyfunction]
                fn extract(py: Python, input_data: &str, n: usize) -> PyResult<Py<PyDict>> {
                    let raw_result = match n {
                        1 => crate::algos::ngrams::words::extract_1(input_data),
                        2 => crate::algos::ngrams::words::extract_2(input_data),
                        3 => crate::algos::ngrams::words::extract_3(input_data),
                        4 => crate::algos::ngrams::words::extract_4(input_data),
                        5 => crate::algos::ngrams::words::extract_5(input_data),
                        _ => {
                            return Err(PyValueError::new_err(
                                "ngrams of size greater than 5 are not supported",
                            ))
                        }
                    };

                    let dict = PyDict::new_bound(py);
                    for (k, v) in raw_result {
                        // return as a tuple
                        let ngram_tuple =
                            PyTuple::new_bound(py, k.iter().map(|s| PyString::new_bound(py, s)));
                        dict.set_item(ngram_tuple, v)?;
                    }

                    Ok(dict.into())
                }
            }
        }

        #[pymodule]
        mod segmentation {
            use super::*;

            // submodule for sentence
            #[pymodule(submodule)]
            mod sentence {
                use super::*;

                // function to segment text into sentences
                #[pyfunction]
                fn get_abbreviations_simple(input_text: &str) -> Vec<String> {
                    crate::algos::segmentation::sentence::get_abbreviations_simple(input_text)
                }

                #[pyfunction]
                fn get_abbreviations_regex(input_text: &str) -> Vec<String> {
                    crate::algos::segmentation::sentence::get_abbreviations_regex(input_text)
                }
            }
        }

        #[pymodule]
        mod similarity {
            use super::*;

            // submodule for strings
            #[pymodule]
            mod strings {
                use super::*;

                // function to compute hamming distance between two strings
                #[pyfunction]
                fn hamming_distance(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::hamming_distance(a, b)
                }

                // function to compute hamming similarity between two strings
                #[pyfunction]
                fn hamming_similarity(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::hamming_similarity(a, b)
                }

                // function to compute levenshtein distance between two strings
                #[pyfunction]
                fn levenshtein_distance(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::levenshtein_distance(a, b)
                }

                // function to compute normalized levenshtein distance between two strings
                #[pyfunction]
                fn normalized_levenshtein_distance(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::normalized_levenshtein_distance(a, b)
                }

                // function to compute levenshtein similarity between two strings
                #[pyfunction]
                fn levenshtein_similarity(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::levenshtein_similarity(a, b)
                }

                // function to compute osa distance between two strings
                #[pyfunction]
                fn osa_distance(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::osa_distance(a, b)
                }

                // function to compute osa similarity between two strings
                #[pyfunction]
                fn osa_similarity(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::osa_similarity(a, b)
                }

                // function to compute damerau levenshtein distance between two strings
                #[pyfunction]
                fn damerau_levenshtein_distance(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::damerau_levenshtein_distance(a, b)
                }

                // function to compute damerau levenshtein similarity between two strings
                #[pyfunction]
                fn damerau_levenshtein_similarity(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::damerau_levenshtein_similarity(a, b)
                }

                // function to compute normalized damerau levenshtein distance between two strings
                #[pyfunction]
                fn normalized_damerau_levenshtein_distance(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::normalized_damerau_levenshtein_distance(a, b)
                }

                // function to compute jaro distance between two strings
                #[pyfunction]
                fn jaro_distance(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::jaro_distance(a, b)
                }

                // function to compute jaro similarity between two strings
                #[pyfunction]
                fn jaro_similarity(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::jaro_similarity(a, b)
                }

                // function to compute jaro winkler distance between two strings
                #[pyfunction]
                fn jaro_winkler_distance(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::jaro_winkler_distance(a, b)
                }

                // function to compute jaro winkler similarity between two strings
                #[pyfunction]
                fn jaro_winkler_similarity(a: &str, b: &str) -> f64 {
                    crate::algos::similarity::strings::jaro_winkler_similarity(a, b)
                }
            }
        }

        #[pymodule]
        mod unicode {
            use super::*;

            #[pymodule(submodule)]
            mod categories {
                use super::*;

                // function to get unicode category for a character
                #[pyfunction]
                fn char_to_category(ch: char) -> &'static str {
                    crate::algos::unicode::categories::category_to_string(
                        crate::algos::unicode::categories::char_to_category(ch),
                    )
                }

                // function to get unicode category group for a character
                #[pyfunction]
                fn char_to_category_group(ch: char) -> &'static str {
                    crate::algos::unicode::categories::category_group_to_string(
                        crate::algos::unicode::categories::char_to_category_group(ch),
                    )
                }

                // function to get unicode category for a string
                #[pyfunction]
                fn to_category_vector(input_data: &str) -> Vec<String> {
                    crate::algos::unicode::categories::to_category_vector(input_data)
                        .iter()
                        .map(|cat| {
                            crate::algos::unicode::categories::category_to_string(*cat).to_string()
                        })
                        .collect()
                }

                // function to get unicode category group for a string
                #[pyfunction]
                fn to_category_group_vector(input_data: &str) -> Vec<String> {
                    crate::algos::unicode::categories::to_category_group_vector(input_data)
                        .iter()
                        .map(|group| {
                            crate::algos::unicode::categories::category_group_to_string(*group)
                                .to_string()
                        })
                        .collect()
                }
            }

            #[pymodule]
            mod normalizations {
                use super::*;

                // function to normalize string using NFD
                #[pyfunction]
                fn nfd_str(buffer: &str) -> String {
                    crate::algos::unicode::normalizations::nfd_str(buffer)
                }

                // function to normalize string using NFKD
                #[pyfunction]
                fn nfkd_str(buffer: &str) -> String {
                    crate::algos::unicode::normalizations::nfkd_str(buffer)
                }

                // function to normalize string using NFC
                #[pyfunction]
                fn nfc_str(buffer: &str) -> String {
                    crate::algos::unicode::normalizations::nfc_str(buffer)
                }

                // function to normalize string using NFKC
                #[pyfunction]
                fn nfkc_str(buffer: &str) -> String {
                    crate::algos::unicode::normalizations::nfkc_str(buffer)
                }
            }

            #[pymodule(submodule)]
            mod segmentations {
                use super::*;

                #[pyfunction]
                fn get_segment_indices_from_breakpoints(
                    buffer: &str,
                    breakpoints: Vec<u64>,
                ) -> Vec<(u64, u64, String)> {
                    crate::algos::unicode::segmentations::get_segment_indices_from_breakpoints(
                        buffer,
                        // convert back to usize
                        &breakpoints.iter().map(|x| *x as usize).collect(),
                    )
                    .iter()
                    .map(|(start, end, segment)| (*start as u64, *end as u64, segment.to_string()))
                    .collect()
                }

                #[pyfunction]
                fn get_segments_from_breakpoints(
                    buffer: &str,
                    breakpoints: Vec<u64>,
                ) -> Vec<String> {
                    // crate::algos::unicode::segmentations::get_segments_from_breakpoints(buffer, breakpoints)
                    crate::algos::unicode::segmentations::get_segments_from_breakpoints(
                        buffer,
                        // convert back to usize
                        &breakpoints.iter().map(|x| *x as usize).collect(),
                    )
                }

                #[pyfunction]
                fn get_grapheme_indices(buffer: &str) -> Vec<(u64, u64, String)> {
                    crate::algos::unicode::segmentations::get_grapheme_indices(buffer)
                        .iter()
                        .map(|(start, end, segment)| {
                            (*start as u64, *end as u64, segment.to_string())
                        })
                        .collect()
                }

                #[pyfunction]
                fn get_word_indices(buffer: &str) -> Vec<(u64, u64, String)> {
                    crate::algos::unicode::segmentations::get_word_indices(buffer)
                        .iter()
                        .map(|(start, end, segment)| {
                            (*start as u64, *end as u64, segment.to_string())
                        })
                        .collect()
                }

                #[pyfunction]
                fn get_sentence_indices(buffer: &str) -> Vec<(u64, u64, String)> {
                    crate::algos::unicode::segmentations::get_sentence_indices(buffer)
                        .iter()
                        .map(|(start, end, segment)| {
                            (*start as u64, *end as u64, segment.to_string())
                        })
                        .collect()
                }

                #[pyfunction]
                fn get_line_indices(buffer: &str) -> Vec<(u64, u64, String)> {
                    crate::algos::unicode::segmentations::get_line_indices(buffer)
                        .iter()
                        .map(|(start, end, segment)| {
                            (*start as u64, *end as u64, segment.to_string())
                        })
                        .collect()
                }

                // function to segment string into grapheme clusters
                #[pyfunction]
                fn segment_graphemes(buffer: &str) -> Vec<String> {
                    crate::algos::unicode::segmentations::segment_graphemes(buffer)
                }

                // function to segment string into words
                #[pyfunction]
                fn segment_words(buffer: &str) -> Vec<String> {
                    crate::algos::unicode::segmentations::segment_words(buffer)
                }

                // function to segment string into sentences
                #[pyfunction]
                fn segment_sentences(buffer: &str) -> Vec<String> {
                    crate::algos::unicode::segmentations::segment_sentences(buffer)
                }

                // function to segment string into lines
                #[pyfunction]
                fn segment_lines(buffer: &str) -> Vec<String> {
                    crate::algos::unicode::segmentations::segment_lines(buffer)
                }
            }
        }

        #[pymodule]
        mod splitting {
            use super::*;

            #[pymodule]
            mod simple {
                use super::*;

                #[pyfunction]
                fn split_str(
                    buffer: &str,
                    min_size: u64,
                    max_size: u64,
                    split_patterns: Vec<String>,
                ) -> Vec<String> {
                    crate::algos::splitting::simple::split_str(
                        buffer,
                        min_size as usize,
                        max_size as usize,
                        split_patterns,
                    )
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
                }

                #[pyfunction]
                fn split_tokens(
                    tokens: Vec<u32>,
                    min_size: u64,
                    max_size: u64,
                    split_patterns: Vec<u32>,
                ) -> Vec<Vec<u32>> {
                    crate::algos::splitting::simple::split_tokens(
                        &tokens,
                        min_size as usize,
                        max_size as usize,
                        split_patterns,
                    )
                    .iter()
                    .map(|t| t.to_vec())
                    .collect()
                }

                #[pyfunction]
                fn split_str_list(
                    texts: Vec<String>,
                    min_size: u64,
                    max_size: u64,
                    split_patterns: Vec<String>,
                ) -> Vec<Vec<String>> {
                    crate::algos::splitting::simple::split_str_list(
                        &texts,
                        min_size as usize,
                        max_size as usize,
                        split_patterns,
                    )
                    .iter()
                    .map(|t| t.to_vec())
                    .collect()
                }

                #[pyfunction]
                fn split_tokens_list(
                    token_lists: Vec<Vec<u32>>,
                    min_size: u64,
                    max_size: u64,
                    split_patterns: Vec<u32>,
                ) -> Vec<Vec<Vec<u32>>> {
                    crate::algos::splitting::simple::split_token_list(
                        &token_lists,
                        min_size as usize,
                        max_size as usize,
                        split_patterns,
                    )
                    .iter()
                    .map(|t| t.to_vec())
                    .collect()
                }
            }
        }

        #[pymodule]
        mod tokenizers {
            use super::*;

            #[pyfunction]
            fn encode_str(tokenizer: &str, text: &str) -> Vec<u32> {
                crate::algos::tokenizers::tokenizers::encode_str(tokenizer, text)
            }

            #[pyfunction]
            fn decode_str(tokenizer: &str, ids: Vec<u32>) -> String {
                crate::algos::tokenizers::tokenizers::decode_str(tokenizer, ids)
            }

            #[pyfunction]
            fn encode_str_list(tokenizer: &str, texts: Vec<String>) -> Vec<Vec<u32>> {
                crate::algos::tokenizers::tokenizers::encode_str_list(tokenizer, texts)
            }

            #[pyfunction]
            fn decode_str_list(tokenizer: &str, ids: Vec<Vec<u32>>) -> Vec<String> {
                crate::algos::tokenizers::tokenizers::decode_str_list(tokenizer, ids)
            }
        }
    }

    #[pymodule]
    mod io {
        use super::*;

        #[pymodule(submodule)]
        mod fs {
            use super::*;

            #[pymodule(submodule)]
            mod directories {
                use super::*;

                #[pyfunction]
                pub fn get_files(path: &str) -> Vec<String> {
                    crate::io::fs::directories::get_files(path)
                }

                #[pyfunction]
                pub fn get_directories(path: &str) -> Vec<String> {
                    crate::io::fs::directories::get_directories(path)
                }

                #[pyfunction]
                pub fn get_entries(path: &str) -> Vec<String> {
                    crate::io::fs::directories::get_entries(path)
                }
            }

            #[pymodule(submodule)]
            mod file_info {
                use super::*;

                #[pymodule_export]
                use crate::io::fs::file_info::FileInfo;

                #[pyfunction]
                pub fn get_file_info_from_buffer(buffer: &[u8]) -> FileInfo {
                    crate::io::fs::file_info::get_file_info_from_buffer(buffer)
                }

                #[pyfunction]
                pub fn get_file_info_from_file(path: &str) -> FileInfo {
                    crate::io::fs::file_info::get_file_info_from_file(path)
                }
            }
        }
    }

    #[pymodule]
    mod parsers {
        use super::*;

        #[pymodule]
        mod html {
            use super::*;

            #[pymodule(submodule)]
            mod conversion {
                use super::*;
                use crate::parsers::html::conversion::{HtmlToMarkdownParser, ParserConfig};

                #[pyfunction]
                pub fn extract_buffer_markdown(
                    buffer: &str,
                    output_links: bool,
                    output_images: bool,
                ) -> String {
                    // crate::parsers::pdf::conversion::extract_buffer_text(buffer)
                    let parser = HtmlToMarkdownParser::new(
                        ParserConfig::new(None, output_links, output_images),
                        buffer,
                    );
                    parser.to_markdown()
                }
            }
        }

        #[pymodule]
        mod pdf {
            use super::*;

            #[pymodule]
            mod detection {
                use super::*;
                use crate::parsers::pdf::detection::DocumentType;

                #[pyclass(eq, eq_int)]
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub enum PyDocumentType {
                    Text = DocumentType::Text as isize,
                    ImagePreOCR = DocumentType::ImagePreOCR as isize,
                    ImagePostOCR = DocumentType::ImagePostOCR as isize,
                    Mixed = DocumentType::Mixed as isize,
                    Unknown = DocumentType::Unknown as isize,
                }

                #[pyfunction]
                pub fn detect_buffer_type(buffer: &[u8]) -> PyDocumentType {
                    match crate::parsers::pdf::detection::detect_buffer_type(buffer) {
                        DocumentType::Text => PyDocumentType::Text,
                        DocumentType::ImagePreOCR => PyDocumentType::ImagePreOCR,
                        DocumentType::ImagePostOCR => PyDocumentType::ImagePostOCR,
                        DocumentType::Mixed => PyDocumentType::Mixed,
                        DocumentType::Unknown => PyDocumentType::Unknown,
                    }
                }

                #[pyfunction]
                pub fn detect_file_type(file_path: &str) -> PyDocumentType {
                    match crate::parsers::pdf::detection::detect_file_type(file_path) {
                        DocumentType::Text => PyDocumentType::Text,
                        DocumentType::ImagePreOCR => PyDocumentType::ImagePreOCR,
                        DocumentType::ImagePostOCR => PyDocumentType::ImagePostOCR,
                        DocumentType::Mixed => PyDocumentType::Mixed,
                        DocumentType::Unknown => PyDocumentType::Unknown,
                    }
                }
            }

            #[pymodule]
            mod conversion {
                use super::*;

                #[pyfunction]
                pub fn extract_buffer_text(buffer: &[u8]) -> String {
                    crate::parsers::pdf::conversion::extract_buffer_text(buffer)
                }

                #[pyfunction]
                pub fn extract_file_text(file_path: &str) -> String {
                    crate::parsers::pdf::conversion::extract_file_text(file_path)
                }

                #[pyfunction]
                pub fn extract_buffer_markdown(buffer: &[u8]) -> String {
                    crate::parsers::pdf::conversion::extract_buffer_markdown(buffer)
                }

                #[pyfunction]
                pub fn extract_file_markdown(file_path: &str) -> String {
                    crate::parsers::pdf::conversion::extract_file_markdown(file_path)
                }
            }
        }

        #[pymodule]
        mod tika {
            use super::*;

            #[pymodule(submodule)]
            mod client {
                use super::*;

                //#[pymodule_export]
                use crate::parsers::tika::client::SyncTikaClient;

                #[pyfunction]
                pub fn get_recursive_metadata_buffer(
                    py: Python,
                    buffer: &[u8],
                    server_url: &str,
                ) -> PyResult<Vec<PyObject>> {
                    let client = SyncTikaClient::new(server_url);
                    convert_values(py, client.get_recursive_metadata_buffer(buffer))
                }

                #[pyfunction]
                pub fn get_recursive_metadata_file(
                    py: Python,
                    file_path: &str,
                    server_url: &str,
                ) -> PyResult<Vec<PyObject>> {
                    let client = SyncTikaClient::new(server_url);
                    convert_values(py, client.get_recursive_metadata_file(file_path))
                }

                #[pyfunction]
                pub fn get_recursive_content_html_buffer(
                    buffer: &[u8],
                    server_url: &str,
                ) -> PyResult<Vec<String>> {
                    let client = SyncTikaClient::new(server_url);
                    Ok(client.get_recursive_content_html_buffer(buffer))
                }

                #[pyfunction]
                pub fn get_recursive_content_html_file(
                    file_path: &str,
                    server_url: &str,
                ) -> PyResult<Vec<String>> {
                    let client = SyncTikaClient::new(server_url);
                    Ok(client.get_recursive_content_html_file(file_path))
                }

                #[pyfunction]
                pub fn get_recursive_content_markdown_buffer(
                    buffer: &[u8],
                    server_url: &str,
                    output_links: bool,
                    output_images: bool,
                ) -> PyResult<Vec<String>> {
                    let client = SyncTikaClient::new(server_url);
                    Ok(client.get_recursive_content_markdown_buffer(
                        buffer,
                        output_links,
                        output_images,
                    ))
                }

                #[pyfunction]
                pub fn get_recursive_content_markdown_file(
                    file_path: &str,
                    server_url: &str,
                    output_links: bool,
                    output_images: bool,
                ) -> PyResult<Vec<String>> {
                    let client = SyncTikaClient::new(server_url);
                    Ok(client.get_recursive_content_markdown_file(
                        file_path,
                        output_links,
                        output_images,
                    ))
                }
            }
        }
    }

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add(
            "__VERSION__",
            option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
        )?;
        Ok(())
    }
}
