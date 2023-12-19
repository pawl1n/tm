use crate::draw::Draw;
use eframe::egui::Ui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

pub struct OptimizationResults {
    pub kullback_criteria: Vec<f64>,
    pub shannon_criteria: Vec<f64>,
}

impl Draw for OptimizationResults {
    fn draw(&self, ui: &mut Ui) {
        Plot::new("Optimization results")
            .legend(Legend::default())
            .show(ui, |ui| {
                ui.line(
                    Line::new(PlotPoints::from_ys_f64(&self.kullback_criteria)).name("Kullback"),
                );
                ui.line(Line::new(PlotPoints::from_ys_f64(&self.shannon_criteria)).name("Shannon"));
            });
    }
}

impl From<Vec<(f64, f64)>> for OptimizationResults {
    fn from(value: Vec<(f64, f64)>) -> Self {
        Self {
            shannon_criteria: value.iter().map(|(s, _)| *s).collect(),
            kullback_criteria: value.iter().map(|(_, k)| *k).collect(),
        }
    }
}
