// SPDX-License-Identifier: {{LICENSE}}

use std::any::TypeId;

use cosmic::Application;
use cosmic::{
    cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry},
    iced::Subscription,
};
use serde::{Deserialize, Serialize};

pub const CONFIG_VERSION: u64 = 1;

#[derive(Clone, CosmicConfigEntry, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub timer_duration: u32,
    pub pause_duration: u32,
    pub long_pause_duration: u32,
    pub pomodoro_before_long_pause: u32,
    pub notifications_active: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timer_duration: 25,
            pause_duration: 5,
            long_pause_duration: 15,
            pomodoro_before_long_pause: 4,
            notifications_active: false,
        }
    }
}

impl Config {
    pub fn load() -> (Option<cosmic_config::Config>, Self) {
        match cosmic_config::Config::new(crate::app::AppModel::APP_ID, CONFIG_VERSION) {
            Ok(config_handler) => {
                let config = match Config::get_entry(&config_handler) {
                    Ok(ok) => ok,
                    Err((errs, config)) => {
                        log::info!("errors loading config: {:?}", errs);
                        config
                    }
                };
                (Some(config_handler), config)
            }
            Err(err) => {
                log::error!("failed to create config handler: {}", err);
                (None, Config::default())
            }
        }
    }

    pub fn subscription() -> Subscription<cosmic_config::Update<Self>> {
        struct ConfigSubscription;
        cosmic_config::config_subscription(
            TypeId::of::<ConfigSubscription>(),
            crate::app::AppModel::APP_ID.into(),
            CONFIG_VERSION,
        )
    }
}
