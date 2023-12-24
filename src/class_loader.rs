use crate::class_manager::ClassManager;

use super::class_data::TextureData;
use eframe::egui::{Button, Context, Label, RadioButton, TextEdit, Ui};
use eframe::epaint::ColorImage;
use image::{open, EncodableLayout};

#[derive(PartialEq)]
pub enum ClassType {
    Training,
    Exam,
}

impl Default for ClassType {
    fn default() -> Self {
        Self::Training
    }
}

#[derive(Default)]
pub struct LoaderResponse {
    pub loaded: Option<TextureData>,
}

impl LoaderResponse {
    pub fn new(loaded: Option<TextureData>) -> Self {
        Self { loaded }
    }

    pub fn loaded(&self) -> &Option<TextureData> {
        &self.loaded
    }
}

#[derive(Default)]
pub struct ClassLoader {
    pub size: (usize, usize),
    pub class_type: ClassType,
    pub error: Option<String>,
    pub path: String,
}

impl ClassLoader {
    pub fn show(
        &mut self,
        manager: &ClassManager,
        exam_manager: &ClassManager,
        ui: &mut Ui,
    ) -> LoaderResponse {
        let mut response = LoaderResponse::default();

        ui.horizontal(|ui| {
            ui.add(TextEdit::singleline(&mut self.path));

            if ui.add(Button::new("Load class")).clicked() {
                match self.load_grayscale(manager, exam_manager, ui.ctx()) {
                    Err(msg) => {
                        self.error = Some(msg.to_string());
                    }
                    Ok(class) => {
                        self.error = None;
                        response = LoaderResponse::new(Some(class));
                    }
                }
            };

            if ui
                .add(RadioButton::new(
                    self.class_type == ClassType::Training,
                    "Training class",
                ))
                .clicked()
            {
                self.class_type = ClassType::Training;
            }

            if ui
                .add(RadioButton::new(
                    self.class_type == ClassType::Exam,
                    "Exam class",
                ))
                .clicked()
            {
                self.class_type = ClassType::Exam;
            }

            if let Some(message) = &self.error {
                ui.add(Label::new(message));

                if ui.add(Button::new("x")).clicked() {
                    self.error = None;
                }
            }
        });

        response
    }

    fn load_grayscale(
        &mut self,
        class_manager: &ClassManager,
        exam_manager: &ClassManager,
        ctx: &Context,
    ) -> Result<TextureData, String> {
        let luma = open(&self.path).map_err(|err| err.to_string())?.to_luma8();
        let vec = luma.to_vec();

        let manager = match self.class_type {
            ClassType::Training => class_manager,
            ClassType::Exam => exam_manager,
        };

        if manager.classes.iter().any(|x| x.bytes.eq(&vec)) {
            return Err("This class has already been loaded".to_owned());
        }

        if manager.classes.is_empty() {
            self.size = (luma.width() as usize, luma.height() as usize);
        } else if self.size != (luma.width() as usize, luma.height() as usize) {
            return Err(
                "Error: Classes should have the same number of realizations and attributes"
                    .to_owned(),
            );
        }

        let image = ColorImage::from_gray(
            [luma.width() as usize, luma.height() as usize],
            luma.as_bytes(),
        );

        let texture = ctx.load_texture(&self.path, image, Default::default());

        Ok(TextureData::new(vec, texture))
    }
}
