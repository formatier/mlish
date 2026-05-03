use iced::widget::text;

pub enum TextClass {
    Nornal,
    Title,
}

impl text::Catalog for super::MlishTheme {
    type Class<'a> = TextClass;

    fn default<'a>() -> Self::Class<'a> {
        TextClass::Nornal
    }

    fn style(&self, item: &Self::Class<'_>) -> text::Style {
        let palette = self.get_palette();

        match item {
            TextClass::Nornal => text::Style {
                color: Some(palette.text),
            },
            TextClass::Title => text::Style {
                color: Some(palette.text_alternate),
            },
        }
    }
}
