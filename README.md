# IdleScreen

[![CI](https://github.com/idlescreen/idlescreen/actions/workflows/ci.yml/badge.svg)](https://github.com/idlescreen/idlescreen/actions/workflows/ci.yml)
[![Security](https://img.shields.io/badge/security-private%20reporting-blue)](https://github.com/idlescreen/idlescreen/security/advisories)

Modular Wayland-native screensaver and ambient display daemon for Linux, written in Rust.

| | |
|---|---|
| Brand | [idlescreen/brand](https://github.com/idlescreen/brand) |
| Packages | [idlescreen.github.io/packages](https://idlescreen.github.io/packages/) |
| Org | [idlescreen](https://github.com/idlescreen) |
| Plugins | [idlescreen plugins](https://github.com/orgs/idlescreen/repositories?q=plugin-) |
| Optional applet | [idlescreen/idlescreen-applet](https://github.com/idlescreen/idlescreen-applet) |

**Package and crate names remain `trance` / `trance-*` for API and install stability.** The product brand and GitHub org are IdleScreen; binary and Debian package name `trance` is intentional.

---

## Install (native packages)

### Debian / Ubuntu / Pop!_OS

```bash
sudo mkdir -p /etc/apt/keyrings
sudo curl -fsSL https://idlescreen.github.io/packages/apt/crateria-keyring.gpg \
  -o /etc/apt/keyrings/idlescreen.gpg
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/idlescreen.gpg] https://idlescreen.github.io/packages/apt stable main" \
  | sudo tee /etc/apt/sources.list.d/idlescreen.list
sudo apt update && sudo apt install trance
```

### Fedora

```bash
sudo curl -fsSL https://idlescreen.github.io/packages/rpm/crateria.repo \
  -o /etc/yum.repos.d/idlescreen.repo
sudo dnf install trance
```

Keyring and repo drop-in filenames on the package host may still use a `crateria-*` prefix for compatibility; the public host is **idlescreen.github.io**.

Optional: `trance-plugins-all`, `trance-cli`, `trance-tui`. COSMIC panel users can install [idlescreen-applet](https://github.com/idlescreen/idlescreen-applet) separately.

Package index: [idlescreen.github.io/packages](https://idlescreen.github.io/packages/)  
Official plugins: [org plugin repositories](https://github.com/orgs/idlescreen/repositories?q=plugin-)

---

## Build from source

```bash
git clone https://github.com/idlescreen/idlescreen.git
cd idlescreen
cargo build --release -p trance-daemon -p trance-cli -p trance-tui
```

System dependencies (Debian/Ubuntu): `libdbus-1-dev libwayland-dev libxkbcommon-dev libssl-dev libpam0g-dev pkg-config`

Checks (mirrors CI on `master`):

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p trance-api -p trance-dbus -p trance-ipc -p trance-daemon
```

An optional multi-stage Alpine `Dockerfile` builds release binaries for containerized tooling; desktop install prefers native packages, not containers.

---

## Releases

1. Tag `vX.Y.Z` on `master`.
2. The Release workflow builds `.deb` / `.rpm` assets and publishes a GitHub Release.
3. When `IDLESCREEN_PACKAGES_DISPATCH_TOKEN` is set, the workflow sends `repository_dispatch` `new_release` to [idlescreen/packages](https://github.com/idlescreen/packages) for signing and Pages index update.

---

## Environment configuration

| Variable | Description | Default |
| :--- | :--- | :---: |
| `TRANCE_IDLE_TIMEOUT_MINS` | Idle minutes before screensaver | `10` |
| `TRANCE_ACTIVE_SAVER` | Active plugin name | `beams` |
| `TRANCE_SHOW_FPS` | FPS overlay | `false` |
| `LOG_LEVEL` | Tracing filter | `info` |

---

## Administration CLI

```bash
trance-cli status
trance-cli enable | disable
trance-cli preview <plugin>
```

Binary name is `trance` for the CLI package on some installs; `trance-cli` is the cargo package name.

---

## Architecture

- Wayland idle and session lock protocols (`ext-idle-notify-v1`, `ext-session-lock-v1`)
- wgpu cell rendering; PAM-backed failsafe lock path
- Out-of-process plugins over UDS + POSIX shared memory with Landlock sandboxing
- Official effects live as separate `plugin-*` repositories under the IdleScreen org

D-Bus well-known names and object paths retain historical identifiers for ABI stability; they are not brand surface.

---

## Security

Please use [private vulnerability reporting](https://github.com/idlescreen/idlescreen/security/advisories/new) (do not file public issues for sensitive bugs).

---

## License

Apache-2.0. See [LICENSE](LICENSE).
