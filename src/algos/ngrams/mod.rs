/// This module contains the n-gram algorithms for bytes, chars, words, and Unicode
/// character categories.
///
pub mod binary;  // avoid plural form `byte` and `bytes` to avoid keyword conflicts
pub mod categories;
pub mod chars;
pub mod words;
