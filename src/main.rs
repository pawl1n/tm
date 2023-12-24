#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod binary_representation;
mod class_data;
mod class_loader;
mod class_manager;
mod corridor;
mod criteria;
mod draw;
mod exam_data;
mod hamming;
mod optimization_results;
mod sk;

use class_loader::ClassLoader;
use class_manager::ClassManager;
use corridor::Corridor;
use criteria::Criteria;
use draw::Show;

use eframe::egui;
use exam_data::ExamResult;
use hamming::SKManager;
use optimization_results::OptimizationResults;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
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
    delta: u8,
    corridor: Corridor,
    widget_stauses: std::collections::HashMap<String, bool>,
    exam_data: Vec<ExamResult>,
    optimization_results: Option<OptimizationResults>,
    class_manager: ClassManager,
    exam_class_manager: ClassManager,
    class_loader: ClassLoader,
    sk_manager: SKManager,
    criterias: Vec<Criteria>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame: egui::Frame = egui::Frame {
            fill: ctx.style().visuals.window_fill(),
            inner_margin: 10.0.into(),
            ..Default::default()
        };

        egui::TopBottomPanel::top("Stages")
            .frame(frame)
            .show(ctx, |ui| {
                self.add_widgets_controls(ui);
            });

        egui::TopBottomPanel::bottom(egui::Id::new("Loader"))
            .frame(frame)
            .show(ctx, |ui| {
                if let Some(data) = self
                    .class_loader
                    .show(&self.class_manager, &self.exam_class_manager, ui)
                    .loaded()
                {
                    if self.class_manager.classes.is_empty() {
                        self.corridor = Corridor::new(&data.bytes, self.class_loader.size)
                    }
                    match self.class_loader.class_type {
                        class_loader::ClassType::Training => {
                            self.class_manager.add_class(data.clone());
                            self.recalculate(ctx);
                        }
                        class_loader::ClassType::Exam => {
                            self.exam_class_manager.add_class(data.clone());
                            self.exam_class_manager
                                .recalculate_binary_representation(&self.corridor.allowances, ctx);
                            self.exam();
                        }
                    }
                }
            });

        egui::CentralPanel::default().frame(frame).show(ctx, |_| {
            if *self.widget_stauses.get("Criteria").unwrap_or(&false) {
                self.criterias.iter().enumerate().for_each(|(i, criteria)| {
                    egui::Window::new(format!("Criteria {i}"))
                        .id(egui::Id::new(format!("Criteria{i}")))
                        .default_size(egui::vec2(250.0, 200.0))
                        .min_width(400.0)
                        .min_height(150.0)
                        .show(ctx, |ui| {
                            criteria.show(ui);
                        });
                });
            }

            if *self.widget_stauses.get("2D").unwrap_or(&false) {
                self.sk_manager.sk.iter().enumerate().for_each(|(i, sk)| {
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
                            self.corridor.show(ui);
                        });
                    });
            }

            if *self.widget_stauses.get("Classes").unwrap_or(&false) {
                egui::Window::new("Classes")
                    .resizable(false)
                    .show(ctx, |ui| {
                        egui::ScrollArea::new([true, true]).show(ui, |ui| {
                            frame.show(ui, |ui| {
                                self.class_manager.show(ui);
                                self.class_manager.binary_representations.show(ui);
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
                                self.exam_class_manager.show(ui);
                                self.exam_class_manager.binary_representations.show(ui);
                            });
                        });
                    });
            }
            if *self.widget_stauses.get("Exam results").unwrap_or(&false) {
                egui::Window::new("Exam results").show(ctx, |ui| {
                    self.exam_data
                        .iter()
                        .enumerate()
                        .for_each(|(i, exam_result)| {
                            ui.add(egui::Label::new(format!(
                                "Exam result for {i}: {exam_result}"
                            )));
                        });
                });
            }

            if *self
                .widget_stauses
                .get("Optimization results")
                .unwrap_or(&false)
            {
                if let Some(optimization_results) = &self.optimization_results {
                    egui::Window::new(format!(
                        "Optimization result of delta for class {}",
                        self.class_manager.selected_class
                    ))
                    .default_size(egui::vec2(250.0, 200.0))
                    .min_width(400.0)
                    .min_height(150.0)
                    .show(ctx, |ui| {
                        optimization_results.show(ui);
                    });
                };
            }
        });
    }
}

impl MyApp {
    fn exam(&mut self) {
        self.exam_data = exam_data::exam(
            &self.class_manager.binary_representations.reference_vectors,
            &self.exam_class_manager.binary_representations.matrices,
            &self.criterias,
            self.class_loader.size.1,
        );
    }

    fn calculate_criteria(&mut self) {
        self.criterias = (0..self.sk_manager.sk.len())
            .map(|i| {
                Criteria::new(
                    i,
                    &self.sk_manager.distances_to_realizations[i],
                    self.class_loader.size.1,
                )
            })
            .collect();

        self.sk_manager
            .sk
            .iter_mut()
            .enumerate()
            .for_each(|(i, sk)| {
                sk.set_radius(
                    self.criterias[i].r_kullback.to_vec(),
                    self.criterias[i].r_shannon.to_vec(),
                );
                sk.set_radius_closest(
                    self.criterias[sk.closest].r_kullback.to_vec(),
                    self.criterias[sk.closest].r_shannon.to_vec(),
                );
            });
    }

    fn recalculate(&mut self, ctx: &egui::Context) {
        self.class_manager
            .recalculate_binary_representation(&self.corridor.allowances, ctx);
        self.exam_class_manager
            .recalculate_binary_representation(&self.corridor.allowances, ctx);

        self.sk_manager = SKManager::new(
            &self.class_manager.binary_representations.matrices,
            &self.class_manager.binary_representations.reference_vectors,
        );

        self.calculate_criteria();
        self.exam();
    }

    fn set_base_class(&mut self, class: usize, ui: &mut egui::Ui) {
        self.optimization_results = None;
        self.class_manager.selected_class = class;

        if self.class_manager.classes.len() <= class {
            return;
        }

        self.corridor.set_base_class(
            &self.class_manager.classes[self.class_manager.selected_class].bytes,
            self.class_manager.classes[0].texture.size().into(),
        );

        self.recalculate(ui.ctx());
    }

    fn add_controls(&mut self, ui: &mut egui::Ui) {
        if self.class_manager.show_controls(ui).changed() {
            self.set_base_class(self.class_manager.selected_class, ui);
        }

        ui.horizontal(|ui| {
            ui.add(egui::Label::new("Delta"));
            if ui
                .add(egui::Slider::new(&mut self.delta, u8::MIN..=u8::MAX))
                .changed()
            {
                self.optimization_results = None;
                self.corridor.delta(self.delta);
                self.recalculate(ui.ctx());
            }

            if self.class_manager.classes.len() > 1
                && ui.add(egui::Button::new("Optimize")).clicked()
            {
                let results: Vec<(f64, f64, bool)> = (u8::MIN..u8::MAX)
                    .map(|delta| {
                        self.delta = delta;
                        self.corridor.delta(self.delta);
                        self.recalculate(ui.ctx());

                        let max_shannon_criteria: Vec<Option<(usize, f64)>> = self
                            .criterias
                            .iter()
                            .map(|c| c.max_shannon_criteria())
                            .collect();

                        let average_shannon = max_shannon_criteria
                            .iter()
                            .map(|c| c.map_or(0.0, |c| c.1))
                            .sum::<f64>()
                            / max_shannon_criteria.len() as f64;

                        let max_kullback_criteria: Vec<Option<(usize, f64)>> = self
                            .criterias
                            .iter()
                            .map(|c| c.max_kullback_criteria())
                            .collect();

                        let average_kullback = max_kullback_criteria
                            .iter()
                            .map(|c| c.map_or(0.0, |c| c.1))
                            .sum::<f64>()
                            / max_kullback_criteria.len() as f64;

                        let in_working_space = max_shannon_criteria.iter().all(|c| {
                            if let Some(c) = c {
                                let characteristics = &self.criterias
                                    [self.class_manager.selected_class]
                                    .characteristics[c.0];

                                characteristics.d1 > 0.5 && characteristics.d2 > 0.5
                            } else {
                                false
                            }
                        });

                        (average_shannon, average_kullback, in_working_space)
                    })
                    .collect();

                let best_delta = results
                    .iter()
                    .enumerate()
                    .filter(|(_, (_, _, in_working_space))| *in_working_space)
                    .max_by(|(_, (a_shannon, _, _)), (_, (b_shannon, _, _))| {
                        a_shannon.total_cmp(b_shannon)
                    })
                    .map(|(i, _)| i as u8)
                    .unwrap_or(0);

                self.delta = best_delta;
                self.corridor.delta(self.delta);
                self.recalculate(ui.ctx());

                self.optimization_results = Some(OptimizationResults::from(results));
            }
        });

        if self.exam_class_manager.show_controls(ui).changed() {
            self.exam();
        }
    }

    fn add_stats(&self, ui: &mut egui::Ui) {
        ui.add(egui::Label::new(format!(
            "Number of realizations: {}",
            self.class_loader.size.0
        )));
        ui.add(egui::Label::new(format!(
            "Number of attributes: {}",
            self.class_loader.size.1
        )));
        ui.add(egui::Label::new(format!(
            "Number of classes: {}",
            self.class_manager.classes.len()
        )));
        ui.add(egui::Label::new(format!(
            "Number of exam classes: {}",
            self.exam_class_manager.classes.len()
        )));
    }

    fn add_widgets_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            self.add_button("Information", ui);
            self.add_button("Settings", ui);
            self.add_button("Classes", ui);
            self.add_button("Allowances", ui);
            self.add_button("2D", ui);
            self.add_button("Criteria", ui);
            self.add_button("Exam classes", ui);
            self.add_button("Exam results", ui);
            self.add_button("Optimization results", ui);
        });
    }

    fn add_button(&mut self, label: &str, ui: &mut egui::Ui) {
        if ui
            .selectable_label(
                *self.widget_stauses.entry(label.to_string()).or_default(),
                label,
            )
            .clicked()
        {
            *self.widget_stauses.get_mut(label).unwrap() =
                !*self.widget_stauses.get(label).unwrap();
        }
    }
}
