use crate::draw::Draw;

use eframe::egui::Ui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

#[derive(Debug, Default)]
pub struct Criteria {
    pub characteristics: Vec<Characteristics>,
    pub kullback_criteria: Vec<f64>,
    pub shannon_criteria: Vec<f64>,
    working_space: Vec<usize>,
    pub r_kullback: Vec<f64>,
    pub r_shannon: Vec<f64>,
}

#[derive(Debug, Default)]
pub struct Characteristics {
    pub d1: f64,
    pub beta: f64,
    pub alpha: f64,
    pub d2: f64,
}

impl Criteria {
    pub fn new(
        distances_to_self: &[u32],
        distances_to_closest: &[u32],
        number_of_realizations: usize,
    ) -> Self {
        let max_radius = Self::calculate_max_radius(distances_to_self, distances_to_closest);

        let self_realizations =
            Self::calculate_number_of_self_realizations(distances_to_self, max_radius);
        let closest_realizations =
            Self::calculate_number_of_self_realizations(distances_to_closest, max_radius);
        // let closest_realizations_in_closest = Self::calculate_number_of_self_realizations(
        //     &sk.distances_from_closest_to_itself,
        //     max_radius,
        // );
        // let self_realizations_in_closest =
        //     Self::calculate_number_of_self_realizations(&sk.distances_from_closest, max_radius);

        let characteristics = Self::calculate_characteristics(
            &self_realizations,
            &closest_realizations,
            number_of_realizations,
        );

        let kullback_criteria = Self::kullback_criteria(&characteristics);
        let shannon_criteria = Self::shannon_criteria(&characteristics);

        let working_space: Vec<usize> = characteristics
            .iter()
            .enumerate()
            .filter(|(_, c)| c.d1 >= 0.5 && c.d2 >= 0.5)
            .map(|(i, _)| i)
            .collect();

        let r_kullback = Self::find_radius(&kullback_criteria, &working_space);
        let r_shannon = Self::find_radius(&shannon_criteria, &working_space);

        Self {
            characteristics,
            kullback_criteria,
            shannon_criteria,
            working_space,
            r_kullback,
            r_shannon,
        }
    }

    fn find_radius(criteria: &[f64], working_space: &[usize]) -> Vec<f64> {
        let iter = working_space.iter().map(|i| criteria[*i]);

        let max = iter.clone().reduce(f64::max).unwrap_or_default();

        iter.filter(|&c| c == max).collect()
    }

    fn kullback_criteria(characteristics: &[Characteristics]) -> Vec<f64> {
        characteristics
            .iter()
            .enumerate()
            .map(|(_, c)| {
                ((2.0 - (c.alpha + c.beta)) / (c.alpha + c.beta)).log2()
                    * (1.0 - (c.alpha + c.beta))
            })
            .collect()
    }

    fn shannon_criteria(characteristics: &[Characteristics]) -> Vec<f64> {
        characteristics
            .iter()
            .enumerate()
            .map(|(_, c)| {
                let a = c.alpha;
                let b = c.beta;
                let d1 = c.d1;
                let d2 = c.d2;

                let mut divisor1 = a + d2;
                let mut divisor2 = d1 + b;

                if divisor1 == 0.0 {
                    divisor1 = 0.000001;
                }

                if divisor2 == 0.0 {
                    divisor2 = 0.000001;
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
            let d1 = realizations[i] as f64 / number_of_realizations as f64;
            let alpha = 1.0 - d1;
            let beta = closest_realizations[i] as f64 / number_of_realizations as f64;
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

    fn calculate_max_radius(distances_to_self: &[u32], distances_to_closest: &[u32]) -> u32 {
        let self_max_distance = distances_to_self.iter().max().unwrap_or(&0);
        let closest_max_distance = distances_to_closest.iter().max().unwrap_or(&0);

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

impl Draw for Criteria {
    fn draw(&self, ui: &mut Ui) {
        let available_width = ui.max_rect().width() - 10.0;
        let available_height = ui.max_rect().height() - 10.0;

        ui.horizontal(|ui| {
            ui.set_height(available_height);

            Plot::new("Kullback")
                .legend(Legend::default())
                .width(available_width / 2.0)
                .show(ui, |ui| {
                    ui.line(
                        Line::new(PlotPoints::from_ys_f64(&self.kullback_criteria))
                            .name("Kullback"),
                    );

                    let points: PlotPoints = self
                        .working_space
                        .iter()
                        .map(|&i| [i as f64, self.kullback_criteria[i]])
                        .collect();
                    ui.line(Line::new(points).name("Working space").fill(0.0));
                });

            Plot::new("Shannon")
                .legend(Legend::default())
                .width(available_width / 2.0)
                .show(ui, |ui| {
                    ui.line(
                        Line::new(PlotPoints::from_ys_f64(&self.shannon_criteria)).name("Shannon"),
                    );

                    let points: PlotPoints = self
                        .working_space
                        .iter()
                        .map(|&i| [i as f64, self.shannon_criteria[i]])
                        .collect();

                    ui.line(Line::new(points).name("Working space").fill(0.0));
                });
        });
    }
}
