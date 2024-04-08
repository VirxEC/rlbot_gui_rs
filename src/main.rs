mod background;
mod bots;
mod global_styles;
mod header;

use iced::{
    widget::{column, container, Image},
    window, Application, Command, Length, Settings, Size,
};
use std::borrow::Cow;

const EXO_2_FONT_BYTES: &[u8] = include_bytes!("../fonts/exo-2.ttf");

fn main() -> iced::Result {
    MainApp::run(Settings {
        window: window::Settings {
            icon: Some(window::icon::from_file("assets/imgs/rlbot_logo.png").unwrap()),
            size: Size {
                width: 1300.0,
                height: 870.0,
            },
            transparent: true,
            ..Default::default()
        },
        fonts: vec![
            Cow::Borrowed(EXO_2_FONT_BYTES),
            Cow::Borrowed(iced_aw::BOOTSTRAP_FONT_BYTES),
        ],
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    RepairBotPack,
    PromptLoadFolder,
    PromptLoadFile,
}

struct MainApp;

impl Application for MainApp {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = iced::Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("RLBotGUI")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        dbg!(message);

        Command::none()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Light
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let image = Image::new("assets/imgs/arenas/UrbanCentral.jpg")
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(iced::ContentFit::Cover);

        let background = container(image).width(Length::Fill).height(Length::Fill);

        let content = column![header::content(), bots::content()];

        background::Modal::new(background, content).into()
    }
}
