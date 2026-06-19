//! Linux-specific /proc readers used by `sys_info`.
//! Extracted from `sys_info.rs` to keep that file under the 250-line cap.

use crate::toolkit::platform::PowerStatus;

// ---------------------------------------------------------------------------
// /proc readers
// ---------------------------------------------------------------------------

pub fn read_proc_stat_field(field: &str) -> Option<u64> {
    let content = std::fs::read_to_string("/proc/stat").ok()?;
    for line in content.lines() {
        if line.starts_with(field) {
            return line.split_whitespace().nth(1)?.parse::<u64>().ok();
        }
    }
    None
}

pub fn read_proc_meminfo_kb(field: &str) -> Option<u64> {
    let content = std::fs::read_to_string("/proc/meminfo").ok()?;
    for line in content.lines() {
        if line.starts_with(field) {
            return line.split_whitespace().nth(1)?.parse::<u64>().ok();
        }
    }
    None
}

pub fn read_proc_cpuinfo_brand() -> Option<String> {
    let content = std::fs::read_to_string("/proc/cpuinfo").ok()?;
    for line in content.lines() {
        if let Some(brand) = line.strip_prefix("model name") {
            if let Some(brand) = brand.split(':').nth(1) {
                let trimmed = brand.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Linux impls for the cross-platform queries in sys_info.rs
// ---------------------------------------------------------------------------

pub fn query_os_version_linux() -> (String, String) {
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                let val = line.split('=').nth(1).unwrap_or("").trim_matches('"');
                if !val.is_empty() {
                    return (val.to_string(), val.to_string());
                }
            }
        }
    }
    ("Linux".to_string(), "Linux".to_string())
}

pub fn query_kernel_linux() -> String {
    std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

pub fn query_cpu_brand_linux() -> Option<String> {
    read_proc_cpuinfo_brand()
}

pub fn query_memory_linux() -> (u64, u64, f32) {
    let total = read_proc_meminfo_kb("MemTotal:").unwrap_or(0);
    let free = read_proc_meminfo_kb("MemFree:").unwrap_or(0);
    let available = read_proc_meminfo_kb("MemAvailable:").unwrap_or(free);
    let used = total.saturating_sub(available);
    if total > 0 {
        return (
            used / 1024,
            total / 1024,
            (used as f32 / total as f32) * 100.0,
        );
    }
    (0, 0, 0.0)
}

pub fn query_cpu_usage_pct_linux() -> f32 {
    // Use /proc/loadavg as a coarse proxy scaled to a percent.
    if let Ok(content) = std::fs::read_to_string("/proc/loadavg") {
        if let Some(first) = content.split_whitespace().next() {
            if let Ok(load) = first.parse::<f32>() {
                let nproc = std::thread::available_parallelism()
                    .map(|n| n.get() as f32)
                    .unwrap_or(4.0);
                return ((load / nproc) * 100.0).clamp(0.0, 100.0);
            }
        }
    }
    0.0
}

pub fn query_uptime_secs_linux() -> u64 {
    std::fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| {
            s.split_whitespace()
                .next()
                .and_then(|v| v.parse::<f32>().ok())
        })
        .map(|v| v as u64)
        .unwrap_or(0)
}

pub fn query_dark_mode_linux() -> bool {
    // 1. Try reading standard GTK-3.0 / GTK-4.0 settings files directly (subprocess-free)
    for path in &[
        ".config/gtk-4.0/settings.ini",
        ".config/gtk-3.0/settings.ini",
    ] {
        if let Some(home) = std::env::var_os("HOME") {
            let full_path = std::path::Path::new(&home).join(path);
            if let Ok(content) = std::fs::read_to_string(full_path) {
                let lower = content.to_lowercase();
                for line in lower.lines() {
                    if line.contains("gtk-application-prefer-dark-theme") && line.contains("true") {
                        return true;
                    }
                    if line.contains("gtk-theme-name") && line.contains("dark") {
                        return true;
                    }
                }
            }
        }
    }

    // 2. Fallback to gsettings if ini files are missing or inconclusive
    if let Ok(output) = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "color-scheme"])
        .output()
    {
        let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if s.contains("prefer-dark") {
            return true;
        }
    }
    if let Ok(output) = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output()
    {
        let s = String::from_utf8_lossy(&output.stdout).to_lowercase();
        if s.contains("dark") {
            return true;
        }
    }
    true
}

pub fn query_power_status_linux() -> Option<PowerStatus> {
    let mut ac_online = true;
    let mut has_ac = false;
    let mut battery_percent: Option<u8> = None;
    if let Ok(entries) = std::fs::read_dir("/sys/class/power_supply") {
        for entry in entries.take(64).flatten() {
            let path = entry.path();
            if let Ok(ty_str) = std::fs::read_to_string(path.join("type")) {
                match ty_str.trim() {
                    "Mains" => {
                        if let Ok(online_str) = std::fs::read_to_string(path.join("online")) {
                            let online = online_str.trim() == "1";
                            if !has_ac {
                                ac_online = online;
                                has_ac = true;
                            } else {
                                ac_online = ac_online || online;
                            }
                        }
                    }
                    "Battery" => {
                        if let Ok(cap_str) = std::fs::read_to_string(path.join("capacity")) {
                            if let Ok(pct) = cap_str.trim().parse::<u8>() {
                                battery_percent = Some(pct);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    battery_percent.map(|pct| PowerStatus {
        ac_online: if has_ac { ac_online } else { true },
        battery_percent: pct,
    })
}
