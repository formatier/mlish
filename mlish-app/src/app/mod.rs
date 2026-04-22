use std::sync::Arc;

use crate::{generated, ui};
use eframe::egui::{CentralPanel, FontData, FontDefinitions, FontId, Style};

pub struct MlishApp {
    counter: u32,
    widgets: ui::Widgets,
}

impl MlishApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let mut font_definitions = FontDefinitions::default();
        generated::font::get_font().iter().for_each(|font| {
            let font_data = FontData::from_static(font.font_data);
            font_definitions
                .font_data
                .insert(font.name.clone(), Arc::new(font_data));
        });
        cc.egui_ctx.set_fonts(font_definitions);

        cc.egui_ctx.set_global_style(Arc::new(Style {
            override_font_id: Some(FontId::new(
                35f32,
                eframe::egui::FontFamily::Name("Kanit-Medium".into()),
            )),
            ..Default::default()
        }));

        Self {
            counter: 0,
            widgets: ui::Widgets,
        }
    }
}

impl eframe::App for MlishApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            let res = self
                .widgets
                .button(ui, format_args!("Counter {}", self.counter).to_string());
            if res.clicked() {
                self.counter += 1;
            }
        });
    }
}
