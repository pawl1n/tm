#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod class_data;
mod corridor;

use class_data::ClassData;
use corridor::{Corridor, DrawPlot};

use eframe::{egui, epaint::ColorImage};
use image::{open, EncodableLayout};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
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
    size: Option<(usize, usize)>,
    error: Option<String>,
    selected_class: usize,
    classes: Vec<ClassData>,
    matrices: Vec<ClassData>,
    delta: u8,
    corridor: Corridor,
    reference_vectors: Vec<ClassData>,
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
                egui::ScrollArea::new([true, true]).show(ui, |ui| {
                    if !self.classes.is_empty() {
                        ui.horizontal(|ui| {
                            let selected = &self.classes[self.selected_class];

                            frame.clone().show(ui, |ui| {
                                ui.set_max_width(self.size.unwrap().0 as f32);
                                ui.vertical_centered(|ui| {
                                    ui.add(egui::Label::new("Selected image"));
                                    ui.image((
                                        selected.texture().id(),
                                        selected.texture().size_vec2(),
                                    ));
                                });
                            });
                            frame.clone().show(ui, |ui| {
                                self.corridor.draw_corridor_plot(ui);
                            });
                        });
                    }

                    frame.clone().show(ui, |ui| {
                        if !self.matrices.is_empty() {
                            ui.add(egui::Label::new("Binary matrices"));

                            ui.horizontal_wrapped(|ui| {
                                self.matrices.iter().for_each(|matrix| {
                                    ui.image((matrix.texture().id(), matrix.texture().size_vec2()));
                                });
                            });
                        }

                        if !self.reference_vectors.is_empty() {
                            ui.add(egui::Label::new("Reference vectors"));

                            ui.horizontal_wrapped(|ui| {
                                self.reference_vectors.iter().for_each(|vector| {
                                    ui.image((vector.texture().id(), vector.texture().size_vec2()));
                                });
                            });
                        }

                        if !self.classes.is_empty() {
                            ui.add(egui::Label::new("All classes"));

                            ui.horizontal_wrapped(|ui| {
                                self.classes.iter().for_each(|matrix| {
                                    ui.image((matrix.texture().id(), matrix.texture().size_vec2()));
                                });
                            });
                        }
                    });
                });
            });
    }
}

impl MyApp {
    fn calculate_binary_matrices(&mut self, ctx: &egui::Context) {
        if let Some((w, h)) = self.size {
            let lower = self.corridor.lower_allowance();
            let upper = self.corridor.upper_allowance();

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
                            let index = i.rem_euclid(w);
                            if *x > lower[index] && *x < upper[index] {
                                255
                            } else {
                                0
                            }
                        })
                        .collect();

                    let image = ColorImage::from_gray([w, h], &key);
                    let texture = ctx.load_texture(
                        "matrix".to_owned() + &i.to_string(),
                        image,
                        Default::default(),
                    );

                    ClassData::new(key, texture)
                })
                .collect();

            self.calculate_reference_vectors(ctx);
        }
    }

    fn calculate_reference_vectors(&mut self, ctx: &egui::Context) {
        if let Some((w, h)) = self.size {
            self.reference_vectors = self
                .matrices
                .iter()
                .enumerate()
                .map(|(i, matrix)| {
                    let mut vector = Vec::with_capacity(w);

                    for i in 0..w {
                        let mut count = 0;

                        for j in 0..h {
                            if matrix.bytes()[i + j * w] == 255 {
                                count += 1;
                            }
                        }

                        vector.push(if count > h / 2 { u8::MAX } else { u8::MIN });
                    }

                    let image = ColorImage::from_gray([w, 10], &vector.repeat(10));
                    let texture = ctx.load_texture(
                        "reference_vector".to_owned() + &i.to_string(),
                        image,
                        Default::default(),
                    );

                    ClassData::new(vector, texture)
                })
                .collect();
        }
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
                    self.corridor.set_base_class(
                        self.classes[self.selected_class].bytes(),
                        self.size.unwrap(),
                    );
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
                self.corridor.delta(self.delta);
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
                    if self.classes.len() == 1 {
                        self.corridor = Corridor::new(self.classes[0].bytes(), self.size.unwrap())
                    }
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
            self.size = Some((luma.width() as usize, luma.height() as usize));
        } else {
            if self.size.unwrap() != (luma.width() as usize, luma.height() as usize) {
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
