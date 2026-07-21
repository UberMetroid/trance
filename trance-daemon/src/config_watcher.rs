// SPDX-License-Identifier: MIT

use std::sync::Arc;
use std::time::Duration;
use notify::{Event, EventKind, RecursiveMode, Watcher};

use crate::config::DaemonConfig;
use crate::controller::DaemonController;

pub fn start_config_watcher(controller: Arc<DaemonController>) {
    let Some(path) = DaemonConfig::get_config_path() else {
        return;
    };
    let Some(parent_dir) = path.parent() else {
        return;
    };

    if !parent_dir.exists() {
        let _ = std::fs::create_dir_all(parent_dir);
    }

    let controller_clone = controller.clone();
    let target_path = path.clone();

    let mut watcher = match notify::recommended_watcher(move |res: Result<Event, _>| {
        if let Ok(event) = res {
            if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_))
                && event.paths.iter().any(|p| p == &target_path)
            {
                tracing::info!("Config file modified on disk; hot-reloading settings...");
                let fresh = DaemonConfig::load();
                let _ = controller_clone.mutate_config(|cfg| {
                    *cfg = fresh;
                });
            }
        }
    }) {
        Ok(w) => w,
        Err(e) => {
            tracing::warn!("Failed to initialize config file watcher: {e}");
            return;
        }
    };

    if let Err(e) = watcher.watch(parent_dir, RecursiveMode::NonRecursive) {
        tracing::warn!("Failed to watch config directory {:?}: {e}", parent_dir);
        return;
    }

    tokio::spawn(async move {
        let _watcher = watcher;
        loop {
            tokio::time::sleep(Duration::from_secs(3600)).await;
        }
    });
}
