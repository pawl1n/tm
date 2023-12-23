use eframe::egui::Ui;

pub trait Show {
    fn show(&self, ui: &mut Ui);
}
