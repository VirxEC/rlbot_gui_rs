mod background;
mod bots;
mod global_styles;
mod header;

use directories::BaseDirs;
use iced::{
    widget::{column, container, Image},
    window, Application, Command, Font, Length, Settings, Size,
};
use native_dialog::FileDialog;
use once_cell::sync::Lazy;
use std::{borrow::Cow, path::PathBuf};

const EXO_2_FONT: Font = Font::with_name("Exo 2");
const EXO_2_FONT_BYTES: &[u8] = include_bytes!("../fonts/exo-2.ttf");

// To minimize sys calls, we store these directories in this lazy static
static BASE_DIRS: Lazy<BaseDirs> = Lazy::new(|| BaseDirs::new().unwrap());

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
    LoadFolder(Option<PathBuf>),
    LoadFile(Option<PathBuf>),
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
        match dbg!(message) {
            Message::RepairBotPack => Command::none(),
            Message::PromptLoadFolder => Command::perform(prompt_load_folder(), Message::LoadFolder),
            Message::PromptLoadFile => Command::perform(prompt_load_file(), Message::LoadFile),
            Message::LoadFolder(path) => {
                dbg!(path);
                Command::none()
            }
            Message::LoadFile(path) => {
                dbg!(path);
                Command::none()
            }
        }
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

async fn prompt_load_folder() -> Option<PathBuf> {
    FileDialog::new()
        .set_location(BASE_DIRS.home_dir())
        .set_title("Select folder of bots to load")
        .show_open_single_dir()
        .inspect_err(|err| println!("Error opening native dialog: {err}"))
        .ok()
        .flatten()
}

async fn prompt_load_file() -> Option<PathBuf> {
    FileDialog::new()
        .set_location(BASE_DIRS.home_dir())
        .add_filter("Bot config or custom map", &["toml", "upk"])
        .set_title("Select bot config or map to load")
        .show_open_single_file()
        .inspect_err(|err| println!("Error opening native dialog: {err}"))
        .ok()
        .flatten()
}
