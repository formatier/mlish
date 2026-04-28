use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, RwLock},
};

mod button;

pub struct PalatteInjectionSchema {
    name: String,
    palette: Palette,
}

#[derive(Debug, Copy, Clone)]
pub struct Palette {
    pub background: iced::Color,
    pub surface: iced::Color,
    pub surface_alternate: iced::Color,
    pub success: iced::Color,
    pub success_alternate: iced::Color,
    pub fail: iced::Color,
    pub fail_alternate: iced::Color,
    pub warn: iced::Color,
    pub warn_alternate: iced::Color,
    pub disable: iced::Color,
    pub text: iced::Color,
    pub text_alternate: iced::Color,
    pub highlight: iced::Color,
    pub shadow: iced::Color,
}

const DARK_PALETTE: Palette = Palette {
    background: iced::Color::from_rgb8(254, 249, 239),
    surface: iced::Color::from_rgb8(254, 236, 199),
    surface_alternate: iced::Color::from_rgb8(254, 215, 135),
    success: iced::Color::from_rgb8(221, 225, 149),
    success_alternate: iced::Color::from_rgb8(186, 243, 65),
    fail: iced::Color::from_rgb8(255, 155, 148),
    fail_alternate: iced::Color::from_rgb8(243, 65, 65),
    warn: iced::Color::from_rgb8(252, 170, 59),
    warn_alternate: iced::Color::from_rgb8(235, 115, 11),
    disable: iced::Color::from_rgb8(225, 217, 200),
    text: iced::Color::from_rgb8(143, 92, 77),
    text_alternate: iced::Color::from_rgb8(70, 17, 2),
    highlight: iced::Color::from_rgb8(255, 206, 104),
    shadow: iced::Color::from_rgb8(59, 58, 54),
};

pub enum PaletteType {
    MlishDark,
    MlishLight,
    Custom(uuid::Uuid),
}

pub struct MlishTheme {
    current_palette: Arc<RwLock<PaletteType>>,
    dark_palette: Palette,
    light_palette: Palette,
    injected_palette: Arc<RwLock<HashMap<uuid::Uuid, PalatteInjectionSchema>>>,
}

impl MlishTheme {
    fn get_palette(&self) -> Palette {
        match *self.current_palette.borrow() {
            PaletteType::MlishDark => self.dark_palette,
            PaletteType::MlishLight => self.light_palette,
            PaletteType::Custom(id) => {
                let injected_palette = self.injected_palette.try_read();
                let palette_schema = injected_palette.get(&id);
                palette_schema.unwrap().palette
            }
        }
    }
}

impl Default for MlishTheme {
    // TODO: Add light palette
    fn default() -> Self {
        Self {
            current_palette: Arc::new(RwLock::new(PaletteType::MlishLight)),
            dark_palette: DARK_PALETTE,
            light_palette: DARK_PALETTE,
            injected_palette: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
