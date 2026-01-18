# Gruvbox Theme Design

**Date:** 2025-01-18
**Author:** Claude Code
**Status:** Approved

## Overview

Implement Gruvbox color scheme (light and dark variants) for the Hacker News egui app, with a sidebar toggle to switch between themes and persistent storage of user preference.

## Requirements

### Functional Requirements
- Toggle between Gruvbox Dark and Gruvbox Light themes
- Theme toggle button in sidebar
- Remember theme choice between sessions
- Apply colors to all egui UI components

### Non-Functional Requirements
- Minimal performance impact on render loop
- Clean color definitions following Gruvbox spec
- Graceful fallback if config file is missing

## Color Palette

### Gruvbox Dark

```
Backgrounds:
  bg_hard:     #1d2021
  bg:          #282828
  bg0:         #1c1c1c
  bg1:         #3c3836
  bg2:         #504945
  bg3:         #665c54
  bg4:         #7c6f64

Foregrounds:
  fg:          #ebdbb2
  fg0:         #fbf1c7
  fg2:         #d5c4a1
  fg3:         #bdae93
  fg4:         #a89984

Accents:
  red:         #fb4934
  green:       #b8bb26
  yellow:      #fabd2f
  blue:        #83a598
  purple:      #d3869b
  aqua:        #8ec07c
  orange:      #fe8019
  gray:        #928374
```

### Gruvbox Light

```
Backgrounds (lighter):
  bg_hard:     #f9f5d7
  bg:          #fbf1c7
  bg0:         #f2e5bc
  bg1:         #ebdbb2
  bg2:         #d5c4a1
  bg3:         #bdae93
  bg4:         #a89984

Foregrounds (darker):
  fg:          #3c3836
  fg0:         #1d2021
  fg2:         #504945
  fg3:         #665c54
  fg4:         #7c6f64

Accents (darker variants):
  red:         #9d0006
  green:       #79740e
  yellow:      #b57614
  blue:        #076678
  purple:      #8f3f71
  aqua:        #427b58
  orange:      #af3a03
  gray:        #7c6f64
```

## Data Structures

### Theme Enum
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GruvboxTheme {
    Dark,
    Light,
}

impl Default for GruvboxTheme {
    fn default() -> Self {
        GruvboxTheme::Dark  // Default to dark theme
    }
}
```

### Color Constants
```rust
pub struct GruvboxDark;
pub struct GruvboxLight;

impl GruvboxDark {
    pub const BG: Color32 = Color32::from_rgb(0x28, 0x28, 0x28);
    pub const FG: Color32 = Color32::from_rgb(0xeb, 0xdb, 0xb2);
    pub const BG1: Color32 = Color32::from_rgb(0x3c, 0x38, 0x36);
    pub const RED: Color32 = Color32::from_rgb(0xfb, 0x49, 0x34);
    pub const GREEN: Color32 = Color32::from_rgb(0xb8, 0xbb, 0x26);
    pub const YELLOW: Color32 = Color32::from_rgb(0xfa, 0xbd, 0x2f);
    pub const BLUE: Color32 = Color32::from_rgb(0x83, 0xa5, 0x98);
    pub const PURPLE: Color32 = Color32::from_rgb(0xd3, 0x86, 0x9b);
    pub const AQUA: Color32 = Color32::from_rgb(0x8e, 0xc0, 0x7c);
    pub const ORANGE: Color32 = Color32::from_rgb(0xfe, 0x80, 0x19);
    pub const GRAY: Color32 = Color32::from_rgb(0x92, 0x83, 0x74);
}

impl GruvboxLight {
    pub const BG: Color32 = Color32::from_rgb(0xfb, 0xf1, 0xc7);
    pub const FG: Color32 = Color32::from_rgb(0x3c, 0x38, 0x36);
    pub const BG1: Color32 = Color32::from_rgb(0xeb, 0xdb, 0xb2);
    pub const RED: Color32 = Color32::from_rgb(0x9d, 0x00, 0x06);
    pub const GREEN: Color32 = Color32::from_rgb(0x79, 0x74, 0x0e);
    pub const YELLOW: Color32 = Color32::from_rgb(0xb5, 0x76, 0x14);
    pub const BLUE: Color32 = Color32::from_rgb(0x07, 0x66, 0x78);
    pub const PURPLE: Color32 = Color32::from_rgb(0x8f, 0x3f, 0x71);
    pub const AQUA: Color32 = Color32::from_rgb(0x42, 0x7b, 0x58);
    pub const ORANGE: Color32 = Color32::from_rgb(0xaf, 0x3a, 0x03);
    pub const GRAY: Color32 = Color32::from_rgb(0x7c, 0x6f, 0x64);
}
```

### Config Structure
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: GruvboxTheme,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: GruvboxTheme::Dark,
        }
    }
}
```

## File Structure

```
src/
├── theme.rs          -- New: Color definitions, theme enum
├── config.rs         -- New: Config persistence
├── app.rs            -- Add theme state, apply_theme()
├── main.rs           -- Apply theme on startup
└── ui.rs             -- Add theme toggle button
```

## UI Design

### Sidebar Theme Toggle
Located below the View toggle:

```
Sidebar Layout:
┌─────────────────┐
│ View            │
│ [Fetched] [Saved]│
│ ───────────────  │
│ Categories      │
│ ...             │
│ ───────────────  │
│ Theme           │
│ 🌙 Dark         │  ← Toggle button
└─────────────────┘
```

## Data Flow

### Startup Flow
```
1. App::new() called
2. Config::load() → read ~/.local/share/my_egui_pro/config.json
3. Parse theme from config (default: Dark)
4. Apply theme to egui context
5. Render UI with applied theme
```

### Theme Toggle Flow
```
1. User clicks theme button in sidebar
2. app.toggle_theme() → Dark ↔ Light
3. config.save() → write new theme to config.json
4. app.apply_theme() → update egui style
5. ctx.request_repaint() → UI updates immediately
```

## Implementation Notes

### Config File
- **Location:** `~/.local/share/my_egui_pro/config.json`
- **Format:** JSON (simple key-value)
- **Error handling:** Use default theme if file missing/corrupt

### Theme Application
Apply theme in the update loop or after state changes:

```rust
impl HackerNewsApp {
    pub fn apply_theme(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        let visuals = &mut style.visuals;

        match self.theme {
            GruvboxTheme::Dark => {
                visuals.dark_mode = true;
                visuals.panel_fill = GruvboxDark::BG;
                visuals.faint_bg_color = GruvboxDark::BG1;
                visuals.extreme_bg_color = GruvboxDark::BG;
                visuals.code_bg_color = GruvboxDark::BG1;
                visuals.warn_fg_color = GruvboxDark::ORANGE;
                visuals.error_fg_color = GruvboxDark::RED;
                visuals.highlight_color = GruvboxDark::BLUE;
            }
            GruvboxTheme::Light => {
                visuals.dark_mode = false;
                visuals.panel_fill = GruvboxLight::BG;
                visuals.faint_bg_color = GruvboxLight::BG1;
                visuals.extreme_bg_color = GruvboxLight::BG;
                visuals.code_bg_color = GruvboxLight::BG1;
                visuals.warn_fg_color = GruvboxLight::ORANGE;
                visuals.error_fg_color = GruvboxLight::RED;
                visuals.highlight_color = GruvboxLight::BLUE;
            }
        }

        // Text colors
        style.text_styles.get_mut(&TextStyle::Body)
            .unwrap().color = match self.theme {
                GruvboxTheme::Dark => GruvboxDark::FG,
                GruvboxTheme::Light => GruvboxLight::FG,
            };

        ctx.set_style(style);
    }
}
```

## Dependencies

No new dependencies required:
- `serde` / `serde_json` - Already used for storage
- `dirs` - Already used for config directory
- `egui` - Built-in theming support

## Build & Run

```bash
cargo run
```

First run:
1. Creates `~/.local/share/my_egui_pro/config.json`
2. Defaults to Dark theme
3. Theme persists across restarts

## Future Enhancements

Out of scope for this implementation (YAGNI):
- Custom color pickers
- Additional color schemes beyond Gruvbox
- Per-category theme customization
- Automatic theme switching based on system preference
