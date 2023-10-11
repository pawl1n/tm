#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod class_data;
use class_data::ClassData;

use eframe::{egui, epaint::ColorImage};
use image::{open, EncodableLayout};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 500.0)),
        ..Default::default()
    };

    eframe::run_native(
        "SATPR",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

#[derive(Default)]
struct MyApp {
    path: String,
    size: Option<(u32, u32)>,
    error: Option<String>,
    selected_class: usize,
    corridor: (Vec<u8>, Vec<u8>, Vec<u8>), // math expectation and lower and upper allowances
    delta: u8,
    classes: Vec<ClassData>,
    matrices: Vec<ClassData>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame: egui::Frame = egui::Frame {
            fill: ctx.style().visuals.window_fill(),
            inner_margin: 10.0.into(),
            ..Default::default()
        };

        egui::TopBottomPanel::bottom(egui::Id::new("Loader"))
            .frame(frame.clone())
            .show(ctx, |ui| {
                self.add_image_selector(ui);
            });

        egui::SidePanel::right(egui::Id::new("Info"))
            .min_width(200.0)
            .frame(frame.clone())
            .resizable(false)
            .show(ctx, |ui| {
                frame.clone().show(ui, |ui| self.add_stats(ui));

                // if !self.images.is_empty() {
                if !self.classes.is_empty() {
                    frame.clone().show(ui, |ui| self.add_controls(ui));
                }
            });

        egui::CentralPanel::default()
            .frame(frame.clone())
            .show(ctx, |ui| {
                if !self.classes.is_empty() {
                    frame.clone().show(ui, |ui| {
                        let selected = &self.classes[self.selected_class];

                        ui.vertical_centered_justified(|ui| {
                            ui.add(egui::Label::new("Selected image"));
                            ui.image((selected.texture().id(), selected.texture().size_vec2()));
                        });
                        frame.clone().show(ui, |ui| {
                            self.draw_plot(ui);
                        });
                    });
                }

                frame.clone().show(ui, |ui| {
                    ui.add(egui::Label::new("Binary matrices"));

                    ui.horizontal_wrapped(|ui| {
                        self.matrices.iter().for_each(|matrix| {
                            ui.image((matrix.texture().id(), matrix.texture().size_vec2()));
                        });
                    });
                });

                egui::ScrollArea::new([true, true]).show(ui, |ui| {
                    ui.add(egui::Label::new("All classes"));

                    ui.horizontal_wrapped(|ui| {
                        self.classes.iter().for_each(|matrix| {
                            ui.image((matrix.texture().id(), matrix.texture().size_vec2()));
                        });
                    });
                });
            });
    }
}

impl MyApp {
    fn calculate_binary_matrices(&mut self, ctx: &egui::Context) {
        if let Some((w, h)) = self.size {
            let (_, lower, upper) = &self.corridor;

            self.matrices = self
                .classes
                .iter()
                .enumerate()
                .map(|(i, class)| {
                    let key: Vec<u8> = class
                        .bytes()
                        .iter()
                        .enumerate()
                        .map(|(i, x)| {
                            let index = i.rem_euclid(w as usize);
                            if *x > lower[index] && *x < upper[index] {
                                255
                            } else {
                                0
                            }
                        })
                        .collect();

                    let image = ColorImage::from_gray([w as usize, h as usize], &key);
                    let texture = ctx.load_texture(i.to_string(), image, Default::default());

                    ClassData::new(key, texture)
                })
                .collect();
        }
    }

    fn draw_plot(&self, ui: &mut egui::Ui) {
        egui_plot::Plot::new("Corridor")
            .legend(egui_plot::Legend::default())
            .height(200.0)
            .show(ui, |ui| {
                ui.line(
                    egui_plot::Line::new(vec_to_plot_points(&self.corridor.0)).name("Expectation"),
                );

                ui.line(
                    egui_plot::Line::new(vec_to_plot_points(&self.corridor.1))
                        .name("Lower allowance"),
                );

                ui.line(
                    egui_plot::Line::new(vec_to_plot_points(&self.corridor.2))
                        .name("Upper allowance"),
                );
            });
    }

    fn calculate_corridor(&mut self) {
        let expectation = self.math_expectation();
        let lower_allowance = self.lower_allowance(&expectation, self.delta);
        let upper_allowance = self.upper_allowance(&expectation, self.delta);

        self.corridor = (expectation, lower_allowance, upper_allowance);
    }

    fn math_expectation(&self) -> Vec<u8> {
        let (w, h) = self.size.expect("Invalid state: select image before use");

        let selected = &self.classes[self.selected_class];

        let mut avgerage_by_col: Vec<u8> = Vec::with_capacity(w as usize);

        // let bytes = selected.0.as_bytes();
        let bytes = selected.bytes();

        for i in 0..w {
            let mut sum: u32 = 0;

            for j in 0..h {
                sum += bytes[(i * j) as usize] as u32;
            }

            avgerage_by_col.push((sum / w) as u8);
        }

        avgerage_by_col
    }

    fn lower_allowance(&self, expectation: &[u8], delta: u8) -> Vec<u8> {
        expectation
            .iter()
            .map(|x| x.saturating_sub(delta))
            .collect()
    }

    fn upper_allowance(&self, expectation: &[u8], delta: u8) -> Vec<u8> {
        expectation
            .iter()
            .map(|x| x.saturating_add(delta))
            .collect()
    }

    fn add_controls(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::Label::new("Select class:"));
        ui.horizontal_wrapped(|ui| {
            for i in 0..self.classes.len() {
                if ui
                    .add(egui::widgets::RadioButton::new(
                        self.selected_class == i,
                        (i + 1).to_string(),
                    ))
                    .clicked()
                {
                    self.selected_class = i;
                    self.calculate_corridor();
                    self.calculate_binary_matrices(ui.ctx());
                }
            }
        });
        ui.horizontal(|ui| {
            ui.add(egui::Label::new("Delta"));
            if ui
                .add(egui::Slider::new(&mut self.delta, 0..=255))
                .changed()
            {
                self.calculate_corridor();
                self.calculate_binary_matrices(ui.ctx());
            }
        });
    }

    fn add_stats(&self, ui: &mut egui::Ui) {
        ui.add(egui::Label::new(format!(
            "Number of realizations: {}",
            self.size.map_or(0, |s| s.1)
        )));
        ui.add(egui::Label::new(format!(
            "Number of traits: {}",
            self.size.map_or(0, |s| s.0)
        )));
        ui.add(egui::Label::new(format!(
            "Number of classes: {}",
            self.classes.len()
        )));
    }

    fn add_image_selector(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.path));

            if ui.add(egui::Button::new("Load image")).clicked() {
                if let Err(msg) = self.load_grayscale(ui.ctx()) {
                    self.error = Some(msg.to_string());
                } else {
                    self.error = None;
                    self.path.clear();
                    self.calculate_corridor();
                    self.calculate_binary_matrices(ui.ctx());
                }
            };

            if let Some(message) = &self.error {
                ui.add(egui::Label::new(message));
                if ui.add(egui::Button::new("x")).clicked() {
                    self.error = None;
                }
            }
        });
    }

    fn load_grayscale(&mut self, ctx: &egui::Context) -> Result<(), String> {
        let luma = open(&self.path).map_err(|err| err.to_string())?.to_luma8();
        let vec = luma.to_vec();

        if self.classes.iter().any(|x| x.bytes().eq(&vec)) {
            return Err("This image has already been loaded".to_owned());
        }

        if self.size.is_none() {
            self.size = Some((luma.width(), luma.height()));
        } else {
            if self.size.unwrap() != (luma.width(), luma.height()) {
                return Err("Error: Images should be of the same format".to_owned());
            }
        }

        let image = ColorImage::from_gray(
            [luma.width() as usize, luma.height() as usize],
            luma.as_bytes(),
        );

        let texture = ctx.load_texture(&self.path, image, Default::default());

        self.classes.push(ClassData::new(vec, texture));

        Ok(())
    }
}

fn vec_to_plot_points(value: &[u8]) -> egui_plot::PlotPoints {
    let points: Vec<f32> = value.iter().map(|x| *x as f32).collect();
    egui_plot::PlotPoints::from_ys_f32(&points)
}
