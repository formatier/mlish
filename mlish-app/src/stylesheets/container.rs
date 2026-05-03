use std::sync::Arc;

use iced::{Shadow, border::Radius, widget::container};

#[derive(Clone, Copy)]
pub enum ContainerClass {
    Normal {
        shadow: Option<super::ShadowLevel>,
        border: Option<ContainerBorderColor>,
        corner: Option<ContainerCornerRadius>,
    },
    Alternate {
        shadow: Option<super::ShadowLevel>,
        border: Option<ContainerBorderColor>,
        corner: Option<ContainerCornerRadius>,
    },
}

#[derive(Clone, Copy)]
pub enum ContainerBorderColor {
    Normal,
    Alternate,
}

impl ContainerBorderColor {
    pub fn into_color(&self, palette: Arc<super::Palette>) -> iced::Color {
        match self {
            ContainerBorderColor::Normal => palette.text,
            ContainerBorderColor::Alternate => palette.text_alternate,
        }
    }
}

const DEFAULT_CORNER_RADIUS: f32 = 8.0;

#[derive(Clone, Copy)]
pub enum ContainerCornerRadius {
    Normal,
    SlightlyRounded,
    CustomEveryCorner(f32),
    Custom(f32, f32, f32, f32),
}

impl ContainerCornerRadius {
    pub fn into_radius(&self) -> Radius {
        match self {
            ContainerCornerRadius::Normal => Radius::new(0),
            ContainerCornerRadius::SlightlyRounded => Radius::new(DEFAULT_CORNER_RADIUS),
            ContainerCornerRadius::CustomEveryCorner(r) => Radius::new(*r),
            ContainerCornerRadius::Custom(tl, tr, bl, br) => Radius::default()
                .top_left(*tl)
                .top_right(*tr)
                .bottom_left(*bl)
                .bottom_right(*br),
        }
    }
}

impl container::Catalog for super::MlishTheme {
    type Class<'a> = ContainerClass;

    fn default<'a>() -> Self::Class<'a> {
        ContainerClass::Normal {
            shadow: Some(super::ShadowLevel::None),
            border: None,
            corner: None,
        }
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        let palette = self.get_palette();
        let mut style = container::Style::default();

        match class {
            ContainerClass::Normal {
                shadow,
                border,
                corner,
            } => {
                style.shadow = shadow.map_or(Shadow::default(), |s| s.into_shadow(palette.clone()));
                style.border.color =
                    border.map_or(palette.surface, |b| b.into_color(palette.clone()));
                style.border.radius = corner.map_or(Radius::default(), |c| c.into_radius());
                style.background = Some(palette.surface.into());
            }
            ContainerClass::Alternate {
                shadow,
                border,
                corner,
            } => {
                style.shadow = shadow.map_or(Shadow::default(), |s| s.into_shadow(palette.clone()));
                style.border.color =
                    border.map_or(palette.surface, |b| b.into_color(palette.clone()));
                style.border.radius = corner.map_or(Radius::default(), |c| c.into_radius());
                style.background = Some(palette.surface_alternate.into());
            }
        }

        style
    }
}
