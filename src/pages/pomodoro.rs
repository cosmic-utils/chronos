use cosmic::{
    iced::{self, Length},
    widget::{self, icon},
    Command, Element,
};

use crate::fl;

#[derive(Debug, Clone)]
pub enum PomodoroMessage {}

pub struct Pomodoro {
    slider_value: f32,
}

impl Default for Pomodoro {
    // Initialize default
    fn default() -> Self {
        Self { slider_value: 50. }
    }
}

impl Pomodoro {
    pub fn view<'a>(&'a self) -> Element<'a, PomodoroMessage> {
        let col = widget::column()
            .push(
                widget::text::title1(fl!("pomodoro"))
                    .width(Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .push(
                widget::text::title1("00:15:59")
                    .width(Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .push(widget::vertical_space(Length::from(10)))
            .push(
                widget::progress_bar(0.0..=100.0, self.slider_value)
                    .width(Length::Fixed(250.0))
                    .height(Length::Fixed(4.0)),
            )
            .push(widget::vertical_space(Length::from(10)))
            .push(
                widget::row()
                    .push(
                        widget::button::icon(icon::from_name("media-playback-start-symbolic"))
                            .extra_large()
                            .tooltip("play"),
                    )
                    .push(
                        //TODO se il timer è in funzione invece che lo stop deve esserci il pause
                        widget::button::icon(icon::from_name("media-playback-stop-symbolic"))
                            .extra_large()
                            .tooltip("play"),
                    ),
            )
            .align_items(iced::Alignment::Center);

        widget::container(col)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_y()
            .into()
    }

    pub fn update(&mut self, message: PomodoroMessage) -> Command<crate::app::Message> {
        let mut commands = Vec::new();
        match message {}
        Command::batch(commands)
    }
}
