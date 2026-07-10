#!/bin/sh
# RPM %preun — $1 is count remaining after this transaction
# (0 = full uninstall, 1+ = upgrade).
#
# Important: do NOT stop the user service on upgrade.
# DNF/RPM often runs %post of the *new* package first, then %preun of the
# *old* package. Stopping here on upgrade would kill the daemon *after*
# the new package already restarted it, leaving it dead for the rest of
# the session.
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

# Upgrade ($1 >= 1): leave the daemon alone; %post restarts it onto the new binary.
# Full uninstall ($1 == 0): stop + disable so nothing is left running.
if [ "${1:-0}" -ne 0 ]; then
    exit 0
fi

try_stop_trance() {
    echo "-> stop trance-daemon for $2 (uninstall)"
    _user_systemctl "$1" "$2" stop trance-daemon.service || true
}

try_disable() {
    echo "-> disable trance-daemon for $2 (uninstall)"
    _user_systemctl "$1" "$2" disable trance-daemon.service || true
}

for_each_user_session try_stop_trance
for_each_user_session try_disable

exit 0
