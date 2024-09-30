// SPDX-License-Identifier: {{LICENSE}}

use cosmic::iced::Size;

mod app;
mod config;
mod core;
mod i18n;
mod pages;

fn main() -> cosmic::iced::Result {
    env_logger::init();
    // Get the system's preferred languages.
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    i18n::init(&requested_languages);

    // Settings for configuring the application window and iced runtime.
    let settings = cosmic::app::Settings::default().size(Size::new(500., 800.));

    // Starts the application's event loop with `()` as the application's flags.
    cosmic::app::run::<app::AppModel>(settings, ())
}
