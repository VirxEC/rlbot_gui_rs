use crate::{global_styles::CustomHoverTextButton, Message};
use iced::{
    alignment::{Horizontal, Vertical},
    font::{Family, Weight},
    theme,
    widget::{button, column, container, image, row, text, Space},
    Background, Border, Color, Element, Font, Length, Theme,
};
use iced_aw::{
    menu,
    menu::{Item, StyleSheet},
    menu_bar,
    style::MenuBarStyle,
    BootstrapIcon, BOOTSTRAP_FONT,
};

pub fn content<'a>() -> Element<'a, Message> {
    let more_menu = menu!((button(text("Repair bot pack").horizontal_alignment(Horizontal::Left))
        .padding([5, 10])
        .width(Length::Fill)
        .style(theme::Button::custom(CustomHoverTextButton))
        .on_press(Message::RepairBotPack))(
        button(text("Extra button with a long name").horizontal_alignment(Horizontal::Left))
            .padding([5, 10])
            // .width(Length::Fill)
            .style(theme::Button::custom(CustomHoverTextButton))
            .on_press(Message::RepairBotPack)
    ))
    .width(Length::Shrink);

    container(row![column![row![
        image("assets/imgs/rlbot_logo.png").width(48),
        text(" RLBot")
            .style(Color::WHITE)
            .font(Font {
                family: Family::Name("exo-2"),
                weight: Weight::Bold,
                ..Default::default()
            })
            .size(24),
        Space::with_width(Length::Fill),
        menu_bar!((
            button(row![
                text("More ").size(18).style(Color::WHITE),
                text(BootstrapIcon::CaretDownFill)
                    .size(18)
                    .font(BOOTSTRAP_FONT)
                    .height(Length::Shrink)
                    .style(Color::WHITE),
            ])
            .padding([10, 15])
            .style(theme::Button::custom(DarkSecondaryButton)),
            more_menu
        ))
        .style(|theme: &Theme| menu::Appearance {
            bar_background: Background::Color(Color::BLACK),
            menu_background: Background::Color(Color::WHITE),
            ..theme.appearance(&MenuBarStyle::Default)
        })
    ]
    .align_items(Vertical::Center.into())]])
    .style(container::Appearance {
        background: Some(iced::Background::Color(Color::BLACK)),
        ..Default::default()
    })
    .width(Length::Fill)
    .height(Length::Shrink)
    .padding(14)
    .into()
}

struct DarkSecondaryButton;

impl button::StyleSheet for DarkSecondaryButton {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        let palette = Theme::Dark.extended_palette();

        button::Appearance {
            background: Some(palette.secondary.base.color.into()),
            text_color: palette.secondary.base.text,
            border: Border::with_radius(5),
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        let palette = Theme::Dark.extended_palette();

        button::Appearance {
            background: Some(Background::from(palette.background.strong.color)),
            ..self.active(&Theme::Dark)
        }
    }
}
