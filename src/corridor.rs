use super::draw::Show;
use eframe::egui::Ui;
use egui_plot::{Legend, Line, Plot};

#[derive(Default, Debug)]
pub struct Allowances {
    pub lower: Vec<u8>,
    pub upper: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct Corridor {
    expectation: Vec<u8>,
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

    fn math_expectation(selected_class: &[u8], (w, h): (usize, usize)) -> Vec<u8> {
        let mut avgerage_by_col: Vec<u8> = Vec::with_capacity(w);

        for i in 0..w {
            let mut sum: u32 = 0;

            for j in 0..h {
                sum += selected_class[i + j * h] as u32;
            }

            avgerage_by_col.push((sum as usize / w) as u8);
        }

        avgerage_by_col
    }

    fn calculate_lower_allowance(&mut self) {
        self.allowances.lower = self
            .expectation
            .iter()
            .map(|x| x.saturating_sub(self.delta))
            .collect();
    }

    fn calculate_upper_allowance(&mut self) {
        self.allowances.upper = self
            .expectation
            .iter()
            .map(|x| x.saturating_add(self.delta))
            .collect();
    }
}

impl Show for Corridor {
    fn show(&self, ui: &mut Ui) {
        Plot::new("Corridor")
            .legend(Legend::default())
            .show(ui, |ui| {
                ui.line(Line::new(vec_to_plot_points(&self.expectation)).name("Expectation"));
                ui.line(
                    Line::new(vec_to_plot_points(&self.allowances.lower)).name("Lower allowance"),
                );
                ui.line(
                    Line::new(vec_to_plot_points(&self.allowances.upper)).name("Upper allowance"),
                );
            });
    }
}

fn vec_to_plot_points(value: &[u8]) -> egui_plot::PlotPoints {
    let points: Vec<f32> = value.iter().map(|x| *x as f32).collect();
    egui_plot::PlotPoints::from_ys_f32(&points)
}
