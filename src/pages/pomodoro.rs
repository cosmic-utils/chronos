
use cosmic::{
    iced::{self, futures::executor::block_on, Length, Padding},
    widget::{self, column, Column},
    Command, Element,
};

#[derive(Debug, Clone)]
pub enum PomodoroMessage {}

pub struct Pomodoro {}

impl Default for Pomodoro {
    // Initialize default
    fn default() -> Self {
        Self {}
    }
}

impl Pomodoro {
    pub fn view<'a>(&'a self) -> Element<'a, PomodoroMessage> {
        let col = widget::column();
        widget::container(col)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .center_y()
            .into()
    }

    pub fn update(&mut self, message: PomodoroMessage) -> Command<crate::app::Message> {
        let mut commands = Vec::new();
        match message {}
        Command::batch(commands)
    }
}