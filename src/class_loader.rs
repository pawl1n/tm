use super::class_data::ClassData;
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
    loaded: bool,
}

impl LoaderResponse {
    pub fn new(loaded: bool) -> Self {
        Self { loaded }
    }

    pub fn loaded(&self) -> bool {
        self.loaded
    }
}

#[derive(Default)]
pub struct ClassLoader {
    pub size: Option<(usize, usize)>,
    pub classes: Vec<ClassData>,
    class_type: ClassType,
    error: Option<String>,
    path: String,
    pub exam_classes: Vec<ClassData>,
}

impl ClassLoader {
    pub fn show(&mut self, ui: &mut Ui) -> LoaderResponse {
        let mut response = LoaderResponse::default();

        ui.horizontal(|ui| {
            ui.add(TextEdit::singleline(&mut self.path));

            if ui.add(Button::new("Load image")).clicked() {
                match self.load_grayscale(ui.ctx()) {
                    Err(msg) => {
                        self.error = Some(msg.to_string());
                    }
                    Ok(class) => {
                        self.error = None;
                        self.path.clear();

                        match self.class_type {
                            ClassType::Training => {
                                self.classes.push(class);
                                response = LoaderResponse::new(true);
                            }
                            ClassType::Exam => {
                                self.exam_classes.push(class);
                                response = LoaderResponse::new(true);
                            }
                        }
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

    fn load_grayscale(&mut self, ctx: &Context) -> Result<ClassData, String> {
        let luma = open(&self.path).map_err(|err| err.to_string())?.to_luma8();
        let vec = luma.to_vec();

        let classes = match self.class_type {
            ClassType::Training => &self.classes,
            ClassType::Exam => &self.exam_classes,
        };

        if classes.iter().any(|x| x.bytes().eq(&vec)) {
            return Err("This image has already been loaded".to_owned());
        }

        if self.size.is_none() {
            self.size = Some((luma.width() as usize, luma.height() as usize));
        } else if self.size.unwrap() != (luma.width() as usize, luma.height() as usize) {
            return Err("Error: Images should be of the same format".to_owned());
        }

        let image = ColorImage::from_gray(
            [luma.width() as usize, luma.height() as usize],
            luma.as_bytes(),
        );

        let texture = ctx.load_texture(&self.path, image, Default::default());

        Ok(ClassData::new(vec, texture))
    }
}
