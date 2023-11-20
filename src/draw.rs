use eframe::egui::Ui;

pub trait Draw {
    fn draw(&self, ui: &mut Ui);
}
