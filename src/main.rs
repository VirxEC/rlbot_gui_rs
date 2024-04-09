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
use std::{borrow::Cow, path::PathBuf};
use tokio::fs;
use toml_edit::{Array, ArrayOfTables, DocumentMut, Item, Table, Value};

const EXO_2_FONT: Font = Font::with_name("Exo 2");
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
    ConfigLoaded(DocumentMut),
    ConfigSaved(()),
    RepairBotPack,
    PromptLoadFolder,
    PromptLoadFile,
    LoadFolder(Option<PathBuf>),
    LoadFile(Option<PathBuf>),
    FolderLoaded(()),
}

struct MainApp {
    base_dirs: BaseDirs,
    config_folder: PathBuf,
    config_file: PathBuf,
    config: DocumentMut,
}

impl MainApp {
    fn get_or_insert_array(&mut self, key: &str) -> &mut Array {
        let item = self.config.entry(key).or_insert(Item::None);

        match item {
            Item::Value(Value::Array(array)) => array,
            _ => {
                *item = Item::Value(Value::Array(Array::new()));
                item.as_value_mut().and_then(Value::as_array_mut).unwrap()
            }
        }
    }
}

impl Application for MainApp {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = iced::Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let base_dirs = BaseDirs::new().unwrap();

        let config_folder = base_dirs.config_dir().join("RLBotGUIX");
        let config_file = config_folder.join("config.toml");
        let config = DocumentMut::new();

        (
            Self {
                base_dirs,
                config_folder: config_folder.clone(),
                config_file: config_file.clone(),
                config,
            },
            Command::perform(load_config(config_folder, config_file), Message::ConfigLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("RLBotGUI")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match dbg!(message) {
            Message::ConfigLoaded(config) => {
                self.config = config;
                Command::none()
            }
            Message::RepairBotPack => Command::none(),
            Message::PromptLoadFolder => Command::perform(
                prompt_load_folder(self.base_dirs.home_dir().to_path_buf()),
                Message::LoadFolder,
            ),
            Message::PromptLoadFile => Command::perform(
                prompt_load_file(self.base_dirs.home_dir().to_path_buf()),
                Message::LoadFile,
            ),
            Message::LoadFolder(path) => {
                if let Some(path) = path {
                    let folders = self.get_or_insert_array("folders");
                    folders.push(path.to_string_lossy().to_string());

                    // we can't reference self in the closure due to lifetime rules
                    let parent = self.config_folder.clone();
                    let config_path = self.config_file.clone();
                    let config = self.config.clone();

                    Command::batch([
                        Command::perform(
                            async move { save_config(parent, config_path, &config).await },
                            Message::ConfigSaved,
                        ),
                        Command::perform(load_folder(path), Message::FolderLoaded),
                    ])
                } else {
                    Command::none()
                }
            }
            Message::LoadFile(path) => {
                dbg!(path);
                Command::none()
            }
            Message::FolderLoaded(()) => Command::none(),
            Message::ConfigSaved(()) => Command::none(),
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

async fn prompt_load_folder(home_dir: PathBuf) -> Option<PathBuf> {
    FileDialog::new()
        .set_location(&home_dir)
        .set_title("Select folder of bots to load")
        .show_open_single_dir()
        .inspect_err(|err| println!("Error opening native dialog: {err}"))
        .ok()
        .flatten()
}

async fn prompt_load_file(home_dir: PathBuf) -> Option<PathBuf> {
    FileDialog::new()
        .set_location(&home_dir)
        .add_filter("Bot config or custom map", &["toml", "upk"])
        .set_title("Select bot config or map to load")
        .show_open_single_file()
        .inspect_err(|err| println!("Error opening native dialog: {err}"))
        .ok()
        .flatten()
}

async fn load_folder(path: PathBuf) {
    dbg!(path);
}

async fn load_config(parent: PathBuf, path: PathBuf) -> DocumentMut {
    match fs::read_to_string(&path).await {
        Ok(content) => content.parse().unwrap_or_default(),
        Err(_) => {
            let config = DocumentMut::new();

            // the config doesn't exist, so we'll create it
            save_config(parent, path, &config).await;

            config
        }
    }
}

async fn save_config(parent: PathBuf, path: PathBuf, config: &DocumentMut) {
    fs::create_dir_all(parent).await.unwrap();
    fs::write(path, config.to_string()).await.unwrap();
}
