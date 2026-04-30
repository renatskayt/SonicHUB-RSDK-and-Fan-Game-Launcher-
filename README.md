# 🎮 SonicHub — RSDK & Fan Game Launcher

Универсальный лаунчер и мод-менеджер для фанатских Sonic-проектов на Linux.

Всё в одном месте: запуск игр, скачивание модов с GameBanana, включение/отключение, удаление — без боли.

---

## Что умеет

- 🚀 **Запуск игр** — RSDKv3/v4/v5, Sonic 1 Forever, Sonic 2 Absolute, Sonic 3 A.I.R.
- 🌐 **GameBanana** — встроенный браузер модов с превью, описанием и поиском
- 📦 **Установка модов** — скачал, распаковал (zip/7z), включил — всё автоматически
- 🔄 **Управление модами** — тоглы вкл/выкл, кнопка удаления с подтверждением
- 🔍 **Умный детект** — видит уже установленные моды, копирует DLL для Mania
- 🎨 **Красивый UI** — тёмная тема, GTK4, выглядит нативно

---

## Скриншоты

*Скоро будут*

---

## Установка

### Готовый бинарь (самый простой способ)

Качаешь из [Releases](https://github.com/renatskayt/SonicHUB-RSDK-and-Fan-Game-Launcher-/releases), даёшь права и запускаешь:

```bash
chmod +x sonichub-launcher
./sonichub-launcher
```

Всё. Никаких зависимостей кроме GTK4 (который уже есть на большинстве дистрибутивов).

### Сборка из исходников

Если хочешь собрать сам — нужен Rust и пара пакетов:

```bash
# Зависимости (Ubuntu/Mint/Debian)
sudo apt install build-essential libgtk-4-dev p7zip-full

# Fedora
sudo dnf install gtk4-devel p7zip

# Arch
sudo pacman -S gtk4 p7zip
```

Дальше:

```bash
git clone https://github.com/renatskayt/SonicHUB-RSDK-and-Fan-Game-Launcher-.git
cd SonicHUB-RSDK-and-Fan-Game-Launcher-
cargo build --release
./target/release/sonichub-launcher
```

> **Нет Rust?** Ставится одной командой: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

---

## Как пользоваться

1. Нажми **＋ Add Game** в боковой панели
2. Выбери движок (RSDK v3/v4/v5, Forever, Absolute, AIR)
3. Укажи пути к exe и Data.rsdk (если нужно)
4. Укажи папку модов
5. Жми **🌐 GameBanana** — ищи и ставь моды в один клик!

### Какой движок — какие настройки

| Движок | Что указывать | Как запускается |
|--------|--------------|-----------------|
| RSDKv3/v4/v5 | exe + Data.rsdk | Напрямую |
| Sonic 1 Forever | exe + Data.rsdk | Через Wine |
| Sonic 2 Absolute | только exe | Через Wine |
| Sonic 3 A.I.R. | команда запуска | `flatpak run ...` или напрямую |

---

## Тех. детали

- **Язык:** Rust 🦀
- **UI:** GTK4
- **Формат конфига модов:** совместим с RSDKv5 (`[Mods]`, `y`/`n`)
- **Архивы:** нативный ZIP + 7z как фолбэк
- **Кеш превью:** `~/.cache/rsdk-launcher/thumbs/`
- **Конфиг игр:** `~/.config/rsdk-launcher/config.json`

## Требования

- Linux x86_64
- GTK4 (обычно уже стоит)
- Wine (если играешь в Forever/Absolute)
- p7zip-full (опционально, для .7z архивов)

---

## Лицензия

MIT — делай что хочешь, просто укажи автора.
