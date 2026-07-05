# Trance Screensaver Suite — Wayland-Native Screensavers <img src="assets/icon.svg" width="48" height="48" alt="trance logo" align="right">

Trance is a modular Wayland-native screensaver system for modern Linux desktops, with first-class integration for Pop!_OS and the COSMIC Desktop environment.

---

## 🏛️ Architecture & Stack
*   **Frontend**: Yew / COSMIC Panel Applet (`trance-applet`)
*   **Backend**: Rust (`trance-daemon`, `trance-cli`)
*   **Deployment**: Debian (APT), Fedora (DNF), Systemd User Service

---

## 🟢 Key Features
*   **Modular Architecture**: Split into a core daemon (`trance-daemon`), optional screensaver plugins, and a panel applet (`trance-applet`).
*   **Resolution Upscaling**: Renders simulation grids at reduced scale and upscales them on the CPU to reduce system power and dependencies.
*   **D-Bus Session API**: Full session interface (`io.github.ubermetroid.trance`) for controlling timeouts, state, inhibits, and screensaver choices.
*   **Wayland Native**: Native integration with `ext-idle-notify-v1` and `zwlr_layer_shell_v1`.

---

## 💾 Deployment & Installation

### Debian / Ubuntu / Pop!_OS (APT)
```bash
# 1. Download the repository GPG keyring and source list
sudo curl -fsSL https://ubermetroid.github.io/packages/apt/ubermetroid-keyring.gpg -o /etc/apt/trusted.gpg.d/ubermetroid.gpg
sudo curl -fsSL https://ubermetroid.github.io/packages/apt/ubermetroid.list -o /etc/apt/sources.list.d/ubermetroid.list

# 2. Update and install
sudo apt update && sudo apt install trance
```
*Note: Installs recommended plugins (`trance-plugins-all`) and the COSMIC applet (`trance-applet`). For the core package only: `sudo apt install --no-install-recommends trance`.*

### Fedora (DNF)
```bash
# 1. Download the repository configuration
sudo curl -fsSL https://ubermetroid.github.io/packages/rpm/ubermetroid.repo -o /etc/yum.repos.d/ubermetroid.repo

# 2. Update and install
sudo dnf check-update && sudo dnf install trance
```

---

## ⚙️ Configuration Options & API

### CLI Controller Reference
Trance provides a CLI tool `trance` (built from `trance-cli`) to manage the daemon:

| Command | Usage | Description |
| :--- | :--- | :--- |
| `status` | `trance status [--json]` | Show live daemon state (or JSON) |
| `enable` / `disable` | `trance enable`, `trance disable` | Toggle idle screensaver activation |
| `preview` | `trance preview <saver>` | Preview a screensaver immediately |
| `stop` | `trance stop` | Stop any running preview or active screensaver |
| `list` | `trance list` | List all installed screensavers |

Other advanced commands include: `config` (get/set settings over D-Bus), `interactive` (TUI menu wizard), `doctor` (diagnostics suite), `bug-report` (scrubbed bug info packaging), `self-update` (policy checking), and `clean` (stale cache pruning).

### D-Bus API Reference
The daemon exports `io.github.ubermetroid.trance` on the session bus at `/io/github/ubermetroid/trance`:

| Method | Description |
| :--- | :--- |
| `GetStatus` | Returns live daemon state (`idle_enabled`, `session_locked`, etc.) |
| `Enable` / `Disable` | Toggle idle screensaver activation |
| `SetTimeout(minutes)` | Set idle timeout (1–240 minutes) |
| `SetSaver(name)` | Set active saver (`""` = random) |
| `ListSavers` | List installed screensaver plugins |
| `Preview(name)` | Start a saver immediately |
| `StopPreview` | Stop a running preview or idle presentation |
| `Inhibit(app, reason)` | Prevent idle activation; returns a cookie |
| `UnInhibit(cookie)` | Remove an inhibit request |

### Environment Variables

| Variable | Default | Description |
| :--- | :--- | :--- |
| `TRANCE_RENDER_SCALE` | `0.75` | Simulation grid scale (`0.25`–`1.0`). Lower = chunkier effect |
| `TRANCE_GPU_FILTER` | `linear` | `linear` or `nearest` CPU upscale filter |
| `TRANCE_MAX_FPS` | `0` (auto) | Cap frame rate. `0` uses monitor refresh rate |

---

## 🛠️ Local Development

Ensure you have the Rust toolchain installed.

```bash
# 1. Build core daemon
cargo build --release -p trance-daemon

# 2. Stop active service and run daemon manually
systemctl --user stop trance-daemon
~/.local/bin/trance-daemon daemon
```

---

## 📄 License
Licensed under the [Apache License, Version 2.0](LICENSE). Copyright 2026 UberMetroid.