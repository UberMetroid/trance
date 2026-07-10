#!/bin/sh
# Shared helpers for deb/rpm maintainer scripts.
# All operations are best-effort: never abort package install/upgrade.
# shellcheck disable=SC2039,SC3043

# Iterate logged-in users that have a usable user bus.
# Calls: for_each_user_session <callback>
# callback receives: uid user
for_each_user_session() {
    _cb="$1"
    command -v loginctl >/dev/null 2>&1 || return 0
    command -v systemctl >/dev/null 2>&1 || return 0

    # Columns vary by systemd version; take first two tokens (uid, user).
    loginctl list-users --no-legend 2>/dev/null | while read -r uid user _rest; do
        case "$uid" in
            ''|*[!0-9]*) continue ;;
        esac
        [ -n "$user" ] || continue
        # Skip system-ish accounts without a session runtime.
        [ -d "/run/user/$uid" ] || continue
        [ -S "/run/user/$uid/bus" ] || continue
        "$_cb" "$uid" "$user" || true
    done
}

_user_systemctl() {
    _uid="$1"
    _user="$2"
    shift 2
    if command -v runuser >/dev/null 2>&1; then
        runuser -u "$_user" -- env \
            XDG_RUNTIME_DIR="/run/user/$_uid" \
            DBUS_SESSION_BUS_ADDRESS="unix:path=/run/user/$_uid/bus" \
            systemctl --user "$@" 2>/dev/null && return 0
    fi
    systemctl --user --machine="${_user}@" "$@" 2>/dev/null || true
}

try_reload_user_units() {
    _uid="$1"
    _user="$2"
    echo "-> daemon-reload for ${_user} (uid ${_uid})"
    _user_systemctl "$_uid" "$_user" daemon-reload || true
}

try_stop_trance() {
    _uid="$1"
    _user="$2"
    echo "-> try-stop trance-daemon for ${_user}"
    _user_systemctl "$_uid" "$_user" stop trance-daemon.service || true
}

# If enabled for this user, always restart (upgrades must leave the daemon
# running on the new binary). try-restart alone is a no-op when inactive.
try_restart_trance() {
    _uid="$1"
    _user="$2"
    echo "-> restart trance-daemon for ${_user} (if enabled/active)"
    _user_systemctl "$_uid" "$_user" reset-failed trance-daemon.service || true
    if command -v runuser >/dev/null 2>&1; then
        if runuser -u "$_user" -- env \
            XDG_RUNTIME_DIR="/run/user/$_uid" \
            DBUS_SESSION_BUS_ADDRESS="unix:path=/run/user/$_uid/bus" \
            systemctl --user is-enabled trance-daemon.service >/dev/null 2>&1; then
            _user_systemctl "$_uid" "$_user" restart trance-daemon.service || true
            return 0
        fi
    elif systemctl --user --machine="${_user}@" is-enabled trance-daemon.service >/dev/null 2>&1; then
        _user_systemctl "$_uid" "$_user" restart trance-daemon.service || true
        return 0
    fi
    _user_systemctl "$_uid" "$_user" try-restart trance-daemon.service || true
}

print_user_hint() {
    echo ""
    echo "  Note: trance-daemon is a *user* systemd service."
    echo "  If the screensaver is not running after install/upgrade, as your"
    echo "  desktop user run:"
    echo "    systemctl --user daemon-reload"
    echo "    systemctl --user enable --now trance-daemon"
    echo "  or:  trance doctor --fix"
    echo ""
}
