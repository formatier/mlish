use eframe::egui::{self, IntoAtoms, Response, Widget, WidgetText, widgets};

#[derive(Clone)]
pub struct Widgets;

impl Widgets {
    pub fn button<'a>(&self, ui: &'a mut egui::Ui, atoms: impl IntoAtoms<'a>) -> Response {
        widgets::Button::new(atoms)
            .corner_radius(20)
            .gap(5f32)
            .ui(ui)
    }

    pub fn label<'a>(&self, ui: &'a mut egui::Ui, text: impl Into<WidgetText>) -> Response {
        widgets::Label::new(text).ui(ui)
    }
}
