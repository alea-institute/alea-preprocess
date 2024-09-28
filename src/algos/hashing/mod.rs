/// This module contains the hashing algorithms used in the project.
///
/// The following algorithms are supported:
///
/// - Blake2
/// - Blake3
/// - Rolling Hash
/// - Context-Triggered Piecewise Hashing (CTPH)
pub mod blake2;
pub mod blake3;
pub mod ctph;
pub mod rolling;
