use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use iced::{Vector, theme};
use mlish_core::inline_mod;

inline_mod!(button);
inline_mod!(text);
inline_mod!(container);

const DEFAULT_OFFSET: f32 = 10.0;

#[derive(Default, Clone, Copy)]
pub enum ShadowLevel {
    #[default]
    None,
    Light,
    Strong,
    Custom(f32),
}

impl ShadowLevel {
    fn calculate_offset(&self) -> f32 {
        match self {
            ShadowLevel::None => 0f32,
            ShadowLevel::Light => DEFAULT_OFFSET / 2.0,
            ShadowLevel::Strong => DEFAULT_OFFSET,
            ShadowLevel::Custom(offset) => *offset,
        }
    }

    fn into_shadow(&self, palette: Arc<Palette>) -> iced::Shadow {
        iced::Shadow {
            color: palette.shadow,
            offset: Vector::new(self.calculate_offset(), self.calculate_offset()),
            blur_radius: 0.0,
        }
    }
}

pub struct PalatteInjectionSchema {
    name: String,
    palette: Arc<Palette>,
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

const LIGHT_PALETTE: Palette = Palette {
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

#[derive(Clone)]
pub struct MlishTheme {
    current_palette: Arc<RwLock<PaletteType>>,
    dark_palette: Arc<Palette>,
    light_palette: Arc<Palette>,
    injected_palette: Arc<RwLock<HashMap<uuid::Uuid, PalatteInjectionSchema>>>,
}

impl MlishTheme {
    pub fn new(preference: PaletteType) -> Self {
        Self {
            current_palette: Arc::new(RwLock::new(preference)),
            dark_palette: Arc::new(LIGHT_PALETTE),
            light_palette: Arc::new(LIGHT_PALETTE),
            injected_palette: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn get_palette(&self) -> Arc<Palette> {
        //todo!("remove unwarp");
        match *self.current_palette.read().unwrap() {
            PaletteType::MlishDark => self.dark_palette.clone(),
            PaletteType::MlishLight => self.light_palette.clone(),
            PaletteType::Custom(id) => {
                let injected_palette = self.injected_palette.try_read().unwrap();
                let palette_schema = injected_palette.get(&id);
                palette_schema.unwrap().palette.clone()
            }
        }
    }

    fn set_palette(&self, palette: PaletteType) {
        *self.current_palette.write().unwrap() = palette;
    }
}

impl theme::Base for MlishTheme {
    fn base(&self) -> theme::Style {
        theme::Style {
            text_color: self.get_palette().text,
            background_color: self.get_palette().background,
        }
    }

    fn default(preference: theme::Mode) -> Self {
        match preference {
            theme::Mode::Dark => Self::new(PaletteType::MlishDark),
            theme::Mode::Light | theme::Mode::None => Self::new(PaletteType::MlishLight),
        }
    }

    fn mode(&self) -> theme::Mode {
        theme::Mode::Light
    }

    fn name(&self) -> &str {
        "Mlish"
    }

    fn palette(&self) -> Option<theme::Palette> {
        Some(theme::Palette {
            background: self.get_palette().background,
            text: self.get_palette().text,
            primary: self.get_palette().surface,
            success: self.get_palette().success,
            danger: self.get_palette().fail,
            warning: self.get_palette().warn,
        })
    }
}
