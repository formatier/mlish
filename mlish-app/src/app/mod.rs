use iced::{
    Element,
    application::IntoBoot,
    widget::{button, column, text},
};

#[derive(Clone)]
pub enum AppMessage {
    In,
    De,
}

#[derive(Default)]
pub struct App {
    counter: i32,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        column![
            button("In").on_press(AppMessage::In),
            text(format!("Counter: {}", self.counter)),
            button("De").on_press(AppMessage::De),
        ]
        .into()
    }

    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::In => self.counter += 1,
            AppMessage::De => self.counter -= 1,
        }
    }
}
