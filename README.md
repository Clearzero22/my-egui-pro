# my_egui_pro - Hacker News Reader

A modern desktop application for browsing Hacker News, built with Rust and egui.

![Screenshot](screenshot.png)

## Features

- 📰 **6 Categories**: Browse Top, New, Best, Ask, Show, and Jobs stories
- 🔗 **Click to Open**: Click any story title to open it in your default browser
- ⭐ **Favorites**: Save your favorite stories with SQLite storage (sorted by date)
- 🎨 **Gruvbox Theme**: Beautiful dark/light theme toggle
- 🚀 **Fast & Lightweight**: Built with Rust and egui for native performance

## Screenshots

### Dark Theme (Default)
- Clean, eye-friendly dark interface with Gruvbox colors

### Light Theme
- Bright alternative for well-lit environments

## Installation

### Build from Source

**Prerequisites:**
- Rust toolchain (1.70+)
- Cargo

**Build:**
```bash
cargo build --release
```

**Run:**
```bash
./target/release/my_egui_pro
```

### Download Release Binaries

Grab the latest release from the [Releases](https://github.com/Clearzero22/my_egui_pro/releases) page.

**Linux/macOS:**
```bash
tar -xzf my_egui_pro-0.1.0-linux-amd64.tar.gz
cd my_egui_pro-0.1.0-linux-amd64
./my_egui_pro
```

**Windows:**
```powershell
Expand-Archive -Path my_egui_pro-0.1.0-windows-amd64.zip
cd my_egui_pro-0.1.0-windows-amd64
.\my_egui_pro.exe
```

## Usage

1. **Select a Category**: Use the sidebar to switch between Top, New, Best, Ask, Show, and Jobs
2. **Read Stories**: Click on any story title to open it in your browser
3. **Save Favorites**: Click the star icon to save stories to your favorites
4. **Toggle Theme**: Click the theme button to switch between dark and light themes
5. **View Saved**: Switch to "Saved" tab to see your favorited stories

## Data Location

Your data is stored locally:

- **Linux/macOS**: `~/.local/share/my_egui_pro/`
  - `config.json` - Theme preferences
  - `favorites.db` - Saved stories database

- **Windows**: `%LOCALAPPDATA%\my_egui_pro\`
  - `config.json` - Theme preferences
  - `favorites.db` - Saved stories database

## Development

**Run tests:**
```bash
cargo test
```

**Run with debug output:**
```bash
RUST_LOG=debug cargo run
```

**Format code:**
```bash
cargo fmt
```

**Run linter:**
```bash
cargo clippy
```

## Building Releases

Use the provided scripts to build release packages:

**Linux/macOS:**
```bash
./scripts/release.sh
```

**Windows:**
```cmd
release.bat
```

Or use GitHub Actions - push a tag to trigger automated builds for all platforms.

## Architecture

```
src/
├── main.rs      # Entry point
├── app.rs       # Main application state
├── category.rs  # Category enum
├── config.rs    # Configuration persistence
├── hn_api.rs    # Hacker News API client
├── storage.rs   # SQLite favorites database
├── story.rs     # Story model
├── theme.rs     # Gruvbox theme implementation
└── ui.rs        # UI rendering
```

## Technology Stack

- **[eframe](https://github.com/emilk/eframe)** - egui framework
- **[egui](https://github.com/emilk/egui)** - Immediate mode GUI
- **[reqwest](https://github.com/seanmonstar/reqwest)** - HTTP client
- **[tokio](https://github.com/tokio-rs/tokio)** - Async runtime
- **[rusqlite](https://github.com/rusqlite/rusqlite)** - SQLite database
- **[time](https://github.com/time-rs/time)** - Time handling

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- [Hacker News](https://news.ycombinator.com/) - For the amazing content
- [egui](https://github.com/emilk/egui) - For the excellent GUI framework
- [Gruvbox](https://github.com/morhetz/gruvbox) - For the beautiful color scheme
