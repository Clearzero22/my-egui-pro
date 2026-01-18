use eframe::egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GruvboxTheme {
    Dark,
    Light,
}

impl Default for GruvboxTheme {
    fn default() -> Self {
        GruvboxTheme::Dark
    }
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

    match theme {
        GruvboxTheme::Dark => {
            visuals.dark_mode = true;
            visuals.panel_fill = GruvboxDark::BG;
            visuals.faint_bg_color = GruvboxDark::BG1;
            visuals.extreme_bg_color = GruvboxDark::BG0;
            visuals.code_bg_color = GruvboxDark::BG1;
            visuals.warn_fg_color = GruvboxDark::ORANGE;
            visuals.error_fg_color = GruvboxDark::RED;
            visuals.selection.bg_fill = GruvboxDark::BLUE;
            visuals.hyperlink_color = GruvboxDark::BLUE;
        }
        GruvboxTheme::Light => {
            visuals.dark_mode = false;
            visuals.panel_fill = GruvboxLight::BG;
            visuals.faint_bg_color = GruvboxLight::BG1;
            visuals.extreme_bg_color = GruvboxLight::BG0;
            visuals.code_bg_color = GruvboxLight::BG1;
            visuals.warn_fg_color = GruvboxLight::ORANGE;
            visuals.error_fg_color = GruvboxLight::RED;
            visuals.selection.bg_fill = GruvboxLight::BLUE;
            visuals.hyperlink_color = GruvboxLight::BLUE;
        }
    }

    // Update text styles
    let text_color = match theme {
        GruvboxTheme::Dark => GruvboxDark::FG,
        GruvboxTheme::Light => GruvboxLight::FG,
    };

    // Override text color for all text styles
    for (_text_style, font_id) in &mut style.text_styles {
        // We can't directly set color on font_id, so we set override_text_color
    }
    visuals.override_text_color = Some(text_color);

    // Button colors
    match theme {
        GruvboxTheme::Dark => {
            visuals.widgets.active.bg_fill = GruvboxDark::BG2;
            visuals.widgets.hovered.bg_fill = GruvboxDark::BG1;
            visuals.widgets.inactive.bg_fill = GruvboxDark::BG;
            visuals.widgets.active.fg_stroke.color = GruvboxDark::FG;
            visuals.widgets.hovered.fg_stroke.color = GruvboxDark::FG;
            visuals.widgets.inactive.fg_stroke.color = GruvboxDark::FG3;
            visuals.widgets.noninteractive.bg_fill = GruvboxDark::BG;
            visuals.widgets.noninteractive.fg_stroke.color = GruvboxDark::FG3;
        }
        GruvboxTheme::Light => {
            visuals.widgets.active.bg_fill = GruvboxLight::BG2;
            visuals.widgets.hovered.bg_fill = GruvboxLight::BG1;
            visuals.widgets.inactive.bg_fill = GruvboxLight::BG;
            visuals.widgets.active.fg_stroke.color = GruvboxLight::FG;
            visuals.widgets.hovered.fg_stroke.color = GruvboxLight::FG;
            visuals.widgets.inactive.fg_stroke.color = GruvboxLight::FG3;
            visuals.widgets.noninteractive.bg_fill = GruvboxLight::BG;
            visuals.widgets.noninteractive.fg_stroke.color = GruvboxLight::FG3;
        }
    }

    // Selection
    match theme {
        GruvboxTheme::Dark => {
            visuals.selection.bg_fill = GruvboxDark::BLUE;
            visuals.selection.stroke.color = GruvboxDark::BLUE;
        }
        GruvboxTheme::Light => {
            visuals.selection.bg_fill = GruvboxLight::BLUE;
            visuals.selection.stroke.color = GruvboxLight::BLUE;
        }
    }

    ctx.set_style(style);
}
