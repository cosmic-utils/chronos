use crate::{app::Message, config::Config, fl};
use cosmic::iced_core::alignment::Horizontal;
use cosmic::{
    iced::{self, Length},
    widget::{self},
    Element, Task,
};

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    TimerDurationChanged(f32),
    PauseDurationChanged(f32),
    LongPauseDurationChanged(f32),
    PomodoroBeforeLongPauseChanged(u32),
    NotificationToggle(bool),
}

pub struct Settings {
    timer_duration_value: f32,
    pause_duration: f32,
    long_pause_duration: f32,
    pomodoro_before_long_pause_str: String,
    pomodoro_before_long_pause: u32,
    notification_active: bool,
}

impl Default for Settings {
    // Initialize default
    fn default() -> Self {
        let config = Config::load().1;
        Self {
            timer_duration_value: config.timer_duration as f32,
            pause_duration: config.pause_duration as f32,
            long_pause_duration: config.long_pause_duration as f32,
            pomodoro_before_long_pause: config.pomodoro_before_long_pause,
            pomodoro_before_long_pause_str: config.pomodoro_before_long_pause.to_string(),
            notification_active: config.notifications_active,
        }
    }
}

impl Settings {
    pub fn view<'a>(&'a self) -> Element<'a, SettingsMessage> {
        let mut col = widget::column();

        col = col.push(self.timer_view());

        widget::container(col)
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_x(Horizontal::Center)
            .into()
    }

    pub fn timer_view<'a>(&'a self) -> Element<'a, SettingsMessage> {
        let mut element = widget::column();

        element = element
            .push(
                widget::settings::section().title(fl!("timer")).add(
                    widget::column()
                        .push(
                            widget::row()
                                .push(
                                    widget::column()
                                        .push(widget::text::text(fl!("timer-duration")))
                                        .width(Length::Fill),
                                )
                                .push(
                                    widget::column()
                                        .push(widget::text::text(format!(
                                            "{} {}",
                                            self.timer_duration_value,
                                            fl!("minutes")
                                        )))
                                        .width(Length::Fill)
                                        .align_x(iced::Alignment::End),
                                ),
                        )
                        .push(
                            widget::slider(
                                0.0..=120.0,
                                self.timer_duration_value,
                                SettingsMessage::TimerDurationChanged,
                            )
                            .width(Length::Fill)
                            .height(38),
                        )
                        .push(widget::divider::horizontal::default())
                        .push(widget::Space::with_height(10))
                        .push(
                            widget::row()
                                .push(
                                    widget::column()
                                        .push(widget::text::text(fl!("pause-duration")))
                                        .width(Length::Fill),
                                )
                                .push(
                                    widget::column()
                                        .push(widget::text::text(format!(
                                            "{} {}",
                                            self.pause_duration,
                                            fl!("minutes")
                                        )))
                                        .width(Length::Fill)
                                        .align_x(iced::Alignment::End),
                                ),
                        )
                        .push(
                            widget::slider(
                                0.0..=120.0,
                                self.pause_duration,
                                SettingsMessage::PauseDurationChanged,
                            )
                            .width(Length::Fill)
                            .height(38),
                        )
                        .push(widget::Space::with_height(10))
                        .push(
                            widget::row()
                                .push(
                                    widget::column()
                                        .push(widget::text::text(fl!("long-pause-duration")))
                                        .width(Length::Fill),
                                )
                                .push(
                                    widget::column()
                                        .push(widget::text::text(format!(
                                            "{} {}",
                                            self.long_pause_duration,
                                            fl!("minutes")
                                        )))
                                        .width(Length::Fill)
                                        .align_x(iced::Alignment::End),
                                ),
                        )
                        .push(
                            widget::slider(
                                0.0..=120.0,
                                self.long_pause_duration,
                                SettingsMessage::LongPauseDurationChanged,
                            )
                            .width(Length::Fill)
                            .height(38),
                        )
                        .push(
                            widget::row()
                                .push(
                                    widget::column()
                                        .push(widget::text::text(fl!("pomodoro-before-long-pause")))
                                        .width(Length::Fill),
                                )
                                .push(
                                    widget::column()
                                        .push(widget::spin_button(
                                            self.pomodoro_before_long_pause_str.clone(),
                                            self.pomodoro_before_long_pause,
                                            1,
                                            0,
                                            100,
                                            SettingsMessage::PomodoroBeforeLongPauseChanged,
                                        ))
                                        .width(Length::Fill)
                                        .align_x(iced::Alignment::End),
                                ),
                        ),
                ),
            )
            .push(widget::Space::with_height(20));

        element = element.push(
            widget::settings::section().title(fl!("notifications")).add(
                widget::column()
                    .width(Length::Fill)
                    .push(widget::text::text(fl!("activate-notification")))
                    .push(
                        widget::toggler(self.notification_active)
                            .on_toggle(SettingsMessage::NotificationToggle),
                    ),
            ),
        );

        element.into()
    }

    pub fn update(&mut self, message: SettingsMessage) -> Task<crate::app::Message> {
        let mut commands = Vec::new();
        match message {
            SettingsMessage::TimerDurationChanged(value) => {
                self.timer_duration_value = value;
                let mut config = Config::load();
                let _ = config
                    .1
                    .set_timer_duration(&config.0.unwrap(), self.timer_duration_value as u32);
            }
            SettingsMessage::PauseDurationChanged(value) => {
                self.pause_duration = value;
                let mut config = Config::load();
                let _ = config
                    .1
                    .set_pause_duration(&config.0.unwrap(), self.pause_duration as u32);
            }
            SettingsMessage::LongPauseDurationChanged(value) => {
                self.long_pause_duration = value;
                let mut config = Config::load();
                let _ = config
                    .1
                    .set_long_pause_duration(&config.0.unwrap(), self.long_pause_duration as u32);
            }
            SettingsMessage::PomodoroBeforeLongPauseChanged(message) => {
                self.pomodoro_before_long_pause = message.clone();
                self.pomodoro_before_long_pause_str = self.pomodoro_before_long_pause.to_string();
                let mut config = Config::load();
                let _ = config.1.set_pomodoro_before_long_pause(
                    &config.0.unwrap(),
                    self.pomodoro_before_long_pause as u32,
                );
            }
            SettingsMessage::NotificationToggle(value) => {
                self.notification_active = value;
                let mut config = Config::load();
                let _ = config.1.set_notifications_active(&config.0.unwrap(), value);
            }
        }
        commands.push(Task::perform(async {}, |_| {
            Message::Pomodoro(super::pomodoro::PomodoroMessage::UpdateConfig)
        }));
        Task::batch(commands)
    }
}
