use std::usize;

use eframe::egui::{Context, Label, RadioButton, Ui};

use crate::binary_representation::BinaryRepresentation;
use crate::class_data::TextureData;
use crate::corridor::Allowances;
use crate::draw::Show;

#[derive(Default)]
pub struct ClassManagerControlsResponse {
    changed: bool,
}

impl ClassManagerControlsResponse {
    pub fn new() -> Self {
        Self { changed: false }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }
}

#[derive(Default)]
pub struct ClassManager {
    pub selected_class: usize,
    pub classes: Vec<TextureData>,
    pub binary_representations: BinaryRepresentation,
}

impl ClassManager {
    pub fn add_class(&mut self, data: TextureData) {
        if self.classes.is_empty() || self.classes[0].texture.size() == data.texture.size() {
            self.classes.push(data);
        }
    }

    pub fn recalculate_binary_representation(&mut self, allowances: &Allowances, ctx: &Context) {
        if !self.classes.is_empty() {
            self.binary_representations = BinaryRepresentation::new(
                &self.classes,
                self.classes[0].texture.size().into(),
                allowances,
                ctx,
            );
        }
    }

    pub fn show_controls(&mut self, ui: &mut Ui) -> ClassManagerControlsResponse {
        let mut response = ClassManagerControlsResponse::new();

        if self.selected_class >= self.classes.len() {
            self.selected_class = 0;
        }

        if self.classes.is_empty() {
            return response;
        }

        let selected = &self.classes[self.selected_class];
        ui.add(Label::new("Selected class"));
        ui.image((selected.texture.id(), selected.texture.size_vec2()));
        ui.add(Label::new("Select class:"));
        ui.horizontal_wrapped(|ui| {
            (0..self.classes.len()).for_each(|i| {
                if ui
                    .add(RadioButton::new(self.selected_class == i, i.to_string()))
                    .clicked()
                {
                    self.selected_class = i;
                    response.changed = true;
                }
            });
        });

        if ui.button("Delete").clicked() {
            if self.classes.len() > self.selected_class {
                self.classes.remove(self.selected_class);
            }

            response.changed = true;
            self.selected_class = 0;
        }

        response
    }
}

impl Show for ClassManager {
    fn show(&self, ui: &mut Ui) {
        ui.add(Label::new("Classes"));

        ui.horizontal(|ui| {
            self.classes.iter().for_each(|class| {
                ui.image((class.texture.id(), class.texture.size_vec2()));
            });
        });
    }
}
