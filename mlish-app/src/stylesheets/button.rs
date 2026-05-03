use iced::{Vector, widget::button};

use crate::stylesheets::ShadowLevel;

pub enum ButtonClass {
    Normal {
        shadow: Option<ShadowLevel>,
        level: Option<ButtonLevel>,
    },
    Success {
        shadow: Option<ShadowLevel>,
        level: Option<ButtonLevel>,
    },
    Warn {
        shadow: Option<ShadowLevel>,
        level: Option<ButtonLevel>,
    },
    Fail {
        shadow: Option<ShadowLevel>,
        level: Option<ButtonLevel>,
    },
    Disable,
}

impl Default for ButtonClass {
    fn default() -> Self {
        Self::Normal {
            shadow: None,
            level: None,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum ButtonLevel {
    #[default]
    Surface,
    SurfaceAlternate,
}

impl button::Catalog for super::MlishTheme {
    type Class<'a> = ButtonClass;

    fn default<'a>() -> Self::Class<'a> {
        ButtonClass::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        let palette = self.get_palette();
        let mut style = button::Style::default();

        match class {
            ButtonClass::Normal { shadow, level } => {
                if let Some(shadow) = shadow {
                    style.shadow = shadow.into_shadow(palette.clone());
                }

                style.background = Some(
                    level
                        .map_or(palette.surface, |level| match level {
                            ButtonLevel::Surface => palette.surface,
                            ButtonLevel::SurfaceAlternate => palette.surface_alternate,
                        })
                        .into(),
                );
            }
            ButtonClass::Success { shadow, level } => {
                if let Some(shadow) = shadow {
                    style.shadow = shadow.into_shadow(palette.clone());
                }

                style.background = Some(
                    level
                        .map_or(palette.surface, |level| match level {
                            ButtonLevel::Surface => palette.surface,
                            ButtonLevel::SurfaceAlternate => palette.surface_alternate,
                        })
                        .into(),
                );
            }
            ButtonClass::Warn { shadow, level } => {
                if let Some(shadow) = shadow {
                    style.shadow = shadow.into_shadow(palette.clone());
                }

                style.background = Some(
                    level
                        .map_or(palette.surface, |level| match level {
                            ButtonLevel::Surface => palette.surface,
                            ButtonLevel::SurfaceAlternate => palette.surface_alternate,
                        })
                        .into(),
                );
            }
            ButtonClass::Fail { shadow, level } => {
                if let Some(shadow) = shadow {
                    style.shadow = shadow.into_shadow(palette.clone());
                }

                style.background = Some(
                    level
                        .map_or(palette.surface, |level| match level {
                            ButtonLevel::Surface => palette.surface,
                            ButtonLevel::SurfaceAlternate => palette.surface_alternate,
                        })
                        .into(),
                );
            }
            ButtonClass::Disable => {
                style.background = Some(palette.disable.into());
            }
        };

        style
    }
}
