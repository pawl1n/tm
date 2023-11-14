use super::painter::SK;

#[derive(Debug, Default)]
pub struct Criteria {
    pub self_realizations: Vec<usize>,
    pub closest_realizations: Vec<usize>,
    pub closest_realizations_in_closest: Vec<usize>,
    pub self_realizations_in_closest: Vec<usize>,
    pub self_characteristics: Vec<Characteristics>,
    pub kullback_criteria: Vec<f32>,
    pub shannon_criteria: Vec<f32>,
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

        let kullback_criteria = Self::kullback_criteria(&self_characteristics);
        let shannon_criteria = Self::shannon_criteria(&self_characteristics);

        Self {
            self_realizations,
            closest_realizations,
            closest_realizations_in_closest,
            self_realizations_in_closest,
            self_characteristics,
            kullback_criteria,
            shannon_criteria,
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

    fn shannon_criteria(characteristics: &[Characteristics]) -> Vec<f32> {
        let error: f32 = 0.00001;

        characteristics
            .iter()
            .map(|c| {
                let mut a = c.alpha;
                let mut b = c.beta;
                let mut d1 = c.d1;
                let mut d2 = c.d2;

                let mut divisor1 = a + d2;
                let mut divisor2 = d1 + b;

                if divisor1 == 0.0 {
                    a += error;
                    d2 += error;
                    divisor1 += error;
                }

                if divisor2 == 0.0 {
                    b += error;
                    d1 += error;
                    divisor2 += error;
                }

                1.0 + 0.5
                    * ((a / divisor1) * (a / divisor1).log2()
                        + (d1 / divisor2) * (d1 / divisor2).log2()
                        + (b / divisor2) * (b / divisor2).log2()
                        + (d2 / divisor1) * (d2 / divisor1).log2())
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
