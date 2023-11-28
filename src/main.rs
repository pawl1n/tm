#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod class_data;
mod class_loader;
mod corridor;
mod criteria;
mod draw;
mod hamming;
mod painter;

use class_data::ClassData;
use corridor::Corridor;
use draw::Draw;

use eframe::{egui, epaint::ColorImage};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "SATPR",
        options,
        Box::new(|cc| {
            // Image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

#[derive(Default)]
struct MyApp {
    selected_class: usize,
    matrices: Vec<ClassData>,
    exam_matrices: Vec<ClassData>,
    delta: u8,
    corridor: Corridor,
    reference_vectors: Vec<ClassData>,
    exam_reference_vectors: Vec<ClassData>,
    sk: Vec<painter::SK>,
    widget_stauses: std::collections::HashMap<String, bool>,
    criterias: Vec<criteria::Criteria>,
    class_loader: class_loader::ClassLoader,
    closest_criterias: Vec<criteria::Criteria>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame: egui::Frame = egui::Frame {
            fill: ctx.style().visuals.window_fill(),
            inner_margin: 10.0.into(),
            ..Default::default()
        };

        egui::SidePanel::right("Widgets")
            .frame(frame)
            .show(ctx, |ui| {
                self.add_widgets_controls(ui);
            });

        egui::TopBottomPanel::bottom(egui::Id::new("Loader"))
            .frame(frame)
            .show(ctx, |ui| {
                if self.class_loader.show(ui).loaded() {
                    if self.class_loader.classes.len() == 1 {
                        self.corridor = Corridor::new(
                            self.class_loader.classes[0].bytes(),
                            self.class_loader.size.unwrap(),
                        )
                    }
                    self.calculate_binary_matrices(ui.ctx());
                }
            });

        if *self.widget_stauses.get("Criteria").unwrap_or(&false) {
            self.criterias.iter().enumerate().for_each(|(i, criteria)| {
                egui::Window::new(format!("Criteria {i}"))
                    .id(egui::Id::new(format!("Criteria{i}")))
                    .default_size(egui::vec2(250.0, 200.0))
                    .min_width(400.0)
                    .min_height(150.0)
                    .show(ctx, |ui| {
                        criteria.draw(ui);
                    });
            });

            self.closest_criterias
                .iter()
                .enumerate()
                .for_each(|(i, criteria)| {
                    egui::Window::new(format!("Criteria {i} closest"))
                        .id(egui::Id::new(format!("Criteria{i} closest")))
                        .default_size(egui::vec2(250.0, 200.0))
                        .min_width(400.0)
                        .min_height(150.0)
                        .show(ctx, |ui| {
                            criteria.draw(ui);
                        });
                });
        }

        if *self.widget_stauses.get("2D").unwrap_or(&false) {
            self.sk.iter().enumerate().for_each(|(i, sk)| {
                egui::Window::new(format!("2D {i}->{}", sk.closest))
                    .id(egui::Id::new(format!("2D{i}")))
                    .default_size(egui::vec2(250.0, 200.0))
                    .min_width(250.0)
                    .min_height(200.0)
                    .show(ctx, |ui| {
                        sk.paint(ui);
                    });
            });
        }

        if *self.widget_stauses.get("Information").unwrap_or(&false) {
            egui::Window::new("Information")
                .resizable(false)
                .show(ctx, |ui| {
                    frame.show(ui, |ui| self.add_stats(ui));
                });
        }

        if !self.class_loader.classes.is_empty() {
            if *self.widget_stauses.get("Settings").unwrap_or(&false) {
                egui::Window::new("Settings")
                    .resizable(false)
                    .show(ctx, |ui| {
                        frame.show(ui, |ui| self.add_controls(ui));
                    });
            }

            if *self.widget_stauses.get("Plot").unwrap_or(&false) {
                egui::Window::new("Plot")
                    .default_size(egui::vec2(400.0, 200.0))
                    .show(ctx, |ui| {
                        frame.show(ui, |ui| {
                            self.corridor.draw(ui);
                        });
                    });
            }

            if *self.widget_stauses.get("SK").unwrap_or(&false) && !self.sk.is_empty() {
                egui::Window::new("SK").show(ctx, |ui| {
                    self.sk.iter().enumerate().for_each(|(i, sk)| {
                        ui.add(egui::Label::new(format!("SK[1,{i}]")));

                        egui::ScrollArea::horizontal()
                            .id_source(format!("sk{i}.0"))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    sk.distances_to_self.iter().for_each(|x| {
                                        ui.label(x.to_string());
                                    });
                                });
                            });
                        ui.add(egui::Label::new(format!("SK[2,{i}]")));
                        egui::ScrollArea::horizontal()
                            .id_source(format!("sk{i}.1"))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    sk.distances_to_closest.iter().for_each(|x| {
                                        ui.label(x.to_string());
                                    });
                                });
                            });
                    });
                });

                egui::Window::new("SK_PARA").show(ctx, |ui| {
                    self.sk.iter().enumerate().for_each(|(i, sk)| {
                        ui.add(egui::Label::new(format!("SK_PARA[1,{i}]")));

                        egui::ScrollArea::horizontal()
                            .id_source(format!("sk_para{i}.0"))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    sk.distances_from_closest_to_itself.iter().for_each(|x| {
                                        ui.label(x.to_string());
                                    });
                                });
                            });
                        ui.add(egui::Label::new(format!("SK_PARA[2,{i}]")));
                        egui::ScrollArea::horizontal()
                            .id_source(format!("sk_para{i}.1"))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    sk.distances_from_closest.iter().for_each(|x| {
                                        ui.label(x.to_string());
                                    });
                                });
                            });
                    });
                });
            }

            if *self.widget_stauses.get("Classes").unwrap_or(&false) {
                egui::Window::new("Classes")
                    .resizable(false)
                    .show(ctx, |ui| {
                        egui::ScrollArea::new([true, true]).show(ui, |ui| {
                            frame.show(ui, |ui| {
                                ui.add(egui::Label::new("Classes"));

                                ui.horizontal_wrapped(|ui| {
                                    self.class_loader.classes.iter().for_each(|matrix| {
                                        ui.image((
                                            matrix.texture().id(),
                                            matrix.texture().size_vec2(),
                                        ));
                                    });
                                });

                                if !self.matrices.is_empty() {
                                    ui.add(egui::Label::new("Binary matrices"));

                                    ui.horizontal_wrapped(|ui| {
                                        self.matrices.iter().for_each(|matrix| {
                                            ui.image((
                                                matrix.texture().id(),
                                                matrix.texture().size_vec2(),
                                            ));
                                        });
                                    });
                                }

                                if !self.reference_vectors.is_empty() {
                                    ui.add(egui::Label::new("Reference vectors"));

                                    ui.horizontal_wrapped(|ui| {
                                        self.reference_vectors.iter().for_each(|vector| {
                                            ui.image((
                                                vector.texture().id(),
                                                vector.texture().size_vec2(),
                                            ));
                                        });
                                    });
                                }
                            });
                        });
                    });
            }

            if *self.widget_stauses.get("Exam classes").unwrap_or(&false) {
                egui::Window::new("Exam classes")
                    .resizable(false)
                    .show(ctx, |ui| {
                        egui::ScrollArea::new([true, true]).show(ui, |ui| {
                            frame.show(ui, |ui| {
                                ui.add(egui::Label::new("Classes"));

                                ui.horizontal_wrapped(|ui| {
                                    self.class_loader.exam_classes.iter().for_each(|matrix| {
                                        ui.image((
                                            matrix.texture().id(),
                                            matrix.texture().size_vec2(),
                                        ));
                                    });
                                });

                                if !self.matrices.is_empty() {
                                    ui.add(egui::Label::new("Binary matrices"));

                                    ui.horizontal_wrapped(|ui| {
                                        self.exam_matrices.iter().for_each(|matrix| {
                                            ui.image((
                                                matrix.texture().id(),
                                                matrix.texture().size_vec2(),
                                            ));
                                        });
                                    });
                                }

                                if !self.reference_vectors.is_empty() {
                                    ui.add(egui::Label::new("Reference vectors"));

                                    ui.horizontal_wrapped(|ui| {
                                        self.exam_reference_vectors.iter().for_each(|vector| {
                                            ui.image((
                                                vector.texture().id(),
                                                vector.texture().size_vec2(),
                                            ));
                                        });
                                    });
                                }
                            });
                        });
                    });
            }
        }
    }
}

impl MyApp {
    fn calculate_criteria(&mut self) {
        let (_, number_of_realizations) = self.class_loader.size.expect("Expected size");

        self.criterias = self
            .sk
            .iter_mut()
            .map(|sk| {
                let criteria = criteria::Criteria::new(
                    &sk.distances_to_self,
                    &sk.distances_to_closest,
                    number_of_realizations,
                );

                sk.set_radius(criteria.r_kullback.clone(), criteria.r_shannon.clone());

                criteria
            })
            .collect();

        self.closest_criterias = self
            .sk
            .iter_mut()
            .map(|sk| {
                let criteria = criteria::Criteria::new(
                    &sk.distances_from_closest_to_itself,
                    &sk.distances_from_closest,
                    number_of_realizations,
                );

                sk.set_radius_closest(criteria.r_kullback.clone(), criteria.r_shannon.clone());

                criteria
            })
            .collect();
    }

    fn calculate_code_distances(&mut self) {
        self.sk = Vec::with_capacity(self.class_loader.classes.len());

        if self.class_loader.classes.len() <= 1 {
            return;
        }

        for index in 0..self.matrices.len() {
            let center = self.reference_vectors[index].bytes();

            let distances_to_center =
                hamming::distances_between(self.matrices[index].bytes(), center);
            let (distances_to_closest, distance, closest) = self
                .reference_vectors
                .iter()
                .enumerate()
                .filter(|(i, _)| i != &index)
                .map(|(i, reference_vector)| {
                    (
                        i,
                        hamming::distance_between(reference_vector.bytes(), center),
                    )
                })
                .min_by_key(|(_, distance)| *distance)
                .map_or_else(
                    || (Vec::new(), 0, 0),
                    |(closest, distance)| {
                        (
                            hamming::distances_between(self.matrices[closest].bytes(), center),
                            distance,
                            closest,
                        )
                    },
                );

            let center = self.reference_vectors[closest].bytes();

            let distances_from_closest_to_itself =
                hamming::distances_between(self.matrices[closest].bytes(), center);
            let distances_from_closest =
                hamming::distances_between(self.matrices[index].bytes(), center);

            self.sk.push(painter::SK::new(
                distances_to_center,
                distances_to_closest,
                distances_from_closest_to_itself,
                distances_from_closest,
                distance,
                closest,
            ));
        }
    }

    fn calculate_binary_matrices(&mut self, ctx: &egui::Context) {
        if let Some(size) = self.class_loader.size {
            let lower = self.corridor.lower_allowance();
            let upper = self.corridor.upper_allowance();

            self.matrices =
                Self::binary_matrices_for(&self.class_loader.classes, size, lower, upper, ctx);
            self.exam_matrices =
                Self::binary_matrices_for(&self.class_loader.exam_classes, size, lower, upper, ctx);

            self.calculate_reference_vectors(size, ctx);
            self.calculate_code_distances();
            self.calculate_criteria();
        }
    }

    fn binary_matrices_for(
        classes: &[ClassData],
        size: (usize, usize),
        lower: &[u8],
        upper: &[u8],
        ctx: &egui::Context,
    ) -> Vec<ClassData> {
        let (attributes, realizations) = size;

        classes
            .iter()
            .enumerate()
            .map(|(i, class)| {
                let key: Vec<u8> = class
                    .bytes()
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        let index = i.rem_euclid(attributes);
                        if *x >= lower[index] && *x <= upper[index] {
                            u8::MAX
                        } else {
                            u8::MIN
                        }
                    })
                    .collect();

                let image = ColorImage::from_gray([attributes, realizations], &key);
                let texture = ctx.load_texture(
                    "matrix".to_owned() + &i.to_string(),
                    image,
                    Default::default(),
                );

                ClassData::new(key, texture)
            })
            .collect()
    }

    fn calculate_reference_vectors(&mut self, size: (usize, usize), ctx: &egui::Context) {
        self.reference_vectors = Self::reference_vectors_for(&self.matrices, size, ctx);
        self.exam_reference_vectors = Self::reference_vectors_for(&self.exam_matrices, size, ctx);
    }

    fn reference_vectors_for(
        matrices: &[ClassData],
        size: (usize, usize),
        ctx: &egui::Context,
    ) -> Vec<ClassData> {
        let (attributes, realizations) = size;

        matrices
            .iter()
            .enumerate()
            .map(|(i, matrix)| {
                let mut vector = Vec::with_capacity(attributes);

                for i in 0..attributes {
                    let mut count = 0;

                    for j in 0..realizations {
                        if matrix.bytes()[i + j * attributes] == u8::MAX {
                            count += 1;
                        }
                    }

                    vector.push(if count > realizations / 2 {
                        u8::MAX
                    } else {
                        u8::MIN
                    });
                }

                let image = ColorImage::from_gray([attributes, 10], &vector.repeat(10));
                let texture = ctx.load_texture(
                    "reference_vector".to_owned() + &i.to_string(),
                    image,
                    Default::default(),
                );

                ClassData::new(vector, texture)
            })
            .collect()
    }

    fn set_base_class(&mut self, class: usize, ui: &mut egui::Ui) {
        if self.class_loader.classes.len() <= class {
            return;
        }

        self.selected_class = class;
        self.corridor.set_base_class(
            self.class_loader.classes[self.selected_class].bytes(),
            self.class_loader.size.unwrap(),
        );
        self.calculate_binary_matrices(ui.ctx());
    }

    fn add_controls(&mut self, ui: &mut egui::Ui) {
        let selected = &self.class_loader.classes[self.selected_class];
        ui.add(egui::Label::new("Selected class"));
        ui.image((selected.texture().id(), selected.texture().size_vec2()));
        ui.add(egui::Label::new("Select class:"));
        ui.horizontal_wrapped(|ui| {
            for i in 0..self.class_loader.classes.len() {
                if ui
                    .add(egui::widgets::RadioButton::new(
                        self.selected_class == i,
                        (i + 1).to_string(),
                    ))
                    .clicked()
                {
                    self.set_base_class(i, ui);
                }
            }
        });
        ui.horizontal(|ui| {
            ui.add(egui::Label::new("Delta"));
            if ui
                .add(egui::Slider::new(&mut self.delta, u8::MIN..=u8::MAX))
                .changed()
            {
                self.corridor.delta(self.delta);
                self.calculate_binary_matrices(ui.ctx());
            }
        });

        if ui.button("Delete").clicked() {
            self.class_loader.classes.remove(self.selected_class);
            self.set_base_class(0, ui);
        }
    }

    fn add_stats(&self, ui: &mut egui::Ui) {
        ui.add(egui::Label::new(format!(
            "Number of realizations: {}",
            self.class_loader.size.map_or(0, |s| s.1)
        )));
        ui.add(egui::Label::new(format!(
            "Number of attributes: {}",
            self.class_loader.size.map_or(0, |s| s.0)
        )));
        ui.add(egui::Label::new(format!(
            "Number of classes: {}",
            self.class_loader.classes.len()
        )));
    }

    fn add_widgets_controls(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            self.add_button("Information", ui);
            self.add_button("Settings", ui);
            self.add_button("Classes", ui);
            self.add_button("Plot", ui);
            self.add_button("SK", ui);
            self.add_button("2D", ui);
            self.add_button("Criteria", ui);
            self.add_button("Exam classes", ui);
        });
    }

    fn add_button(&mut self, label: &str, ui: &mut egui::Ui) {
        ui.checkbox(
            self.widget_stauses.entry(label.to_string()).or_default(),
            label,
        );
    }
}
