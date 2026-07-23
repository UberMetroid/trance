<h1 align="center">
  <img src="assets/icon.png" width="48" height="48" valign="middle" alt=""> Trance
</h1>

<p align="center">
  <b>Modular Wayland-native screensaver and ambient display daemon for Linux, written in Rust.</b>
</p>

<p align="center">
  Part of <a href="https://github.com/crateria">Crateria</a> · Brand: <a href="https://github.com/crateria/brand">crateria/brand</a>
  · Docs: <a href="https://crateria.github.io/">crateria.github.io</a>
</p>

<p align="center">
  <a href="https://github.com/crateria/trance/actions/workflows/ci.yml"><img src="https://github.com/crateria/trance/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://github.com/crateria/trance/security/advisories"><img src="https://img.shields.io/badge/security-private%20reporting-blue" alt="Security"></a>
</p>

---

### Install (native packages)

**Debian / Ubuntu / Pop!_OS:**

```bash
sudo mkdir -p /etc/apt/keyrings
sudo curl -fsSL https://crateria.github.io/packages/apt/crateria-keyring.gpg \
  -o /etc/apt/keyrings/crateria.gpg
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/crateria.gpg] https://crateria.github.io/packages/apt stable main" \
  | sudo tee /etc/apt/sources.list.d/crateria.list
sudo apt update && sudo apt install trance
```

**Fedora:**

```bash
sudo curl -fsSL https://crateria.github.io/packages/rpm/crateria.repo \
  -o /etc/yum.repos.d/crateria.repo
sudo dnf install trance
```

Package index: [crateria.github.io/packages](https://crateria.github.io/packages/)  
Official plugins: [crateria/trance-plugins](https://github.com/crateria/trance-plugins)

---

### Build from source

```bash
git clone https://github.com/crateria/trance.git
cd trance
cargo build --release -p trance-daemon -p trance-cli -p trance-tui
```

System deps (Debian/Ubuntu): `libdbus-1-dev libwayland-dev libxkbcommon-dev libssl-dev libpam0g-dev pkg-config`

Checks: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets && cargo test` (see CI).

---

### Releases

1. Tag `vX.Y.Z` on `master` → **Release** workflow builds `.deb` / `.rpm` and publishes a GitHub Release.  
2. Release workflow can **repository_dispatch** `crateria/packages` to import assets into the APT/DNF index.  
3. Users install from [crateria.github.io/packages](https://crateria.github.io/packages/).

---

### Environment configuration

| Variable | Description | Default |
| :--- | :--- | :---: |
| `TRANCE_IDLE_TIMEOUT_MINS` | Idle minutes before screensaver | `10` |
| `TRANCE_ACTIVE_SAVER` | Active plugin name | `beams` |
| `TRANCE_SHOW_FPS` | FPS overlay | `false` |
| `LOG_LEVEL` | Tracing filter | `info` |

---

### Administration CLI

```bash
trance-cli status
trance-cli enable | disable
trance-cli preview <plugin>
```

---

### Architecture

- **Wayland** — `ext-idle-notify-v1`, `ext-session-lock-v1`
- **wgpu** cell rendering; PAM lock integration
- **Plugins** — loadable effects ([trance-plugins](https://github.com/crateria/trance-plugins))

---

### Security

Please use [private vulnerability reporting](https://github.com/crateria/trance/security/advisories/new) (do not file public issues for sensitive bugs). Org policy: [SECURITY.md](https://github.com/crateria/.github/blob/master/SECURITY.md).

---

### License

Apache-2.0. See [LICENSE](LICENSE).
