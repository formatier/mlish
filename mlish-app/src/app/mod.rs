use iced::{
    Element,
    widget::{button, center, row, text},
};

use crate::stylesheets::{ButtonClass, ButtonLevel, MlishTheme, PaletteType, ShadowLevel};

#[derive(Clone)]
pub enum AppMessage {
    In,
    De,
}

pub struct App {
    counter: i32,
    theme: MlishTheme,
}

impl App {
    pub fn new() -> Self {
        Self {
            counter: 0,
            theme: MlishTheme::new(PaletteType::MlishLight),
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage, MlishTheme> {
        row![
            button("In")
                .class(ButtonClass::Normal {
                    shadow: Some(ShadowLevel::Light),
                    level: Some(ButtonLevel::Surface),
                })
                .on_press(AppMessage::In),
            text(format!("Counter: {}", self.counter)),
            button(center("Decrement"))
                .class(ButtonClass::Normal {
                    shadow: Some(ShadowLevel::Strong),
                    level: Some(ButtonLevel::Surface),
                })
                .height(48.0)
                .on_press(AppMessage::De),
        ]
        .into()
    }

    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::In => self.counter += 1,
            AppMessage::De => self.counter -= 1,
        }
    }

    pub fn theme(&self) -> Option<MlishTheme> {
        Some(self.theme.clone())
    }

    pub fn title(&self) -> String {
        "Mlish".into()
    }
}
