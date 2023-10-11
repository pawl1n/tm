use eframe::egui::TextureHandle;

pub struct ClassData {
    bytes: Vec<u8>,
    texture: TextureHandle,
}

impl ClassData {
    pub fn new(bytes: Vec<u8>, texture: TextureHandle) -> Self {
        Self { bytes, texture }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn texture(&self) -> &TextureHandle {
        &self.texture
    }
}
