# IdleScreen names

## What users install

| Product | Package | Pulls |
|---------|---------|--------|
| COSMIC | **`idle-cosmic`** | `idle-daemon` + `idle-savers` + applet |
| TUI | **`idle-tui`** | `idle-daemon` |
| Windows | **`idle-windows`** | platform host (stub) |
| Studio | **`idle-studio`** | offline director (uses `render`) |

Do **not** advertise `dnf install idle` or `dnf install idle-daemon` as the product path.

## Engine packages (dependencies)

| Package | Role | Binary |
|---------|------|--------|
| `idle-daemon` | Background host | `idle-daemon` |
| `idle-cli` | Control CLI | **`idle`** |
| `idle-savers` | All effects meta | — |
| `idle-saver-*` | One effect | `.so` under `/usr/libexec/idle/screensavers/` |

## CLI

After a product install (or with `idle-cli` pulled as a recommend):

```bash
idle status
idle doctor
idle preview beams
```

## Source repo

GitHub: **[idlescreen/idle](https://github.com/idlescreen/idle)** (engine workspace).  
Not a user install unit.

## Frozen ABI

- D-Bus: `io.github.ubermetroid.trance`
- Plugin stem: `libscreensaver_<name>.so`
