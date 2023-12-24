use eframe::{
    egui::{Context, Label, Ui},
    epaint::ColorImage,
};

use crate::{class_data::TextureData, corridor::Allowances, draw::Show};

#[derive(Default)]
pub struct BinaryRepresentation {
    pub matrices: Vec<TextureData>,
    pub reference_vectors: Vec<TextureData>,
}

impl BinaryRepresentation {
    pub fn new(
        classes: &[TextureData],
        size: (usize, usize),
        allowances: &Allowances,
        ctx: &Context,
    ) -> Self {
        let matrices = Self::calculate_binary_matrices(classes, size, allowances, ctx);
        let reference_vectors = Self::calculate_reference_vectors(&matrices, size, ctx);

        Self {
            matrices,
            reference_vectors,
        }
    }

    fn calculate_binary_matrices(
        classes: &[TextureData],
        size: (usize, usize),
        allowances: &Allowances,
        ctx: &Context,
    ) -> Vec<TextureData> {
        let (attributes, realizations) = size;

        classes
            .iter()
            .enumerate()
            .map(|(i, class)| {
                let key: Vec<u8> = class
                    .bytes
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        let index = i.rem_euclid(attributes);
                        if *x >= allowances.lower[index] && *x <= allowances.upper[index] {
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

                TextureData::new(key, texture)
            })
            .collect()
    }

    fn calculate_reference_vectors(
        matrices: &[TextureData],
        size: (usize, usize),
        ctx: &Context,
    ) -> Vec<TextureData> {
        let (attributes, realizations) = size;

        matrices
            .iter()
            .enumerate()
            .map(|(i, matrix)| {
                let mut vector = Vec::with_capacity(attributes);

                for i in 0..attributes {
                    let mut count = 0;

                    for j in 0..realizations {
                        if matrix.bytes[i + j * attributes] == u8::MAX {
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

                TextureData::new(vector, texture)
            })
            .collect()
    }
}

impl Show for BinaryRepresentation {
    fn show(&self, ui: &mut Ui) {
        ui.add(Label::new("Binary matrices"));

        ui.horizontal(|ui| {
            self.matrices.iter().for_each(|matrix| {
                ui.image((matrix.texture.id(), matrix.texture.size_vec2()));
            });
        });

        ui.add(Label::new("Reference vectors"));

        ui.horizontal(|ui| {
            self.reference_vectors.iter().for_each(|vector| {
                ui.image((vector.texture.id(), vector.texture.size_vec2()));
            });
        });
    }
}
