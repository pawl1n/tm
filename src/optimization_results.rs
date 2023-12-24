use crate::draw::Show;
use eframe::egui::Ui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

pub struct OptimizationResults {
    pub kullback_criteria: Vec<f64>,
    pub shannon_criteria: Vec<f64>,
    pub working_space: Vec<bool>,
}

impl Show for OptimizationResults {
    fn show(&self, ui: &mut Ui) {
        Plot::new("Optimization results")
            .legend(Legend::default())
            .auto_bounds_x()
            .auto_bounds_y()
            .show(ui, |ui| {
                ui.line(
                    Line::new(
                        self.kullback_criteria
                            .iter()
                            .enumerate()
                            .filter(|(_, &y)| y != 0.0)
                            .map(|(i, &y)| [i as f64, y])
                            .collect::<Vec<[f64; 2]>>(),
                    )
                    .name("Kullback"),
                );
                ui.line(
                    Line::new(
                        self.shannon_criteria
                            .iter()
                            .enumerate()
                            .filter(|(_, &y)| y != 0.0)
                            .map(|(i, &y)| [i as f64, y])
                            .collect::<Vec<[f64; 2]>>(),
                    )
                    .name("Shannon"),
                );

                let points: PlotPoints = self
                    .working_space
                    .iter()
                    .enumerate()
                    .filter(|(_, &w)| w)
                    .map(|(i, _)| [i as f64, self.shannon_criteria[i]])
                    .collect();
                ui.line(Line::new(points).name("Working space").fill(0.0));
            });
    }
}

impl From<Vec<(f64, f64, bool)>> for OptimizationResults {
    fn from(value: Vec<(f64, f64, bool)>) -> Self {
        Self {
            shannon_criteria: value.iter().map(|(s, _, _)| *s).collect(),
            kullback_criteria: value.iter().map(|(_, k, _)| *k).collect(),
            working_space: value.iter().map(|(_, _, w)| *w).collect(),
        }
    }
}
