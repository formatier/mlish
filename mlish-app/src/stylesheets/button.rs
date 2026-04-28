use iced::{Vector, padding, widget::button};

#[derive(Default)]
pub enum Button {
    #[default]
    Normal {
        shadow: Option<ButtonShadow>,
        level: Option<ButtonLevel>,
    },
    Success {
        shadow: Option<ButtonShadow>,
        level: Option<ButtonLevel>,
    },
    Warn {
        shadow: Option<ButtonShadow>,
        level: Option<ButtonLevel>,
    },
    Fail {
        shadow: Option<ButtonShadow>,
        level: Option<ButtonLevel>,
    },
    Disable,
}

const DEFAULT_OFFSET: f32 = 4.0;

#[derive(Default)]
pub enum ButtonShadow {
    #[default]
    None,
    Light(Option<f32>),
    Strong(Option<f32>),
}

impl ButtonShadow {
    fn calculate_offset(&self) -> f32 {
        match self {
            ButtonShadow::None => 0f32,
            ButtonShadow::Light(offset) => offset.map_or(DEFAULT_OFFSET, |offset| offset / 2),
            ButtonShadow::Strong(offset) => offset.map_or(DEFAULT_OFFSET, |offset| offset),
        }
    }
}

#[derive(Default)]
pub enum ButtonLevel {
    #[default]
    Surface,
    SurfaceAlternate,
}

impl button::Catalog for super::MlishTheme {
    type Class<'a> = Button;

    fn default<'a>() -> Self::Class<'a> {
        Button::Normal
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        let mut style = button::Style::default();

        match class {
            Button::Normal { shadow, level } => {
                if let Some(shadow) = shadow {
                    style.shadow = iced::Shadow {
                        color: self.get_palette().shadow,
                        offset: Vector::new(shadow.calculate_offset(), -shadow.calculate_offset()),
                        blur_radius: 0.0,
                    };
                }

                style.with_background(level.map_or(
                    self.get_palette().surface,
                    |level| match level {
                        ButtonLevel::Surface => self.get_palette().surface,
                        ButtonLevel::SurfaceAlternate => self.get_palette().surface_alternate,
                    },
                ));
            }
            Button::Success { shadow, level } => {
                if let Some(shadow) = shadow {
                    style.shadow = iced::Shadow {
                        color: self.get_palette().shadow,
                        offset: Vector::new(shadow.calculate_offset(), -shadow.calculate_offset()),
                        blur_radius: 0.0,
                    };
                }

                style.with_background(level.map_or(
                    self.get_palette().success,
                    |level| match level {
                        ButtonLevel::Surface => self.get_palette().success,
                        ButtonLevel::SurfaceAlternate => self.get_palette().success_alternate,
                    },
                ));
            }
            Button::Warn { shadow, level } => {
                if let Some(shadow) = shadow {
                    style.shadow = iced::Shadow {
                        color: self.get_palette().shadow,
                        offset: Vector::new(shadow.calculate_offset(), -shadow.calculate_offset()),
                        blur_radius: 0.0,
                    };
                }

                style.with_background(level.map_or(self.get_palette().warn, |level| match level {
                    ButtonLevel::Surface => self.get_palette().warn,
                    ButtonLevel::SurfaceAlternate => self.get_palette().warn_alternate,
                }));
            }
            Button::Fail { shadow, level } => {
                if let Some(shadow) = shadow {
                    style.shadow = iced::Shadow {
                        color: self.get_palette().shadow,
                        offset: Vector::new(shadow.calculate_offset(), -shadow.calculate_offset()),
                        blur_radius: 0.0,
                    };
                }

                style.with_background(level.map_or(self.get_palette().fail, |level| match level {
                    ButtonLevel::Surface => self.get_palette().fail,
                    ButtonLevel::SurfaceAlternate => self.get_palette().fail_alternate,
                }));
            }
            Button::Disable => {
                style.with_background(self.get_palette().disable);
            }
        };

        style
    }
}
