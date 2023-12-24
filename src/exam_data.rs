use std::fmt::Display;

use crate::{class_data::TextureData, criteria::Criteria, sk_manager::SKManager};

pub type ExamRealizationResults = (Vec<usize>, usize);

#[derive(Debug)]
pub enum ExamResult {
    Found(usize, ExamRealizationResults),
    Unknown(ExamRealizationResults),
}

impl Display for ExamResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExamResult::Found(class, results) => write!(f, "{}: {:?}", class, results),
            ExamResult::Unknown(results) => write!(f, "Unknown: {:?}", results),
        }
    }
}

pub fn exam(
    reference_vectors: &[TextureData],
    exam_matrices: &[TextureData],
    criterias: &[Criteria],
    realizations: usize,
) -> Vec<ExamResult> {
    exam_matrices
        .iter()
        .map(|matrix| {
            let distances: Vec<Vec<f64>> = reference_vectors
                .iter()
                .enumerate()
                .map(|(i, center)| {
                    SKManager::distances_between(&matrix.bytes, &center.bytes)
                        .iter()
                        .map(|x| 1.0 - *x as f64 / criterias[i].min_radius())
                        .collect()
                })
                .collect();

            let results: Vec<usize> = distances
                .iter()
                .enumerate()
                .map(|(class, realizations)| {
                    realizations
                        .iter()
                        .enumerate()
                        .filter(|(i, &realization)| {
                            realization > 0.0
                                && !distances.iter().enumerate().any(
                                    |(other_class, other_realizations)| {
                                        other_class != class && other_realizations[*i] > 0.0
                                    },
                                )
                        })
                        .count()
                })
                .collect();

            let unknown: usize = realizations - results.iter().sum::<usize>();

            let statistics: ExamRealizationResults = (results, unknown);

            let classes: Vec<usize> = distances
                .iter()
                .enumerate()
                .filter(|(_, class)| class.iter().sum::<f64>() / realizations as f64 > 0.0)
                .map(|(i, _)| i)
                .collect();

            if classes.len() == 1 {
                ExamResult::Found(classes[0], statistics)
            } else {
                ExamResult::Unknown(statistics)
            }
        })
        .collect()
}
