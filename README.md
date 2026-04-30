# 🎮 SonicHub Launcher

A unified mod manager and game launcher for Sonic fan projects on Linux.

![GTK4](https://img.shields.io/badge/GTK4-Linux-blue) ![Rust](https://img.shields.io/badge/Rust-1.70+-orange) ![License](https://img.shields.io/badge/license-MIT-green)

## ✨ Features

- **Multi-Engine Support** — Launch and manage games across multiple Sonic engines:
  - RSDKv3 (Sonic CD)
  - RSDKv4 (Sonic 1/2 2013)
  - RSDKv5 (Sonic Mania)
  - Sonic 1 Forever (via Wine)
  - Sonic 2 Absolute (via Wine)
  - Sonic 3 A.I.R. (Flatpak / Native)

- **🌐 GameBanana Integration** — Browse, search, and install mods directly from GameBanana
- **📦 One-Click Mod Install** — Supports `.zip` and `.7z` archives with automatic extraction
- **🔄 Mod Management** — Enable/disable mods with toggles, delete with confirmation
- **🖼️ Custom Banners** — Set custom banner images for each game profile
- **🔍 Smart Detection** — Detects already-installed mods, auto-copies DLLs for Mania mods
- **🎨 Modern UI** — Dark theme with smooth design, built with GTK4

## 📸 Screenshots

*Coming soon*

## 🚀 Installation

### Pre-built Binary (Linux x86_64)

```bash
# Download and run
chmod +x sonichub-launcher
./sonichub-launcher
```

### Build from Source

#### Dependencies

```bash
# Ubuntu/Debian/Mint
sudo apt install build-essential libgtk-4-dev p7zip-full

# Fedora
sudo dnf install gtk4-devel p7zip

# Arch
sudo pacman -S gtk4 p7zip
```

#### Build

```bash
git clone https://github.com/RenatskaYT/sonichub-launcher.git
cd sonichub-launcher
cargo build --release
./target/release/sonichub-launcher
```

## 🎯 Quick Start

1. Click **＋ Add Game** in the sidebar
2. Enter game name, select engine type
3. Set paths to executable and Data.rsdk (if needed)
4. Set your mods folder path
5. Click **🌐 GameBanana** to browse and install mods!

### Engine-Specific Notes

| Engine | Executable | Data.rsdk | Launch Method |
|--------|-----------|-----------|---------------|
| RSDKv3/v4/v5 | Native Linux binary | Required | Direct |
| Sonic 1 Forever | `.exe` file | Required | Wine |
| Sonic 2 Absolute | `.exe` file | Not needed | Wine |
| Sonic 3 A.I.R. | Command (e.g. `flatpak run org.sonic3air.Sonic3AIR`) | Not needed | Command |

## 🔧 Technical Details

- **Language:** Rust
- **UI Framework:** GTK4 (native Linux)
- **Mod Config:** Compatible with RSDKv5 `modconfig.ini` format (`[Mods]` section, `y`/`n` values)
- **Archive Support:** Native ZIP + 7z fallback
- **Thumbnail Cache:** FNV-1a hashed, stored in `~/.cache/rsdk-launcher/thumbs/`

## 📋 Requirements

- Linux (x86_64)
- GTK4 runtime libraries
- Wine (for Sonic 1 Forever / Sonic 2 Absolute)
- p7zip-full (optional, for .7z mod archives)

## 🤝 Contributing

Pull requests welcome! Feel free to open issues for bugs or feature requests.

## 📄 License

MIT License
