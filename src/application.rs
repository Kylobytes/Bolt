use cosmic::{
    app::{Command, Core},
    executor,
    iced::window,
    widget::{column, container, row, text},
    ApplicationExt, Element,
};

#[derive(Default)]
pub struct Application {
    core: Core,
}

#[derive(Clone, Debug)]
pub enum Message {}

impl cosmic::Application for Application {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.kylobytes.Bolt";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(
        core: Core,
        _input: Self::Flags,
    ) -> (Self, Command<Self::Message>) {
        let mut app = Self { core };
        let command: Command<Message> = app.update_title();

        (app, command)
    }

    fn view(&self) -> Element<Self::Message> {
        let content =
            container(row().push(column().push(text("Welcome to Bolt"))));

        Element::from(content)
    }
}

impl Application {
    fn update_title(&mut self) -> Command<Message> {
        let title = "Bolt".to_string();

        self.set_header_title(title.clone());
        self.set_window_title(title.clone(), window::Id::MAIN)
    }
}
