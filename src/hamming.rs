use crate::{class_data::TextureData, sk::SK};

#[derive(Default)]
pub struct SKManager {
    pub sk: Vec<SK>,
}

impl SKManager {
    pub fn new(matrices: &[TextureData], reference_vectors: &[TextureData]) -> SKManager {
        let sk = matrices
            .iter()
            .enumerate()
            .map(|(index, matrix)| {
                let center = &reference_vectors[index].bytes;

                let distances_to_center = Self::distances_between(&matrix.bytes, center);
                let (distances_to_closest, distance, closest) = reference_vectors
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i != &index)
                    .map(|(i, reference_vector)| {
                        (i, Self::distance_between(&reference_vector.bytes, center))
                    })
                    .min_by_key(|(_, distance)| *distance)
                    .map_or_else(
                        || (Vec::new(), 0, 0),
                        |(closest, distance)| {
                            (
                                Self::distances_between(&matrices[closest].bytes, center),
                                distance,
                                closest,
                            )
                        },
                    );

                let center = &reference_vectors[closest].bytes;

                let distances_from_closest_to_itself =
                    Self::distances_between(&matrices[closest].bytes, center);
                let distances_from_closest = Self::distances_between(&matrix.bytes, center);

                SK::new(
                    distances_to_center,
                    distances_to_closest,
                    distances_from_closest_to_itself,
                    distances_from_closest,
                    distance,
                    closest,
                )
            })
            .collect();

        Self { sk }
    }

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
            .map(|realization| Self::distance_between(realization, center))
            .collect()
    }
}
