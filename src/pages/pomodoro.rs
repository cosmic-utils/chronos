use cosmic::iced::Alignment;
use cosmic::{
    iced::{self, Length, Padding},
    widget::{self, icon},
    Element, Task,
};
use notify_rust::Notification;

use crate::{app::Message, config::Config, fl};

enum CompletedItem {
    Pomodoro(u32),
    Pause(u32),
}

#[derive(Debug, Clone)]
pub enum PomodoroMessage {
    UpdateConfig,
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
    history: Vec<CompletedItem>,
    notifications_active: bool,
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
            history: Vec::new(),
            notifications_active: config.notifications_active,
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
                .align_x(iced::alignment::Horizontal::Center),
            )
            .push(
                widget::text::title1(self.format_slider_value())
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .push(widget::Space::with_height(10))
            .push(
                widget::progress_bar(0.0..=self.slider_max_value, self.slider_value)
                    .width(Length::Fixed(250.0))
                    .height(Length::Fixed(4.0)),
            )
            .push(widget::Space::with_height(10))
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
            .push(self.history_view())
            .align_x(Alignment::Center);

        widget::container(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }

    pub fn history_view<'a>(&'a self) -> Element<'a, PomodoroMessage> {
        let mut inner_col = widget::column();
        if self.history.len() > 0 {
            for item in &self.history {
                match item {
                    CompletedItem::Pomodoro(seconds) => {
                        inner_col = inner_col.push(
                            widget::row()
                                .push(
                                    widget::column()
                                        .push(widget::text::text(fl!("pomodoro")))
                                        .width(Length::Fill),
                                )
                                .push(
                                    widget::column()
                                        .push(widget::text::text(
                                            self.format_seconds(seconds.clone()),
                                        ))
                                        .width(Length::Fill)
                                        .align_x(Alignment::End),
                                ),
                        );
                    }
                    CompletedItem::Pause(seconds) => {
                        inner_col = inner_col.push(
                            widget::row()
                                .push(
                                    widget::column()
                                        .push(widget::text::text(fl!("pause")))
                                        .width(Length::Fill),
                                )
                                .push(
                                    widget::column()
                                        .push(widget::text::text(
                                            self.format_seconds(seconds.clone()),
                                        ))
                                        .width(Length::Fill)
                                        .align_x(Alignment::End),
                                ),
                        );
                    }
                }
                inner_col = inner_col.push(widget::Space::with_height(5));
                inner_col = inner_col.push(widget::divider::horizontal::default());
                inner_col = inner_col.push(widget::Space::with_height(5));
            }
        } else {
            inner_col = inner_col.push(widget::text::text(fl!("no-elements")));
        }

        widget::column()
            .push(widget::Space::with_height(20))
            .push(
                widget::column()
                    .width(Length::Fixed(350.))
                    .push(
                        widget::container(widget::text::text(fl!("history"))).padding(Padding {
                            top: 0.,
                            right: 0.,
                            bottom: 0.,
                            left: 10.,
                        }),
                    )
                    .push(widget::Space::with_height(5))
                    .push(
                        widget::container(
                            widget::column().push(inner_col).padding(Padding::from(10)),
                        )
                        .class(cosmic::theme::Container::Card)
                        .width(Length::Fixed(350.)),
                    ),
            )
            .into()
    }

    pub fn update(&mut self, message: PomodoroMessage) -> Task<crate::app::Message> {
        let mut commands = Vec::new();
        match message {
            PomodoroMessage::UpdateConfig => {
                self.reset_all();
            }
            PomodoroMessage::PomodoroTick => {
                self.slider_value -= 1.;

                if self.slider_value <= 0. {
                    if self.pomodoro_completed < self.pomodoro_before_long_pause {
                        if self.slider_max_value == self.timer_duration as f32 * 60. {
                            // Pomodoro just finished, start short pause
                            log::info!("start short pause");
                            let seconds = self.pause_duration as u32 * 60;
                            self.slider_value = seconds as f32;
                            self.slider_max_value = seconds as f32;
                            self.pomodoro_completed += 1;
                            self.history
                                .push(CompletedItem::Pomodoro(self.timer_duration as u32 * 60));
                        } else {
                            // Short pause finished, start new pomodoro
                            log::info!("start new pomodoro");
                            let seconds = self.timer_duration as u32 * 60;
                            self.slider_value = seconds as f32;
                            self.slider_max_value = seconds as f32;
                            self.history
                                .push(CompletedItem::Pause(self.pause_duration as u32 * 60));
                        }
                    } else {
                        // After completing the specified number of Pomodoros, start long pause
                        if self.slider_max_value == self.timer_duration as f32 * 60. {
                            log::info!("start long pause");
                            let seconds = self.long_pause_duration as u32 * 60;
                            self.slider_value = seconds as f32;
                            self.slider_max_value = seconds as f32;
                            self.pomodoro_completed = 0;
                            self.history
                                .push(CompletedItem::Pomodoro(self.timer_duration as u32 * 60));
                        } else {
                            // Long pause finished, start new pomodoro
                            log::info!("start new pomodoro after long pause");
                            let seconds = self.timer_duration as u32 * 60;
                            self.slider_value = seconds as f32;
                            self.slider_max_value = seconds as f32;
                            self.history
                                .push(CompletedItem::Pause(self.long_pause_duration as u32 * 60));
                        }
                    }
                }
            }
            PomodoroMessage::StartPomodoro => {
                commands.push(Task::perform(async {}, |_| Message::StartPomodoroTimer));
                self.in_action = true;
                if self.notifications_active {
                    let _ = Notification::new()
                        .summary(&fl!("pomodoro-started"))
                        .body(&fl!("pomodoro-started-des"))
                        .appname("Chronos")
                        .show();
                }
            }
            PomodoroMessage::PausePomodoro => {
                commands.push(Task::perform(async {}, |_| Message::PausePomodoroTimer));
                if self.notifications_active {
                    let _ = Notification::new()
                        .summary(&fl!("pomodoro-paused"))
                        .body(&fl!("pomodoro-paused-des"))
                        .appname("Chronos")
                        .show();
                }
                self.in_action = false;
            }
            PomodoroMessage::ResetPomodoro => {
                if self.notifications_active {
                    let _ = Notification::new()
                        .summary(&fl!("pomodoro-stopped"))
                        .body(&fl!("pomodoro-stopped-des"))
                        .appname("Chronos")
                        .show();
                }
                self.reset_all();
            }
        }
        Task::batch(commands)
    }

    fn reset_all(&mut self) {
        let config = Config::load().1;
        let seconds = (config.timer_duration * 60) as f32;
        self.in_action = false;
        self.slider_value = seconds;
        self.slider_max_value = seconds;
        self.timer_duration = config.timer_duration as f32;
        self.pause_duration = config.pause_duration as f32;
        self.long_pause_duration = config.long_pause_duration as f32;
        self.pomodoro_completed = 0;
        self.pomodoro_before_long_pause = config.pomodoro_before_long_pause;
        self.history = Vec::new();
        self.notifications_active = config.notifications_active;
    }

    fn format_slider_value(&self) -> String {
        let hours = self.slider_value as u32 / 3600;
        let minutes = (self.slider_value as u32 % 3600) / 60;
        let remaining_seconds = self.slider_value as u32 % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, remaining_seconds)
    }

    fn format_seconds(&self, seconds: u32) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let remaining_seconds = seconds % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, remaining_seconds)
    }
}
