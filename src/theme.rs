use eframe::egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GruvboxTheme {
    Dark,
    Light,
}

impl GruvboxTheme {
    pub fn display_name(&self) -> &'static str {
        match self {
            GruvboxTheme::Dark => "🌙 Dark",
            GruvboxTheme::Light => "☀️ Light",
        }
    }

    fn colors(&self) -> ThemeColors {
        match self {
            GruvboxTheme::Dark => ThemeColors {
                dark_mode: true,
                bg: GruvboxDark::BG,
                bg0: GruvboxDark::BG0,
                bg1: GruvboxDark::BG1,
                bg2: GruvboxDark::BG2,
                fg: GruvboxDark::FG,
                fg3: GruvboxDark::FG3,
                red: GruvboxDark::RED,
                blue: GruvboxDark::BLUE,
                orange: GruvboxDark::ORANGE,
            },
            GruvboxTheme::Light => ThemeColors {
                dark_mode: false,
                bg: GruvboxLight::BG,
                bg0: GruvboxLight::BG0,
                bg1: GruvboxLight::BG1,
                bg2: GruvboxLight::BG2,
                fg: GruvboxLight::FG,
                fg3: GruvboxLight::FG3,
                red: GruvboxLight::RED,
                blue: GruvboxLight::BLUE,
                orange: GruvboxLight::ORANGE,
            },
        }
    }
}

impl Default for GruvboxTheme {
    fn default() -> Self {
        GruvboxTheme::Dark
    }
}

struct ThemeColors {
    dark_mode: bool,
    bg: Color32,
    bg0: Color32,
    bg1: Color32,
    bg2: Color32,
    fg: Color32,
    fg3: Color32,
    red: Color32,
    blue: Color32,
    orange: Color32,
}

pub struct GruvboxDark;

impl GruvboxDark {
    pub const BG: Color32 = Color32::from_rgb(0x28, 0x28, 0x28);
    pub const BG0: Color32 = Color32::from_rgb(0x1c, 0x1c, 0x1c);
    pub const BG1: Color32 = Color32::from_rgb(0x3c, 0x38, 0x36);
    pub const BG2: Color32 = Color32::from_rgb(0x50, 0x49, 0x45);
    pub const BG3: Color32 = Color32::from_rgb(0x66, 0x5c, 0x54);
    pub const BG4: Color32 = Color32::from_rgb(0x7c, 0x6f, 0x64);

    pub const FG: Color32 = Color32::from_rgb(0xeb, 0xdb, 0xb2);
    pub const FG0: Color32 = Color32::from_rgb(0xfb, 0xf1, 0xc7);
    pub const FG2: Color32 = Color32::from_rgb(0xd5, 0xc4, 0xa1);
    pub const FG3: Color32 = Color32::from_rgb(0xbd, 0xae, 0x93);
    pub const FG4: Color32 = Color32::from_rgb(0xa8, 0x99, 0x84);

    pub const RED: Color32 = Color32::from_rgb(0xfb, 0x49, 0x34);
    pub const GREEN: Color32 = Color32::from_rgb(0xb8, 0xbb, 0x26);
    pub const YELLOW: Color32 = Color32::from_rgb(0xfa, 0xbd, 0x2f);
    pub const BLUE: Color32 = Color32::from_rgb(0x83, 0xa5, 0x98);
    pub const PURPLE: Color32 = Color32::from_rgb(0xd3, 0x86, 0x9b);
    pub const AQUA: Color32 = Color32::from_rgb(0x8e, 0xc0, 0x7c);
    pub const ORANGE: Color32 = Color32::from_rgb(0xfe, 0x80, 0x19);
    pub const GRAY: Color32 = Color32::from_rgb(0x92, 0x83, 0x74);
}

pub struct GruvboxLight;

impl GruvboxLight {
    pub const BG: Color32 = Color32::from_rgb(0xfb, 0xf1, 0xc7);
    pub const BG0: Color32 = Color32::from_rgb(0xf2, 0xe5, 0xbc);
    pub const BG1: Color32 = Color32::from_rgb(0xeb, 0xdb, 0xb2);
    pub const BG2: Color32 = Color32::from_rgb(0xd5, 0xc4, 0xa1);
    pub const BG3: Color32 = Color32::from_rgb(0xbd, 0xae, 0x93);
    pub const BG4: Color32 = Color32::from_rgb(0xa8, 0x99, 0x84);

    pub const FG: Color32 = Color32::from_rgb(0x3c, 0x38, 0x36);
    pub const FG0: Color32 = Color32::from_rgb(0x1d, 0x20, 0x21);
    pub const FG2: Color32 = Color32::from_rgb(0x50, 0x49, 0x45);
    pub const FG3: Color32 = Color32::from_rgb(0x66, 0x5c, 0x54);
    pub const FG4: Color32 = Color32::from_rgb(0x7c, 0x6f, 0x64);

    pub const RED: Color32 = Color32::from_rgb(0x9d, 0x00, 0x06);
    pub const GREEN: Color32 = Color32::from_rgb(0x79, 0x74, 0x0e);
    pub const YELLOW: Color32 = Color32::from_rgb(0xb5, 0x76, 0x14);
    pub const BLUE: Color32 = Color32::from_rgb(0x07, 0x66, 0x78);
    pub const PURPLE: Color32 = Color32::from_rgb(0x8f, 0x3f, 0x71);
    pub const AQUA: Color32 = Color32::from_rgb(0x42, 0x7b, 0x58);
    pub const ORANGE: Color32 = Color32::from_rgb(0xaf, 0x3a, 0x03);
    pub const GRAY: Color32 = Color32::from_rgb(0x7c, 0x6f, 0x64);
}

pub fn apply_theme(ctx: &eframe::egui::Context, theme: &GruvboxTheme) {
    let mut style = (*ctx.style()).clone();
    let visuals = &mut style.visuals;
    let c = theme.colors();

    // Base colors
    visuals.dark_mode = c.dark_mode;
    visuals.panel_fill = c.bg;
    visuals.faint_bg_color = c.bg1;
    visuals.extreme_bg_color = c.bg0;
    visuals.code_bg_color = c.bg1;
    visuals.warn_fg_color = c.orange;
    visuals.error_fg_color = c.red;
    visuals.override_text_color = Some(c.fg);

    // Interactive elements
    visuals.selection.bg_fill = c.blue;
    visuals.selection.stroke.color = c.blue;
    visuals.hyperlink_color = c.blue;

    // Button colors
    visuals.widgets.active.bg_fill = c.bg2;
    visuals.widgets.hovered.bg_fill = c.bg1;
    visuals.widgets.inactive.bg_fill = c.bg;
    visuals.widgets.active.fg_stroke.color = c.fg;
    visuals.widgets.hovered.fg_stroke.color = c.fg;
    visuals.widgets.inactive.fg_stroke.color = c.fg3;
    visuals.widgets.noninteractive.bg_fill = c.bg;
    visuals.widgets.noninteractive.fg_stroke.color = c.fg3;

    ctx.set_style(style);
}
