# 🎮 SonicHub — RSDK & Fan Game Launcher

All-in-one mod manager and game launcher for Sonic fan projects on Linux.

Browse mods on GameBanana, install them in one click, toggle on/off, delete — all from a single app.

---

## What it does

- 🚀 **Launch games** — RSDKv3/v4/v5, Sonic 1 Forever, Sonic 2 Absolute, Sonic 3 A.I.R.
- 🌐 **GameBanana browser** — search mods, see previews and descriptions, paginate through results
- 📦 **One-click install** — download, extract (zip/7z), enable — fully automatic
- 🔄 **Mod management** — toggle mods on/off, delete with confirmation
- 🔍 **Smart detection** — knows which mods are already installed, auto-copies DLLs for Mania
- 🍷 **Wine support** — launches Forever/Absolute through Wine automatically
- ✈️ **Flatpak support** — launch S3 A.I.R. via `flatpak run` or any custom command
- 🎨 **Clean UI** — dark theme, GTK4, feels native on Linux

---

## Screenshots

*Coming soon*

---

## Installation

### Pre-built binary (easiest)

Grab it from [Releases](https://github.com/renatskayt/SonicHUB-RSDK-and-Fan-Game-Launcher-/releases), make it executable, and run:

```bash
chmod +x sonichub-launcher
./sonichub-launcher
```

That's it. Only needs GTK4 (already installed on most distros).

### Build from source

You'll need Rust and a few packages:

```bash
# Ubuntu/Mint/Debian
sudo apt install build-essential libgtk-4-dev p7zip-full

# Fedora
sudo dnf install gtk4-devel p7zip

# Arch
sudo pacman -S gtk4 p7zip
```

Then:

```bash
git clone https://github.com/renatskayt/SonicHUB-RSDK-and-Fan-Game-Launcher-.git
cd SonicHUB-RSDK-and-Fan-Game-Launcher-
cargo build --release
./target/release/sonichub-launcher
```

> **Don't have Rust?** Install it with: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

---

## How to use

1. Click **＋ Add Game** in the sidebar
2. Pick your engine (RSDK v3/v4/v5, Forever, Absolute, AIR)
3. Set the paths to the executable and Data.rsdk (if needed)
4. Set your mods folder
5. Hit **🌐 GameBanana** — search and install mods with one click!

### Engine setup cheat sheet

| Engine | What to set | How it launches |
|--------|------------|-----------------|
| RSDKv3/v4/v5 | exe + Data.rsdk | Native |
| Sonic 1 Forever | .exe + Data.rsdk | Wine |
| Sonic 2 Absolute | .exe only | Wine |
| Sonic 3 A.I.R. | launch command | `flatpak run ...` or direct |

---

## Technical stuff

- **Language:** Rust 🦀
- **UI:** GTK4
- **Mod config format:** RSDKv5-compatible (`[Mods]` section, `y`/`n` values)
- **Archives:** native ZIP + 7z fallback
- **Thumbnail cache:** `~/.cache/rsdk-launcher/thumbs/`
- **Game config:** `~/.config/rsdk-launcher/config.json`

## Requirements

- Linux x86_64
- GTK4 runtime (usually pre-installed)
- Wine (for Forever / Absolute)
- p7zip-full (optional, for .7z archives)

---

## License

MIT — do whatever you want, just credit the author.
