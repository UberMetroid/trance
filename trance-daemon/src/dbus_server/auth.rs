// SPDX-License-Identifier: MIT

//! Authorization for D-Bus control methods.

use zbus::message::Header;
use zbus::Connection;

const TRUSTED_CONTROL_PEERS: &[&str] = &["trance", "trance-applet"];

fn peer_exe_basename(pid: u32) -> Option<String> {
    let path = format!("/proc/{pid}/exe");
    let target = std::fs::read_link(path).ok()?;
    target
        .file_name()
        .and_then(|name| name.to_str())
        .map(str::to_string)
}

fn is_trusted_control_peer(pid: u32) -> bool {
    if std::env::var("TRANCE_DBUS_TRUST_ALL").ok().as_deref() == Some("1") {
        return true;
    }
    peer_exe_basename(pid).is_some_and(|name| TRUSTED_CONTROL_PEERS.contains(&name.as_str()))
}

/// Control methods (preview, config writes) require trance CLI or applet.
pub async fn require_control_peer(
    connection: &Connection,
    header: &Header<'_>,
) -> zbus::fdo::Result<()> {
    let sender = header.sender().ok_or_else(|| {
        zbus::fdo::Error::AccessDenied("control request missing D-Bus sender".into())
    })?;

    let dbus = zbus::fdo::DBusProxy::new(connection)
        .await
        .map_err(|error| zbus::fdo::Error::Failed(error.to_string()))?;
    let creds = dbus
        .get_connection_credentials((*sender).clone().into())
        .await
        .map_err(|_| zbus::fdo::Error::AccessDenied("cannot verify D-Bus peer".into()))?;
    let pid = creds.process_id().ok_or_else(|| {
        zbus::fdo::Error::AccessDenied("D-Bus peer PID unavailable".into())
    })?;

    if is_trusted_control_peer(pid) {
        Ok(())
    } else {
        Err(zbus::fdo::Error::AccessDenied(
            "control methods require the trance CLI or panel applet".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trusted_peer_names_are_fixed() {
        assert!(TRUSTED_CONTROL_PEERS.contains(&"trance"));
        assert!(TRUSTED_CONTROL_PEERS.contains(&"trance-applet"));
    }

    #[test]
    fn current_process_is_readable() {
        let pid = std::process::id();
        assert!(peer_exe_basename(pid).is_some());
    }
}