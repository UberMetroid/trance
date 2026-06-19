// SPDX-License-Identifier: MIT

use crate::config::Local76Config;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::platform_specific::shell::wayland::commands::popup::{destroy_popup, get_popup};
use cosmic::iced::{futures, window::Id, Limits, Subscription};
use cosmic::prelude::*;
use cosmic::widget;
use futures::SinkExt;

pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    config: crate::config::Config,
    local_config: Local76Config,
    screensavers: Vec<String>,
    daemon_running: bool,
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            core: cosmic::Core::default(),
            popup: None,
            config: crate::config::Config::default(),
            local_config: Local76Config::default(),
            screensavers: Vec::new(),
            daemon_running: false,
        }
    }
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    SubscriptionChannel,
    UpdateConfig(crate::config::Config),
    ToggleIdleEnabled(bool),
    ActiveSaverSelected(String),
    LockNow,
    OpenConfigurator,
}

impl AppModel {
    fn check_daemon(&mut self) {
        let pid_path = if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
            std::path::PathBuf::from(runtime_dir).join("trance-daemon.pid")
        } else {
            std::env::temp_dir().join("trance-daemon.pid")
        };
        if pid_path.exists() {
            if let Ok(pid_str) = std::fs::read_to_string(&pid_path) {
                if let Ok(pid) = pid_str.trim().parse::<i32>() {
                    unsafe {
                        if libc::kill(pid, 0) == 0 {
                            self.daemon_running = true;
                            return;
                        }
                    }
                }
            }
        }
        self.daemon_running = false;
    }

    fn get_saver_to_run(&self) -> Option<String> {
        if let Some(ref active) = self.local_config.active_saver {
            Some(active.clone())
        } else {
            // Pick a random screensaver from the list
            if self.screensavers.is_empty() {
                None
            } else {
                use std::time::{SystemTime, UNIX_EPOCH};
                let micros = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_micros() as usize;
                let idx = micros % self.screensavers.len();
                Some(self.screensavers[idx].clone())
            }
        }
    }
}

/// Create a COSMIC application from the app model
impl cosmic::Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "com.system76.CosmicApplet.Trance";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let mut app = AppModel {
            core,
            config: cosmic_config::Config::new(Self::APP_ID, crate::config::Config::VERSION)
                .map(|context| match crate::config::Config::get_entry(&context) {
                    Ok(config) => config,
                    Err((_errors, config)) => {
                        config
                    }
                })
                .unwrap_or_default(),
            local_config: Local76Config::load(),
            screensavers: trance_runner::discovery::detect_screensavers(),
            daemon_running: false,
            popup: None,
        };
        app.check_daemon();

        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    /// Describes the interface based on the current state of the application model.
    ///
    /// The applet's button in the panel will be drawn using the main view method.
    /// This view should emit messages to toggle the applet's popup window, which will
    /// be drawn using the `view_window` method.
    fn view(&self) -> Element<'_, Self::Message> {
        self.core
            .applet
            .icon_button("display-symbolic")
            .on_press(Message::TogglePopup)
            .into()
    }

    /// The applet's popup window will be drawn using this view method. If there are
    /// multiple poups, you may match the id parameter to determine which popup to
    /// create a view for.
    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        let options = {
            let mut opts = vec!["Random".to_string()];
            for s in &self.screensavers {
                opts.push(s.clone());
            }
            opts
        };
        let selected = Some(self.local_config.active_saver.clone().unwrap_or_else(|| "Random".to_string()));
        let pick_list = cosmic::iced::widget::pick_list(
            options,
            selected,
            Message::ActiveSaverSelected,
        );

        let daemon_status = if self.daemon_running {
            "Daemon: Active"
        } else {
            "Daemon: Inactive"
        };

        let header = cosmic::iced::widget::Column::new()
            .spacing(4)
            .push(widget::text("Trance Screensaver").size(16))
            .push(widget::text(daemon_status).size(12));

        let actions = cosmic::iced::widget::Row::new()
            .spacing(8)
            .push(widget::button::suggested("Lock Now").on_press(Message::LockNow))
            .push(widget::button::standard("Open Configurator").on_press(Message::OpenConfigurator));

        let content_list = widget::list_column()
            .add(header)
            .add(widget::settings::item(
                "Idle Activation",
                widget::toggler(self.local_config.idle_enabled).on_toggle(Message::ToggleIdleEnabled),
            ))
            .add(widget::settings::item(
                "Active Screensaver",
                pick_list,
            ))
            .add(actions);

        self.core.applet.popup_container(content_list).into()
    }

    /// Register subscriptions for this application.
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            // Create a subscription which emits updates through a channel.
            Subscription::run(|| {
                cosmic::iced::stream::channel(4, move |mut channel: futures::channel::mpsc::Sender<_>| async move {
                    _ = channel.send(Message::SubscriptionChannel).await;
                    futures::future::pending().await
                })
            }),
            // Watch for application configuration changes.
            self.core()
                .watch_config::<crate::config::Config>(Self::APP_ID)
                .map(|update| {
                    Message::UpdateConfig(update.config)
                }),
        ])
    }

    /// Handles messages emitted by the application and its widgets.
    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::SubscriptionChannel => {}
            Message::UpdateConfig(config) => {
                self.config = config;
            }
            Message::ToggleIdleEnabled(toggled) => {
                self.local_config.idle_enabled = toggled;
                let _ = self.local_config.save();
            }
            Message::ActiveSaverSelected(saver) => {
                if saver == "Random" {
                    self.local_config.active_saver = None;
                } else {
                    self.local_config.active_saver = Some(saver);
                }
                let _ = self.local_config.save();
            }
            Message::LockNow => {
                let target_saver = self.get_saver_to_run();
                if let Some(saver) = target_saver {
                    let _ = std::process::Command::new("trance")
                        .args(["start", &saver])
                        .spawn();
                }
            }
            Message::OpenConfigurator => {
                let _ = std::process::Command::new("xterm")
                    .args(["-bg", "black", "-fg", "white", "-T", "Trance", "-geometry", "135x40", "-e", "trance"])
                    .spawn();
            }
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    self.local_config = Local76Config::load();
                    self.screensavers = trance_runner::discovery::detect_screensavers();
                    self.check_daemon();

                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        .min_width(300.0)
                        .min_height(200.0)
                        .max_height(1080.0);
                    get_popup(popup_settings)
                }
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}
