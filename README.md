# AIMP Discord RPC

Discord Rich Presence for [AIMP](https://aimp.ru) music player (v6.00 beta for Linux).

Shows your currently playing track in your Discord profile — like Spotify or Kopuz.

## Known issue
D-Bus wont cath your track until you open some other audio that D-bsu WILL catch
to workaround this: open video/track in your browser and then use utility 

## Features

- "Listening to AIMP" activity type (not "Playing")
- Track title, artist, album in Discord status
- Elapsed time with live progress bar
- Play / Pause / Stop state indicator
- Radio stream detection (auto-hides progress)
- System tray icon with playback controls (play/pause, stop, next, prev)
- Album art as tray icon (when available via MPRIS)
- Auto-reconnect on AIMP restart
- Auto-start with system session
- Configurable polling interval and Discord assets

## Requirements

- Linux with D-Bus (any modern distro)
- [AIMP v6.00 beta for Linux](https://aimp.ru/forum/index.php?topic=70706.0)
- Discord desktop client running
- libdbus (`libdbus-1-dev` / `dbus-devel` / `libdbus`)
- GTK3 (`libgtk-3-dev` / `gtk3`) и libappindicator (`libappindicator-gtk3` / `libappindicator3-dev`)

## Building

### On Linux (native)

```bash
# Debian / Ubuntu / Mint
sudo apt install libdbus-1-dev pkg-config libgtk-3-dev libappindicator3-dev

# Arch / Manjaro
sudo pacman -S dbus gtk3 libappindicator-gtk3

# Fedora
sudo dnf install dbus-devel gtk3-devel libappindicator-gtk3

# Build
cargo build --release

# Install to PATH and application menu
./install.sh
```

The binary will be at `target/release/aimp-discord-rpc`

## Installation

1. Build from source or download from [Releases](https://github.com/vokiry/aimp-discord-rpc/releases)
2. Create a Discord application at https://discord.com/developers/applications
3. Upload assets (`aimp_logo`, `play`, `pause`) in the **Rich Presence** → **Art Assets** section
4. Copy `config.example.toml` to config path and set your `app_id`

### Install to system

After building, run the install script to add the binary to `$PATH`, create a desktop entry (app menu), and enable autostart:

```bash
# User install (default)
./install.sh

# System-wide (requires sudo)
sudo PREFIX=/usr/local ./install.sh
```

This will:
- Copy binary to `~/.local/bin/aimp-discord-rpc` (user) or `/usr/local/bin/aimp-discord-rpc` (system)
- Add desktop entry to application menu
- Enable autostart on login

After installation, the app will appear in your application menu as **AIMP Discord RPC** and start automatically with your session.

### Config paths

| Path | Description |
|------|-------------|
| `~/.config/aimp-discord-rpc/config.toml` | Default (XDG) |
| `./config.toml` | Current directory |
| `--config ./myconfig.toml` | Custom path (CLI flag) |

## Usage

```bash
aimp-discord-rpc                          # Run in background with tray icon
aimp-discord-rpc --app-id 123456789       # Override Discord App ID
aimp-discord-rpc --poll-ms 1500           # Poll AIMP every 1.5s
aimp-discord-rpc --config ./config.toml   # Custom config path
aimp-discord-rpc --verbose                # Verbose logging to console
```

## Configuration

```toml
# Your Discord application ID (required — create at discord.com/developers/applications)
app_id = 429559336982020107

# How often to poll AIMP for updates (milliseconds)
poll_interval_ms = 2000

# Discord asset keys — upload these in your app's Rich Presence settings
large_image_key = "aimp_logo"
small_image_play = "play"
small_image_pause = "pause"

# Display options
show_timestamps = true     # Show elapsed time + progress bar
show_album = true          # Show album name in large image tooltip
```

## How it works

Uses MPRIS (Media Player Remote Interfacing Specification) via D-Bus — no plugins required:

1. **D-Bus** — connects to AIMP's MPRIS interface (`org.mpris.MediaPlayer2.Player`) to read track metadata, playback status, and position
2. **Discord IPC** — connects to Discord via Unix domain socket (`discord-ipc-0`), sends Rich Presence updates with `type: Listening`
3. **System tray** — creates a tray icon with playback controls; shows album art when available

### Patched dependency

This project uses a [vendored fork](vendor/discord-rpc-client) of `discord-rpc-client` with an added `kind` field (serialized as `type`) to set the activity type to `Listening` instead of the default `Playing`.

## Troubleshooting

- **No status shown**: Make sure Discord is running and Activity Privacy has "Share detected activity with others" enabled
- **AIMP not detected**: Ensure AIMP v6.00 beta (or later) is running. Check with `playerctl -l` to see available MPRIS players
- **"Failed to connect"**: Discord might not be running, or the IPC socket is busy — the app auto-retries
- **Assets not showing**: Upload `aimp_logo`, `play`, `pause` in your Discord application settings under Rich Presence
- **Progress bar shows wrong time**: Make sure AIMP supports `playerctl position` — some beta builds may have incomplete MPRIS position support
- **Tray icon not showing**: Install `libappindicator-gtk3` / `libappindicator3-dev`
- **MPRIS errors**: Some AIMP beta builds have incomplete MPRIS support; playback controls may not work until AIMP updates

## Dependencies

| Crate | Purpose |
|-------|---------|
| `mpris` | D-Bus MPRIS client for reading track info |
| `discord-rpc-client` (patched) | Discord IPC protocol with Listening activity type |
| `tray-icon` | System tray icon and menu |
| `gtk` | GTK3 initialization for tray on Linux |
| `image` / `url` | Album art loading from MPRIS file:// URLs |
| `clap` | CLI argument parsing |
| `toml` / `serde` | Config deserialization |
| `crossbeam-channel` | Thread communication |

## License

MIT
