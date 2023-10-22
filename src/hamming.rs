/// Returns the Hamming distance between two vectors.
///
/// # Panics
///
/// Panics if vectors have different lengths.
pub fn distance_between(vector1: &[u8], vector2: &[u8]) -> u32 {
    assert!(vector1.len() == vector2.len());
    let mut sum: u32 = 0;
    for i in 0..vector1.len() {
        if vector2[i] != vector1[i] {
            sum += 1;
        }
    }

    sum
}

/// Returns the vector of Hamming distances between each realization and vector.
pub fn distances_between(realizations: &[u8], center: &[u8]) -> Vec<u32> {
    realizations
        .chunks(center.len())
        .map(|realization| distance_between(realization, center))
        .collect()
}
