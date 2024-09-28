use cosmic::{widget::icon, Element};

use crate::{app, fl};

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub enum NavPage {
    #[default]
    Pomodoro,
    Settings,
}

impl Default for &NavPage {
    fn default() -> Self {
        &NavPage::Pomodoro
    }
}

impl NavPage {
    pub fn title(&self) -> String {
        match self {
            Self::Pomodoro => fl!("pomodoro"),
            Self::Settings => fl!("settings"),
        }
    }

    pub fn icon(&self) -> cosmic::widget::Icon {
        match self {
            Self::Pomodoro => icon::from_name("process-working-spinner-hour-symbolic").into(),
            Self::Settings => icon::from_name("application-default-symbolic").into(),
        }
    }

    pub fn view<'a>(&self, app: &'a app::AppModel) -> Element<'a, app::Message> {
        match self {
            NavPage::Pomodoro => app.pomodoro.view().map(app::Message::Pomodoro),
            NavPage::Settings => app.settings.view().map(app::Message::Settings),
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::Pomodoro, Self::Settings]
    }
}
