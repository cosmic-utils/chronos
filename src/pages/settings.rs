use cosmic::{
    iced::{self, Length},
    widget::{self, spin_button},
    Command, Element,
};
use fraction::Decimal;

use crate::fl;

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    TimerDurationChanged(f32),
    PauseDurationChanged(f32),
    LongPauseDurationChanged(f32),
    PomodoroBeforeLongPauseChanged(spin_button::Message),
}

pub struct Settings {
    timer_duration_value: f32,
    pause_duration: f32,
    long_pause_duration: f32,
    pomodoro_before_long_pause_str: String,
    pomodoro_before_long_pause: u32,
}

impl Default for Settings {
    // Initialize default
    fn default() -> Self {
        Self {
            timer_duration_value: 0.,
            pause_duration: 0.,
            long_pause_duration: 0.,
            pomodoro_before_long_pause: 4,
            pomodoro_before_long_pause_str: "4".to_string(),
        }
    }
}

impl Settings {
    pub fn view<'a>(&'a self) -> Element<'a, SettingsMessage> {
        let mut col = widget::column();

        col = col.push(self.timer_view());

        widget::container(col)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .center_y()
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
                                        .align_items(iced::Alignment::End),
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
                        .push(widget::vertical_space(Length::from(10)))
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
                                        .align_items(iced::Alignment::End),
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
                        .push(widget::vertical_space(Length::from(10)))
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
                                        .align_items(iced::Alignment::End),
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
                                .push(widget::text::text(fl!("pomodoro-before-long-pause")))
                                .push(
                                    //TODO il bottone che aumenta e diminuisce un numero
                                    widget::spin_button(
                                        &self.pomodoro_before_long_pause_str,
                                        SettingsMessage::PomodoroBeforeLongPauseChanged,
                                    ),
                                ),
                        ),
                ),
            )
            .push(widget::vertical_space(Length::from(20)));

        element.into()
    }

    pub fn update(&mut self, message: SettingsMessage) -> Command<crate::app::Message> {
        let mut commands = Vec::new();
        match message {
            SettingsMessage::TimerDurationChanged(value) => {
                self.timer_duration_value = value;
                //TODO store in settings
            }
            SettingsMessage::PauseDurationChanged(value) => {
                self.pause_duration = value;
                //TODO store in settings
            }
            SettingsMessage::LongPauseDurationChanged(value) => {
                self.long_pause_duration = value;
                //TODO store in settings
            }
            SettingsMessage::PomodoroBeforeLongPauseChanged(message) => {
                //TODO aggiungere il controllo che non deve scendere sotto lo zero (o sotto l'uno?)
                // poi si deve rendere editabile senza premener epiu e meno
                // impostare il limite anche in altezza? non oltre 10 per esempio?
                match message {
                    spin_button::Message::Increment => {
                        if self.pomodoro_before_long_pause < 15 {
                            self.pomodoro_before_long_pause += 1;
                        }
                    }
                    spin_button::Message::Decrement => {
                        if self.pomodoro_before_long_pause > 0 {
                            self.pomodoro_before_long_pause -= 1;
                        }
                    }
                }
                self.pomodoro_before_long_pause_str = self.pomodoro_before_long_pause.to_string();
            }
        }
        Command::batch(commands)
    }
}
