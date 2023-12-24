use super::draw::Show;
use eframe::egui::Ui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

#[derive(Default, Debug)]
pub struct Allowances {
    pub lower: Vec<f64>,
    pub upper: Vec<f64>,
}

#[derive(Debug, Default)]
pub struct Corridor {
    expectation: Vec<f64>,
    pub allowances: Allowances,
    delta: u8,
}

impl Corridor {
    pub fn new(selected_class: &[u8], size: (usize, usize)) -> Self {
        let mut created = Corridor {
            expectation: Corridor::math_expectation(selected_class, size),
            ..Default::default()
        };

        created.calculate_allowances();

        created
    }

    pub fn set_base_class(&mut self, selected_class: &[u8], size: (usize, usize)) {
        self.expectation = Corridor::math_expectation(selected_class, size);
        self.calculate_allowances();
    }

    pub fn delta(&mut self, delta: u8) {
        self.delta = delta;
        self.calculate_allowances();
    }

    fn calculate_allowances(&mut self) {
        self.calculate_lower_allowance();
        self.calculate_upper_allowance();
    }

    fn math_expectation(selected_class: &[u8], (w, h): (usize, usize)) -> Vec<f64> {
        (0..w).map(|i| {
            (0..h).map(|j| {
                selected_class[i + j * h] as u32
            }).sum::<u32>() as f64 / h as f64
        }).collect()
    }

    fn calculate_lower_allowance(&mut self) {
        self.allowances.lower = self
            .expectation
            .iter()
            .map(|x| x - self.delta as f64)
            .collect();
    }

    fn calculate_upper_allowance(&mut self) {
        self.allowances.upper = self
            .expectation
            .iter()
            .map(|x| x + self.delta as f64)
            .collect();
    }
}

impl Show for Corridor {
    fn show(&self, ui: &mut Ui) {
        Plot::new("Corridor")
            .legend(Legend::default())
            .auto_bounds_x()
            .auto_bounds_y()
            .show(ui, |ui| {
                ui.line(Line::new(PlotPoints::from_ys_f64(&self.expectation)).name("Expectation"));
                ui.line(
                    Line::new(PlotPoints::from_ys_f64(&self.allowances.lower)).name("Lower allowance"),
                );
                ui.line(
                    Line::new(PlotPoints::from_ys_f64(&self.allowances.upper)).name("Upper allowance"),
                );
            });
    }
}
