use crate::{global_styles::CustomHoverTextButton, Message, EXO_2_FONT};
use iced::{
    alignment::{Horizontal, Vertical},
    border::Radius,
    font::Weight,
    theme,
    widget::{button, column, container, row, scrollable, text, Space},
    Alignment, Background, Border, Color, Length,
};
use iced_aw::{menu, menu::Item, menu_bar, BootstrapIcon, BOOTSTRAP_FONT};

pub fn content() -> iced::Element<'static, Message> {
    let add_menu = menu!((button(row![
            text(BootstrapIcon::FolderPlus)
                .size(18)
                .font(BOOTSTRAP_FONT)
                .height(Length::Shrink),
            text(" Load folder").size(18).horizontal_alignment(Horizontal::Left),
        ].align_items(Alignment::Center))
        .padding([5, 10])
        // .width(Length::Fill)
        .style(theme::Button::custom(CustomHoverTextButton))
        .on_press(Message::PromptLoadFolder))(
        button(
            row![
                text(BootstrapIcon::FileEarmarkPlus)
                    .size(18)
                    .font(BOOTSTRAP_FONT)
                    .height(Length::Shrink),
                text(" Load file").size(18).horizontal_alignment(Horizontal::Left)
            ]
            .align_items(Alignment::Center)
        )
        .padding([5, 10])
        .width(Length::Fill)
        .style(theme::Button::custom(CustomHoverTextButton))
        .on_press(Message::PromptLoadFile)
    ))
    .width(Length::Shrink);

    container(
        container(column![
            row![
                text("Bots").size(24).vertical_alignment(Vertical::Center).font({
                    let mut font = EXO_2_FONT;
                    font.weight = Weight::Bold;
                    font
                }),
                Space::with_width(20),
                menu_bar!((
                    button(
                        row![
                            text(BootstrapIcon::PlusLg)
                                .size(20)
                                .font(BOOTSTRAP_FONT)
                                .style(Color::BLACK)
                                .height(Length::Shrink),
                            text(" Add ").size(20).style(Color::BLACK),
                            text(BootstrapIcon::CaretDownFill)
                                .size(20)
                                .font(BOOTSTRAP_FONT)
                                .height(Length::Shrink)
                                .style(Color::BLACK),
                        ]
                        .align_items(Alignment::Center)
                    )
                    .style(theme::Button::Secondary)
                    .padding(10),
                    add_menu
                )),
                Space::with_width(20),
                button(
                    row![
                        text(BootstrapIcon::Gear)
                            .size(20)
                            .font(BOOTSTRAP_FONT)
                            .style(Color::BLACK)
                            .height(Length::Shrink),
                        text(" Manage Bot Settings").size(20).style(Color::BLACK),
                    ]
                    .align_items(Alignment::Center)
                )
                .on_press(Message::ManageBotSettings)
                .style(theme::Button::Secondary)
                .padding(10)
            ]
            .padding(10)
            .align_items(Alignment::Center),
            scrollable(row![])
        ])
        .style(theme::Container::from(container::Appearance {
            background: Some(Background::Color(Color::WHITE)),
            border: Border {
                color: Color::WHITE,
                radius: Radius::from(5.),
                width: 1.,
            },
            ..Default::default()
        }))
        .center_y()
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([15, 40])
    .into()
}
