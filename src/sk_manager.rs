use crate::{class_data::TextureData, sk::SK};

#[derive(Default)]
pub struct SKManager {
    pub sk: Vec<SK>,
    pub distances: Vec<Vec<u32>>,
    pub distances_to_realizations: Vec<Vec<Vec<u32>>>,
}

impl SKManager {
    pub fn new(matrices: &[TextureData], reference_vectors: &[TextureData]) -> SKManager {
        let distances: Vec<Vec<u32>> = (0..matrices.len())
            .map(|i| {
                (0..matrices.len())
                    .map(|j| {
                        Self::distance_between(
                            &reference_vectors[i].bytes,
                            &reference_vectors[j].bytes,
                        )
                    })
                    .collect()
            })
            .collect();

        let distances_to_realizations: Vec<Vec<Vec<u32>>> = (0..matrices.len())
            .map(|i| {
                (0..matrices.len())
                    .map(|j| {
                        Self::distances_between(&matrices[j].bytes, &reference_vectors[i].bytes)
                    })
                    .collect()
            })
            .collect();

        let sk = (0..matrices.len())
            .map(|i| {
                let distances_to_center = &distances_to_realizations[i][i];

                let closest = (0..matrices.len())
                    .filter(|&j| i != j)
                    .min_by_key(|&j| distances[i][j])
                    .unwrap_or_default();

                SK::new(
                    distances_to_center.to_vec(),
                    distances_to_realizations[i][closest].clone(),
                    distances_to_realizations[closest][closest].clone(),
                    distances_to_realizations[closest][i].clone(),
                    distances[i][closest],
                    closest,
                )
            })
            .collect();

        Self {
            sk,
            distances,
            distances_to_realizations,
        }
    }

    /// Returns the Hamming distance between two vectors.
    ///
    /// # Panics
    ///
    /// Panics if vectors have different lengths.
    pub fn distance_between(vector1: &[u8], vector2: &[u8]) -> u32 {
        assert_eq!(vector1.len(), vector2.len());
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
            .map(|realization| Self::distance_between(realization, center))
            .collect()
    }
}
