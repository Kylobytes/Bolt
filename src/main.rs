use cosmic::app::Settings;

use crate::application::Application;

mod application;

#[tokio::main]
async fn main() -> cosmic::iced::Result {
    cosmic::app::run::<Application>(Settings::default(), ())
}
