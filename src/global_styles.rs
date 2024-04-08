use iced::{widget::button, Background, Border, Theme};

pub struct CustomHoverTextButton;

impl button::StyleSheet for CustomHoverTextButton {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.extended_palette();

        button::Appearance {
            text_color: palette.background.base.text,
            border: Border::with_radius(4),
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.extended_palette();

        button::Appearance {
            background: Some(Background::from(palette.secondary.base.color)),
            ..self.active(style)
        }
    }
}
