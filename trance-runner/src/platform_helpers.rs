//! Linux terminal/screen helpers used by the screensaver runner.
//! (Windows support has been removed; this is now Linux-only for the supported
//! distros: Debian family, Red Hat family, Gentoo, Arch.)

// ---------------------------------------------------------------------------
// Monitor refresh rate
// ---------------------------------------------------------------------------

pub fn get_monitor_refresh_rate() -> u32 {
    120 // Reasonable default for terminal-based screensavers on Linux
}

// ---------------------------------------------------------------------------
// Terminal size
// ---------------------------------------------------------------------------

pub fn get_terminal_size() -> (usize, usize) {
    unsafe {
        let mut ws: libc::winsize = std::mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) == 0 {
            (ws.ws_col as usize, ws.ws_row as usize)
        } else {
            (80, 24)
        }
    }
}

// ---------------------------------------------------------------------------
// Mouse activity
// ---------------------------------------------------------------------------

pub fn check_mouse_activity(_initial_pos: &mut Option<(i32, i32)>) -> bool {
    false // Mouse activity detection not needed for these fullscreen terminal savers
}

// ---------------------------------------------------------------------------
// Keypress detection
// ---------------------------------------------------------------------------

pub fn check_keypress() -> bool {
    unsafe {
        let mut fd_set: libc::fd_set = std::mem::zeroed();
        libc::FD_SET(libc::STDIN_FILENO, &mut fd_set);
        let mut timeout = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        libc::select(
            libc::STDIN_FILENO + 1,
            &mut fd_set,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut timeout,
        ) > 0
    }
}

// ---------------------------------------------------------------------------
// Misc
// ---------------------------------------------------------------------------

#[allow(dead_code)]
pub fn command_exists(cmd: &str) -> bool {
    // Avoid shell metachar injection. Try "which" first (common on Linux),
    // then fallback to attempting to invoke the command.
    if let Ok(status) = std::process::Command::new("which")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
    {
        if status.success() {
            return true;
        }
    }
    // Fallback: if the command can at least be started (even if it exits non-zero),
    // consider it present. (Used for "xterm" in fullscreen launch paths.)
    std::process::Command::new(cmd)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok()
}
