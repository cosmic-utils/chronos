use cosmic::{
    iced::{self, Length},
    widget::{self, icon},
    Command, Element,
};
use notify_rust::Notification;

use crate::{app::Message, config::Config, fl};

#[derive(Debug, Clone)]
pub enum PomodoroMessage {
    PomodoroTick,
    StartPomodoro,
    PausePomodoro,
    ResetPomodoro,
}

pub struct Pomodoro {
    in_action: bool,
    slider_max_value: f32,
    slider_value: f32,
    timer_duration: f32,
    pause_duration: f32,
    long_pause_duration: f32,
    pomodoro_completed: u32,
    pomodoro_before_long_pause: u32,
}

impl Default for Pomodoro {
    // Initialize default
    fn default() -> Self {
        let config = Config::load().1;
        let seconds = config.timer_duration * 60;
        Self {
            in_action: false,
            slider_value: seconds as f32,
            slider_max_value: seconds as f32,
            timer_duration: config.timer_duration as f32,
            pause_duration: config.pause_duration as f32,
            long_pause_duration: config.long_pause_duration as f32,
            pomodoro_completed: 0,
            pomodoro_before_long_pause: config.pomodoro_before_long_pause,
        }
    }
}

impl Pomodoro {
    pub fn view<'a>(&'a self) -> Element<'a, PomodoroMessage> {
        let col = widget::column()
            .push(
                widget::text::title1(if self.in_action {
                    fl!("pomodoro")
                } else {
                    fl!("pause")
                })
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .push(
                widget::text::title1(self.format_slider_value())
                    .width(Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .push(widget::vertical_space(Length::from(10)))
            .push(
                widget::progress_bar(0.0..=self.slider_max_value, self.slider_value)
                    .width(Length::Fixed(250.0))
                    .height(Length::Fixed(4.0)),
            )
            .push(widget::vertical_space(Length::from(10)))
            .push(
                widget::row()
                    .push(
                        widget::button::icon(icon::from_name("media-playback-start-symbolic"))
                            .extra_large()
                            .on_press(PomodoroMessage::StartPomodoro),
                    )
                    .push(if self.in_action {
                        widget::button::icon(icon::from_name("media-playback-pause-symbolic"))
                            .extra_large()
                            .on_press(PomodoroMessage::PausePomodoro)
                    } else {
                        widget::button::icon(icon::from_name("media-playback-stop-symbolic"))
                            .extra_large()
                            .on_press(PomodoroMessage::ResetPomodoro)
                    }),
            )
            .align_items(iced::Alignment::Center);

        widget::container(col)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_y()
            .into()
    }

    fn format_slider_value(&self) -> String {
        let hours = self.slider_value as u32 / 3600;
        let minutes = (self.slider_value as u32 % 3600) / 60;
        let remaining_seconds = self.slider_value as u32 % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, remaining_seconds)
    }

    pub fn update(&mut self, message: PomodoroMessage) -> Command<crate::app::Message> {
        let mut commands = Vec::new();
        match message {
            PomodoroMessage::PomodoroTick => {
                self.slider_value -= 1.;

                if self.slider_value <= 0. {
                    if self.in_action {
                        self.pomodoro_completed += 1;
                        if self.pomodoro_completed == self.pomodoro_before_long_pause {
                            log::info!("start long pause");
                            let seconds = self.long_pause_duration as u32 * 60;
                            self.slider_value = seconds as f32;
                            self.slider_max_value = seconds as f32;
                            self.pomodoro_completed = 0;
                        } else {
                            log::info!("start short pause");
                            let seconds = self.pause_duration as u32 * 60;
                            self.slider_value = seconds as f32;
                            self.slider_max_value = seconds as f32;
                        }
                        self.in_action = false;
                    } else {
                        log::info!("start new pomodoro");
                        let seconds = self.timer_duration as u32 * 60;
                        self.slider_value = seconds as f32;
                        self.slider_max_value = seconds as f32;
                        self.in_action = true;
                    }
                }
            }
            PomodoroMessage::StartPomodoro => {
                commands.push(Command::perform(async {}, |_| Message::StartPomodoroTimer));
                self.in_action = true;
                let res = Notification::new()
                    .summary(&fl!("pomodoro-started"))
                    .body(&fl!("pomodoro-started-des"))
                    .appname("Chronos")
                    .show();
                log::info!("notification result is ok: {:?}", res.is_ok());
            }
            PomodoroMessage::PausePomodoro => {
                commands.push(Command::perform(async {}, |_| Message::PausePomodoroTimer));
                self.in_action = false;
            }
            PomodoroMessage::ResetPomodoro => {
                self.slider_value = self.slider_max_value;
            }
        }
        Command::batch(commands)
    }
}
