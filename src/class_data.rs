use eframe::egui::TextureHandle;

#[derive(Clone)]
pub struct TextureData {
    pub bytes: Vec<u8>,
    pub texture: TextureHandle,
}

impl TextureData {
    pub fn new(bytes: Vec<u8>, texture: TextureHandle) -> Self {
        Self { bytes, texture }
    }
}
