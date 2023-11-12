use super::painter::SK;

#[derive(Debug, Default)]
pub struct Criteria {
    pub self_realizations: Vec<usize>,
    pub closest_realizations: Vec<usize>,
    pub closest_realizations_in_closest: Vec<usize>,
    pub self_realizations_in_closest: Vec<usize>,
    pub self_characteristics: Vec<Characteristics>,
    // pub closest_characteristics: Vec<Characteristics>,
    pub kullback_criteria: Vec<f32>,
    pub sheffer_criteria: Vec<f32>,
}

#[derive(Debug, Default)]
pub struct Characteristics {
    pub d1: f32,
    pub beta: f32,
    pub alpha: f32,
    pub d2: f32,
}

impl Criteria {
    pub fn new(sk: &SK, number_of_realizations: usize) -> Self {
        let max_radius = Self::calculate_max_radius(sk);

        let self_realizations =
            Self::calculate_number_of_self_realizations(&sk.distances_to_self, max_radius);
        let closest_realizations =
            Self::calculate_number_of_self_realizations(&sk.distances_to_closest, max_radius);
        let closest_realizations_in_closest = Self::calculate_number_of_self_realizations(
            &sk.distances_from_closest_to_itself,
            max_radius,
        );
        let self_realizations_in_closest =
            Self::calculate_number_of_self_realizations(&sk.distances_from_closest, max_radius);

        let self_characteristics = Self::calculate_characteristics(
            &self_realizations,
            &closest_realizations,
            number_of_realizations,
        );

        // let closest_characteristics = Self::calculate_characteristics(
        //     &closest_realizations_in_closest,
        //     &self_realizations_in_closest,
        //     number_of_realizations,
        // );

        let kullback_criteria = Self::kullback_criteria(&self_characteristics);
        let sheffer_criteria = Self::sheffer_criteria(&self_characteristics);

        Self {
            self_realizations,
            closest_realizations,
            closest_realizations_in_closest,
            self_realizations_in_closest,
            self_characteristics,
            // closest_characteristics,
            kullback_criteria,
            sheffer_criteria,
        }
    }

    fn kullback_criteria(characteristics: &[Characteristics]) -> Vec<f32> {
        characteristics
            .iter()
            .map(|c| {
                ((2.0 - (c.alpha + c.beta)) / (c.alpha + c.beta)).log2()
                    * (1.0 - (c.alpha + c.beta))
            })
            .collect()
    }

    fn sheffer_criteria(characteristics: &[Characteristics]) -> Vec<f32> {
        characteristics
            .iter()
            .map(|c| {
                let a = c.alpha.max(0.000001);
                let b = c.beta.max(0.000001);
                let d1 = c.d1.max(0.000001);
                let d2 = c.d2.max(0.000001);

                1.0 + 0.5
                    * ((a / (a + d2)) * (a / (a + d2)).log2()
                        + (d1 / (d1 + b)) * (d1 / (d1 + b)).log2()
                        + (b / (d1 + b)) * (b / (d1 + b)).log2()
                        + (d2 / (a + d2)) * (d2 / (a + d2)).log2())
            })
            .collect()
    }

    fn calculate_characteristics(
        realizations: &[usize],
        closest_realizations: &[usize],
        number_of_realizations: usize,
    ) -> Vec<Characteristics> {
        let mut characteristics: Vec<Characteristics> = Vec::new();

        for i in 0..realizations.len() {
            let d1 = realizations[i] as f32 / number_of_realizations as f32;
            let alpha = 1.0 - d1;
            let beta = closest_realizations[i] as f32 / number_of_realizations as f32;
            let d2 = 1.0 - beta;

            characteristics.push(Characteristics {
                d1,
                alpha,
                beta,
                d2,
            });
        }

        characteristics
    }

    fn calculate_max_radius(sk: &SK) -> u32 {
        let self_max_distance = sk.distances_to_self.iter().max().unwrap_or(&0);
        let closest_max_distance = sk.distances_to_closest.iter().max().unwrap_or(&0);

        *self_max_distance.max(closest_max_distance)
    }

    fn calculate_number_of_self_realizations(
        distances_to_realizations: &[u32],
        max_radius: u32,
    ) -> Vec<usize> {
        let mut number_of_realizations: Vec<usize> = Vec::new();

        for i in 0..max_radius {
            number_of_realizations.push(
                distances_to_realizations
                    .iter()
                    .filter(|d| d <= &&i)
                    .count(),
            );
        }

        number_of_realizations
    }
}
