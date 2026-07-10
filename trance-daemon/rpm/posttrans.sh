#!/bin/sh
# RPM %posttrans — runs after the *entire* transaction (installs + uninstalls).
#
# Why this exists: DNF/RPM may run %post of the new package first, then %preun
# of the *old* package. Older trance preun scripts stopped the user service on
# upgrade, which left the daemon dead after the new package had already
# restarted it. posttrans restarts once more so upgrades stay healthy even when
# the old package still carries the buggy preun.
set -u

for_each_user_session() {
    _cb="$1"
    command -v loginctl >/dev/null 2>&1 || return 0
    command -v systemctl >/dev/null 2>&1 || return 0
    loginctl list-users --no-legend 2>/dev/null | while read -r uid user _rest; do
        case "$uid" in ''|*[!0-9]*) continue ;; esac
        [ -n "$user" ] || continue
        [ -d "/run/user/$uid" ] || continue
        [ -S "/run/user/$uid/bus" ] || continue
        "$_cb" "$uid" "$user" || true
    done
}

_user_systemctl() {
    _uid="$1"; _user="$2"; shift 2
    if command -v runuser >/dev/null 2>&1; then
        runuser -u "$_user" -- env \
            XDG_RUNTIME_DIR="/run/user/$_uid" \
            DBUS_SESSION_BUS_ADDRESS="unix:path=/run/user/$_uid/bus" \
            systemctl --user "$@" 2>/dev/null && return 0
    fi
    systemctl --user --machine="${_user}@" "$@" 2>/dev/null || true
}

try_restart_if_enabled() {
    echo "-> posttrans restart trance-daemon for $2 (if enabled)"
    _user_systemctl "$1" "$2" daemon-reload || true
    _user_systemctl "$1" "$2" reset-failed trance-daemon.service || true
    if command -v runuser >/dev/null 2>&1; then
        if runuser -u "$2" -- env \
            XDG_RUNTIME_DIR="/run/user/$1" \
            DBUS_SESSION_BUS_ADDRESS="unix:path=/run/user/$1/bus" \
            systemctl --user is-enabled trance-daemon.service >/dev/null 2>&1; then
            _user_systemctl "$1" "$2" restart trance-daemon.service || true
            return 0
        fi
    elif systemctl --user --machine="${2}@" is-enabled trance-daemon.service >/dev/null 2>&1; then
        _user_systemctl "$1" "$2" restart trance-daemon.service || true
        return 0
    fi
    # Not enabled: only bounce if already active.
    _user_systemctl "$1" "$2" try-restart trance-daemon.service || true
}

echo "trance RPM post-transaction (ensure daemon running if enabled)..."
for_each_user_session try_restart_if_enabled
exit 0
