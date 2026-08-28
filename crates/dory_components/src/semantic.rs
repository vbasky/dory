//! Semantic color tokens for banners and data-grid row states.
//!
//! These tokens are hand-picked per-theme (Ayu Dark, Mirage, Light) to ensure
//! legibility across all three palettes. They are NOT derived at runtime from
//! `theme.*` opacity calculations — the hex values are embedded here.
//!
//! # Usage
//!
//! ```
//! use dory_components::semantic::{BannerColors, RowStateColors, ThemeSettingGlobal};
//! ```
//!
//! Register the current theme once during startup via `ThemeSettingGlobal::set`.
//! Then call `BannerColors::for_current(cx)` or `RowStateColors::for_current(cx)`
//! in any rendering context.

use dory_core::{FontSetting, ThemeSetting};
use gpui::{App, Global, Hsla, hsla};
use gpui_component::ActiveTheme;

// ---------------------------------------------------------------------------
// Hex helpers
// ---------------------------------------------------------------------------

fn hex(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;

    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let l = (max + min) / 2.0;

    if (max - min).abs() < f32::EPSILON {
        return hsla(0.0, 0.0, l, a);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if (max - rf).abs() < f32::EPSILON {
        let mut h = (gf - bf) / d;
        if gf < bf {
            h += 6.0;
        }
        h
    } else if (max - gf).abs() < f32::EPSILON {
        (bf - rf) / d + 2.0
    } else {
        (rf - gf) / d + 4.0
    };

    hsla(h / 6.0, s, l, a)
}

fn from_hex(hex_value: u32, alpha: f32) -> Hsla {
    let r = ((hex_value >> 16) & 0xFF) as u8;
    let g = ((hex_value >> 8) & 0xFF) as u8;
    let b = (hex_value & 0xFF) as u8;
    hex(r, g, b, alpha)
}

// ---------------------------------------------------------------------------
// GPUI global — tracks the active ThemeSetting
// ---------------------------------------------------------------------------

/// GPUI global tracking the active `ThemeSetting`.
///
/// Register once during startup (after `theme::apply_theme`) by calling
/// `ThemeSettingGlobal::set(cx, setting)`. Semantic color accessors use it
/// to select the correct token values for the active palette.
#[derive(Debug, Clone, Copy)]
pub struct ThemeSettingGlobal {
    pub setting: ThemeSetting,
}

impl Global for ThemeSettingGlobal {}

impl ThemeSettingGlobal {
    /// Register (or update) the active `ThemeSetting` in the GPUI context.
    pub fn set(cx: &mut App, setting: ThemeSetting) {
        cx.set_global(ThemeSettingGlobal { setting });
    }

    /// Read the active `ThemeSetting`. Falls back to `ThemeSetting::DoryDark` when
    /// the global has not been registered.
    pub fn get(cx: &App) -> ThemeSetting {
        cx.try_global::<Self>()
            .map(|g| g.setting)
            .unwrap_or(ThemeSetting::DoryDark)
    }
}

// ---------------------------------------------------------------------------
// GPUI global — tracks the active FontSetting
// ---------------------------------------------------------------------------

/// GPUI global tracking the active `FontSetting`.
///
/// Register during startup (after config load) via `FontSettingGlobal::set`.
/// Typography roles use it to choose the platform system font or a named
/// installed family for non-mono UI text.
#[derive(Debug, Clone)]
pub struct FontSettingGlobal {
    pub setting: FontSetting,
}

impl Global for FontSettingGlobal {}

impl FontSettingGlobal {
    /// Register (or update) the active `FontSetting` in the GPUI context.
    pub fn set(cx: &mut App, setting: FontSetting) {
        cx.set_global(FontSettingGlobal { setting });
    }

    /// Read the active `FontSetting`. Falls back to the platform system font
    /// when the global has not been registered.
    pub fn get(cx: &App) -> FontSetting {
        cx.try_global::<Self>()
            .map(|g| g.setting.clone())
            .unwrap_or_else(FontSetting::system)
    }
}

// ---------------------------------------------------------------------------
// BannerColors
// ---------------------------------------------------------------------------

/// Semantic colors for informational banners (info, success, warning, error).
///
/// Each variant exposes a `background` (low-chroma tinted surface) and
/// `foreground` (high-contrast text/icon color) that are legible on top of it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BannerColors {
    /// Background and foreground for an informational banner.
    pub info_bg: Hsla,
    pub info_fg: Hsla,
    /// Background and foreground for a success banner.
    pub success_bg: Hsla,
    pub success_fg: Hsla,
    /// Background and foreground for a warning banner.
    pub warning_bg: Hsla,
    pub warning_fg: Hsla,
    /// Background and foreground for an error/danger banner.
    pub error_bg: Hsla,
    pub error_fg: Hsla,
}

impl BannerColors {
    /// Select tokens for the Ayu Dark palette.
    pub fn dark() -> Self {
        Self {
            // #59C2FF at 12% over dark background
            info_bg: from_hex(0x59C2FF, 0.12),
            info_fg: from_hex(0x59C2FF, 1.0),
            // #AAD94C at 12% over dark background
            success_bg: from_hex(0xAAD94C, 0.12),
            success_fg: from_hex(0xAAD94C, 1.0),
            // #FFB454 at 12% over dark background
            warning_bg: from_hex(0xFFB454, 0.12),
            warning_fg: from_hex(0xFFB454, 1.0),
            // #F07178 at 12% over dark background
            error_bg: from_hex(0xF07178, 0.12),
            error_fg: from_hex(0xF07178, 1.0),
        }
    }

    /// Select tokens for the Ayu Mirage palette.
    pub fn mirage() -> Self {
        Self {
            // #73D0FF at 14% over mirage background — slightly more opaque for contrast
            info_bg: from_hex(0x73D0FF, 0.14),
            info_fg: from_hex(0x73D0FF, 1.0),
            // #AAD94C at 14%
            success_bg: from_hex(0xAAD94C, 0.14),
            success_fg: from_hex(0xAAD94C, 1.0),
            // #FFCC66 at 14%
            warning_bg: from_hex(0xFFCC66, 0.14),
            warning_fg: from_hex(0xFFCC66, 1.0),
            // #F28779 at 14%
            error_bg: from_hex(0xF28779, 0.14),
            error_fg: from_hex(0xF28779, 1.0),
        }
    }

    /// Select tokens for the Ayu Light palette.
    pub fn light() -> Self {
        Self {
            // #399EE6 at 10% over light background — low saturation tint
            info_bg: from_hex(0x399EE6, 0.10),
            info_fg: from_hex(0x2A7BBF, 1.0),
            // #86B300 at 10%
            success_bg: from_hex(0x86B300, 0.10),
            success_fg: from_hex(0x6A8F00, 1.0),
            // #F2AE49 at 10%
            warning_bg: from_hex(0xF2AE49, 0.10),
            warning_fg: from_hex(0xC07800, 1.0),
            // #E65050 at 10%
            error_bg: from_hex(0xE65050, 0.10),
            error_fg: from_hex(0xBF3030, 1.0),
        }
    }

    /// Return the `BannerColors` that reproduce exactly what the former
    /// `tokens::BannerColors` produced for all 9 call-sites.
    ///
    /// - `info`, `success`, `error`: theme-agnostic fixed hex values taken
    ///   verbatim from the former `tokens::BannerColors` implementation.
    /// - `warning`: derived from `theme.primary` at runtime exactly as the
    ///   former implementation did (bg = primary @ 0.20 alpha,
    ///   fg = primary @ 1.0 alpha).
    ///
    /// The named constructors `dark()`, `mirage()`, and `light()` carry
    /// per-palette semantic values intended for future use. Call sites that
    /// need pixel-exact backwards compatibility MUST call this method instead.
    pub fn for_current(cx: &App) -> Self {
        let theme = cx.theme();
        let mut warning_bg = theme.primary;
        warning_bg.a = 0.20;
        let mut warning_fg = theme.primary;
        warning_fg.a = 1.0;

        Self {
            // #1E3A5F / #93C5FD — former tokens::BannerColors::info_*
            info_bg: from_hex(0x1E3A5F, 1.0),
            info_fg: from_hex(0x93C5FD, 1.0),
            // #14532D / #86EFAC — former tokens::BannerColors::success_*
            success_bg: from_hex(0x14532D, 1.0),
            success_fg: from_hex(0x86EFAC, 1.0),
            // theme.primary @ 0.20 / 1.0 — former tokens::BannerColors::warning_*
            warning_bg,
            warning_fg,
            // #7F1D1D / #FCA5A5 — former tokens::BannerColors::danger_*
            error_bg: from_hex(0x7F1D1D, 1.0),
            error_fg: from_hex(0xFCA5A5, 1.0),
        }
    }
}

// ---------------------------------------------------------------------------
// RowStateColors
// ---------------------------------------------------------------------------

/// Semantic background tints for data-grid row states.
///
/// All values are semi-transparent so they blend with alternating row stripes.
/// `dirty` is `None` — dirty state is indicated at the cell level only.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RowStateColors {
    /// Dirty rows: `None` — use cell-level indicators instead of row background.
    pub dirty: Option<Hsla>,
    /// Row currently being saved (optimistic, transient).
    pub saving: Hsla,
    /// Row whose last save attempt failed.
    pub error: Hsla,
    /// New row pending INSERT.
    pub pending_insert: Hsla,
    /// Row marked for DELETE.
    pub pending_delete: Hsla,
}

impl RowStateColors {
    /// Row state tokens for the Ayu Dark palette.
    pub fn dark() -> Self {
        Self {
            dirty: None,
            saving: from_hex(0xFFB454, 0.10),
            error: from_hex(0xF07178, 0.15),
            pending_insert: from_hex(0xAAD94C, 0.15),
            pending_delete: from_hex(0xF07178, 0.10),
        }
    }

    /// Row state tokens for the Ayu Mirage palette.
    pub fn mirage() -> Self {
        Self {
            dirty: None,
            saving: from_hex(0xFFCC66, 0.12),
            error: from_hex(0xF28779, 0.16),
            pending_insert: from_hex(0xAAD94C, 0.16),
            pending_delete: from_hex(0xF28779, 0.12),
        }
    }

    /// Row state tokens for the Ayu Light palette.
    pub fn light() -> Self {
        Self {
            dirty: None,
            saving: from_hex(0xF2AE49, 0.14),
            error: from_hex(0xE65050, 0.14),
            pending_insert: from_hex(0x86B300, 0.14),
            pending_delete: from_hex(0xE65050, 0.12),
        }
    }

    /// Row state tokens for the Nord palette.
    pub fn nord() -> Self {
        Self {
            dirty: None,
            saving: from_hex(0xEBCB8B, 0.12),
            error: from_hex(0xBF616A, 0.16),
            pending_insert: from_hex(0xA3BE8C, 0.16),
            pending_delete: from_hex(0xBF616A, 0.12),
        }
    }

    /// Row state tokens for the Dracula palette.
    pub fn dracula() -> Self {
        Self {
            dirty: None,
            saving: from_hex(0xF1FA8C, 0.12),
            error: from_hex(0xFF5555, 0.16),
            pending_insert: from_hex(0x50FA7B, 0.16),
            pending_delete: from_hex(0xFF5555, 0.12),
        }
    }

    /// Row state tokens for the Catppuccin Latte palette.
    pub fn catppuccin_latte() -> Self {
        Self {
            dirty: None,
            saving: from_hex(0xDF8E1D, 0.14),
            error: from_hex(0xD20F39, 0.14),
            pending_insert: from_hex(0x40A02B, 0.14),
            pending_delete: from_hex(0xD20F39, 0.12),
        }
    }

    /// Row state tokens for the GitHub Light palette.
    pub fn github_light() -> Self {
        Self {
            dirty: None,
            saving: from_hex(0x9A6700, 0.14),
            error: from_hex(0xCF222E, 0.14),
            pending_insert: from_hex(0x1A7F37, 0.14),
            pending_delete: from_hex(0xCF222E, 0.12),
        }
    }

    /// Row state tokens for the One Light palette.
    pub fn one_light() -> Self {
        Self {
            dirty: None,
            saving: from_hex(0xC18401, 0.14),
            error: from_hex(0xE45649, 0.14),
            pending_insert: from_hex(0x50A14F, 0.14),
            pending_delete: from_hex(0xE45649, 0.12),
        }
    }

    /// Row state tokens for Dory Dark (VS Code Dark Modern).
    pub fn dory_dark() -> Self {
        Self {
            dirty: None,
            saving: from_hex(0xCCA700, 0.12),
            error: from_hex(0xF85149, 0.16),
            pending_insert: from_hex(0x2EA043, 0.16),
            pending_delete: from_hex(0xF85149, 0.12),
        }
    }

    /// Row state tokens for Dory Light (Zed One Light).
    pub fn dory_light() -> Self {
        Self {
            dirty: None,
            saving: from_hex(0xA48819, 0.14),
            error: from_hex(0xD36151, 0.14),
            pending_insert: from_hex(0x669F59, 0.14),
            pending_delete: from_hex(0xD36151, 0.12),
        }
    }

    /// Return the `RowStateColors` for the currently active theme.
    ///
    /// Reads `ThemeSettingGlobal` from `cx`; falls back to Dory Dark when absent.
    pub fn for_current(cx: &App) -> Self {
        match ThemeSettingGlobal::get(cx) {
            ThemeSetting::DoryDark => Self::dory_dark(),
            ThemeSetting::DoryLight => Self::dory_light(),
            ThemeSetting::Dark => Self::dark(),
            ThemeSetting::Mirage => Self::mirage(),
            ThemeSetting::Light => Self::light(),
            ThemeSetting::Nord => Self::nord(),
            ThemeSetting::Dracula => Self::dracula(),
            ThemeSetting::CatppuccinLatte => Self::catppuccin_latte(),
            ThemeSetting::GitHubLight => Self::github_light(),
            ThemeSetting::OneLight => Self::one_light(),
        }
    }
}

// ---------------------------------------------------------------------------
// ChartColors
// ---------------------------------------------------------------------------

/// Semantic colors for chart chrome: inspector overlays, axis-bar pills,
/// legend, and stats dock.
///
/// `dark()` and `mirage()` reproduce today's hardcoded canvas literals so the
/// visual output on those themes is unchanged. `light()` provides legible Ayu
/// Light equivalents.
///
/// All values are self-contained per-theme hex/hsl literals — they do NOT
/// derive from `cx.theme()` at runtime so the struct can be constructed without
/// a live render context (e.g., in unit tests and `for_current` dispatch).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChartColors {
    /// Panel / overlay background (inspector, readout overlay).
    pub panel_bg: Hsla,
    /// Panel / overlay border.
    pub panel_border: Hsla,
    /// Label text (muted descriptors, axis tick labels).
    pub label_fg: Hsla,
    /// Primary value text (high-contrast numbers and identifiers).
    pub value_fg: Hsla,
    /// Secondary / muted text (counters, de-emphasised stats).
    pub muted_fg: Hsla,
    /// Row / item hover background.
    pub hover_bg: Hsla,
    /// Pill / chip background (axis-bar column pills).
    pub pill_bg: Hsla,
    /// Pill / chip border.
    pub pill_border: Hsla,
    /// Checkbox checked fill (axis-bar toggle).
    pub checkbox_checked: Hsla,
    /// Stats accent — cyan-family highlight for the stats dock value.
    pub stats_accent: Hsla,
}

impl ChartColors {
    /// Chart chrome tokens for the Ayu Dark palette.
    ///
    /// Values reproduce the hardcoded canvas literals present before this
    /// struct was introduced, preserving visual parity on Dark.
    pub fn dark() -> Self {
        Self {
            panel_bg: hsla(0.0, 0.0, 0.08, 1.0),
            panel_border: hsla(0.0, 0.0, 1.0, 0.08),
            label_fg: hsla(0.0, 0.0, 0.55, 1.0),
            value_fg: hsla(0.0, 0.0, 0.90, 1.0),
            muted_fg: hsla(0.0, 0.0, 0.45, 1.0),
            hover_bg: hsla(0.0, 0.0, 1.0, 0.06),
            pill_bg: hsla(0.0, 0.0, 1.0, 0.06),
            pill_border: hsla(0.0, 0.0, 1.0, 0.12),
            checkbox_checked: hsla(0.55, 0.7, 0.5, 1.0),
            stats_accent: from_hex(0x95E6CB, 1.0),
        }
    }

    /// Chart chrome tokens for the Ayu Mirage palette.
    ///
    /// Values are the Mirage equivalents of the Dark literals, preserving
    /// visual parity on Mirage.
    pub fn mirage() -> Self {
        Self {
            panel_bg: hsla(0.0, 0.0, 0.10, 1.0),
            panel_border: hsla(0.0, 0.0, 1.0, 0.08),
            label_fg: hsla(0.0, 0.0, 0.58, 1.0),
            value_fg: hsla(0.0, 0.0, 0.92, 1.0),
            muted_fg: hsla(0.0, 0.0, 0.48, 1.0),
            hover_bg: hsla(0.0, 0.0, 1.0, 0.06),
            pill_bg: hsla(0.0, 0.0, 1.0, 0.06),
            pill_border: hsla(0.0, 0.0, 1.0, 0.12),
            checkbox_checked: hsla(0.55, 0.7, 0.5, 1.0),
            stats_accent: from_hex(0x95E6CB, 1.0),
        }
    }

    /// Chart chrome tokens for the Ayu Light palette.
    ///
    /// Values are hand-picked Ayu Light equivalents ensuring legibility on a
    /// light background. `stats_accent` uses Ayu Light's cyan (#4CBF99) and
    /// `checkbox_checked` uses the Light info/chart-1 blue (#399EE6).
    pub fn light() -> Self {
        Self {
            panel_bg: from_hex(0xF7F8FA, 1.0),
            panel_border: from_hex(0xD9DEE8, 1.0),
            label_fg: from_hex(0x787E85, 1.0),
            value_fg: from_hex(0x5C6166, 1.0),
            muted_fg: from_hex(0xABB0B6, 1.0),
            hover_bg: from_hex(0x5C6166, 0.06),
            pill_bg: from_hex(0x5C6166, 0.06),
            pill_border: from_hex(0xD9DEE8, 1.0),
            checkbox_checked: from_hex(0x399EE6, 1.0),
            stats_accent: from_hex(0x4CBF99, 1.0),
        }
    }

    /// Chart chrome tokens for the Nord palette.
    pub fn nord() -> Self {
        Self {
            panel_bg: from_hex(0x3B4252, 1.0),
            panel_border: from_hex(0x434C5E, 1.0),
            label_fg: from_hex(0x8A94A8, 1.0),
            value_fg: from_hex(0xD8DEE9, 1.0),
            muted_fg: from_hex(0x7A8498, 1.0),
            hover_bg: from_hex(0xD8DEE9, 0.06),
            pill_bg: from_hex(0xD8DEE9, 0.06),
            pill_border: from_hex(0xD8DEE9, 0.12),
            checkbox_checked: from_hex(0x88C0D0, 1.0),
            stats_accent: from_hex(0x8FBCBB, 1.0),
        }
    }

    /// Chart chrome tokens for the Dracula palette.
    pub fn dracula() -> Self {
        Self {
            panel_bg: from_hex(0x343746, 1.0),
            panel_border: from_hex(0x44475A, 1.0),
            label_fg: from_hex(0x9093A6, 1.0),
            value_fg: from_hex(0xF8F8F2, 1.0),
            muted_fg: from_hex(0x7A7D8F, 1.0),
            hover_bg: from_hex(0xF8F8F2, 0.06),
            pill_bg: from_hex(0xF8F8F2, 0.06),
            pill_border: from_hex(0xF8F8F2, 0.12),
            checkbox_checked: from_hex(0xBD93F9, 1.0),
            stats_accent: from_hex(0x8BE9FD, 1.0),
        }
    }

    /// Chart chrome tokens for the Catppuccin Latte palette.
    pub fn catppuccin_latte() -> Self {
        Self {
            panel_bg: from_hex(0xE6E9EF, 1.0),
            panel_border: from_hex(0xDCE0E8, 1.0),
            label_fg: from_hex(0x8C8FA1, 1.0),
            value_fg: from_hex(0x4C4F69, 1.0),
            muted_fg: from_hex(0xB1B4C4, 1.0),
            hover_bg: from_hex(0x4C4F69, 0.06),
            pill_bg: from_hex(0x4C4F69, 0.06),
            pill_border: from_hex(0xDCE0E8, 1.0),
            checkbox_checked: from_hex(0x1E66F5, 1.0),
            stats_accent: from_hex(0x04A5E5, 1.0),
        }
    }

    /// Chart chrome tokens for the GitHub Light palette.
    pub fn github_light() -> Self {
        Self {
            panel_bg: from_hex(0xF6F8FA, 1.0),
            panel_border: from_hex(0xD0D7DE, 1.0),
            label_fg: from_hex(0x6E7781, 1.0),
            value_fg: from_hex(0x24292F, 1.0),
            muted_fg: from_hex(0x9AA0A8, 1.0),
            hover_bg: from_hex(0x24292F, 0.06),
            pill_bg: from_hex(0x24292F, 0.06),
            pill_border: from_hex(0xD0D7DE, 1.0),
            checkbox_checked: from_hex(0x0969DA, 1.0),
            stats_accent: from_hex(0x1B7C83, 1.0),
        }
    }

    /// Chart chrome tokens for the One Light palette.
    pub fn one_light() -> Self {
        Self {
            panel_bg: from_hex(0xF0F0F1, 1.0),
            panel_border: from_hex(0xE5E5E6, 1.0),
            label_fg: from_hex(0xA0A1A7, 1.0),
            value_fg: from_hex(0x383A42, 1.0),
            muted_fg: from_hex(0xB8B9BE, 1.0),
            hover_bg: from_hex(0x383A42, 0.06),
            pill_bg: from_hex(0x383A42, 0.06),
            pill_border: from_hex(0xE5E5E6, 1.0),
            checkbox_checked: from_hex(0x4078F2, 1.0),
            stats_accent: from_hex(0x0184BC, 1.0),
        }
    }

    /// Chart chrome tokens for Dory Dark (VS Code Dark Modern).
    pub fn dory_dark() -> Self {
        Self {
            panel_bg: from_hex(0x181818, 1.0),
            panel_border: from_hex(0x2B2B2B, 1.0),
            label_fg: from_hex(0x9D9D9D, 1.0),
            value_fg: from_hex(0xCCCCCC, 1.0),
            muted_fg: from_hex(0x868686, 1.0),
            hover_bg: from_hex(0xFFFFFF, 0.06),
            pill_bg: from_hex(0xFFFFFF, 0.06),
            pill_border: from_hex(0xFFFFFF, 0.12),
            checkbox_checked: from_hex(0x0078D4, 1.0),
            stats_accent: from_hex(0x4EC9B0, 1.0),
        }
    }

    /// Chart chrome tokens for Dory Light (Zed One Light).
    pub fn dory_light() -> Self {
        Self {
            panel_bg: from_hex(0xEBEBEC, 1.0),
            panel_border: from_hex(0xC9C9CA, 1.0),
            label_fg: from_hex(0x58585A, 1.0),
            value_fg: from_hex(0x242529, 1.0),
            muted_fg: from_hex(0x7A7A7C, 1.0),
            hover_bg: from_hex(0x242529, 0.06),
            pill_bg: from_hex(0x242529, 0.06),
            pill_border: from_hex(0xC9C9CA, 1.0),
            checkbox_checked: from_hex(0x5C78E2, 1.0),
            stats_accent: from_hex(0x3B7EA8, 1.0),
        }
    }

    /// Return the `ChartColors` for the currently active theme.
    ///
    /// Reads `ThemeSettingGlobal` from `cx`; falls back to Dory Dark when absent.
    pub fn for_current(cx: &App) -> Self {
        match ThemeSettingGlobal::get(cx) {
            ThemeSetting::DoryDark => Self::dory_dark(),
            ThemeSetting::DoryLight => Self::dory_light(),
            ThemeSetting::Dark => Self::dark(),
            ThemeSetting::Mirage => Self::mirage(),
            ThemeSetting::Light => Self::light(),
            ThemeSetting::Nord => Self::nord(),
            ThemeSetting::Dracula => Self::dracula(),
            ThemeSetting::CatppuccinLatte => Self::catppuccin_latte(),
            ThemeSetting::GitHubLight => Self::github_light(),
            ThemeSetting::OneLight => Self::one_light(),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use dory_core::ThemeSetting;
    use gpui::TestAppContext;

    #[gpui::test]
    fn theme_setting_global_falls_back_to_dory_dark_when_absent(cx: &mut TestAppContext) {
        cx.update(|cx| {
            assert_eq!(ThemeSettingGlobal::get(cx), ThemeSetting::DoryDark);
        });
    }

    #[gpui::test]
    fn theme_setting_global_roundtrips_all_variants(cx: &mut TestAppContext) {
        cx.update(|cx| {
            ThemeSettingGlobal::set(cx, ThemeSetting::Mirage);
            assert_eq!(ThemeSettingGlobal::get(cx), ThemeSetting::Mirage);

            ThemeSettingGlobal::set(cx, ThemeSetting::Light);
            assert_eq!(ThemeSettingGlobal::get(cx), ThemeSetting::Light);

            ThemeSettingGlobal::set(cx, ThemeSetting::Dark);
            assert_eq!(ThemeSettingGlobal::get(cx), ThemeSetting::Dark);

            ThemeSettingGlobal::set(cx, ThemeSetting::DoryDark);
            assert_eq!(ThemeSettingGlobal::get(cx), ThemeSetting::DoryDark);

            ThemeSettingGlobal::set(cx, ThemeSetting::DoryLight);
            assert_eq!(ThemeSettingGlobal::get(cx), ThemeSetting::DoryLight);
        });
    }

    /// `for_current` returns the former `tokens::BannerColors` fixed values
    /// for info/success/error across all themes, and derives warning from
    /// `theme.primary`. The per-palette constructors (`dark`, `mirage`, `light`)
    /// are distinct and carry per-theme semantic values for future use.
    #[gpui::test]
    fn banner_colors_for_current_returns_legacy_pixel_exact_values(cx: &mut TestAppContext) {
        // gpui_component::init registers the Theme global required by cx.theme().
        cx.update(gpui_component::init);
        cx.update(|cx| {
            // info/success/error are theme-agnostic (same across all themes).
            ThemeSettingGlobal::set(cx, ThemeSetting::Dark);
            let colors_dark = BannerColors::for_current(cx);
            ThemeSettingGlobal::set(cx, ThemeSetting::Mirage);
            let colors_mirage = BannerColors::for_current(cx);
            ThemeSettingGlobal::set(cx, ThemeSetting::Light);
            let colors_light = BannerColors::for_current(cx);

            // Fixed hex values taken from former tokens::BannerColors.
            // info_bg = #1E3A5F at full opacity.
            assert_eq!(colors_dark.info_bg, colors_mirage.info_bg);
            assert_eq!(colors_dark.info_bg, colors_light.info_bg);
            assert_eq!(colors_dark.info_fg, colors_mirage.info_fg);

            // success_bg = #14532D at full opacity.
            assert_eq!(colors_dark.success_bg, colors_mirage.success_bg);
            assert_eq!(colors_dark.success_fg, colors_mirage.success_fg);

            // error_bg = #7F1D1D at full opacity.
            assert_eq!(colors_dark.error_bg, colors_mirage.error_bg);
            assert_eq!(colors_dark.error_fg, colors_mirage.error_fg);

            // All fg colors must be fully opaque.
            assert_eq!(colors_dark.info_fg.a, 1.0);
            assert_eq!(colors_dark.success_fg.a, 1.0);
            assert_eq!(colors_dark.error_fg.a, 1.0);
        });
    }

    #[gpui::test]
    fn banner_colors_for_current_warning_derives_from_theme_primary(cx: &mut TestAppContext) {
        // gpui_component::init registers the Theme global required by cx.theme().
        cx.update(gpui_component::init);
        cx.update(|cx| {
            // warning_bg = theme.primary @ 0.20, warning_fg = theme.primary @ 1.0.
            let colors = BannerColors::for_current(cx);
            assert!((colors.warning_bg.a - 0.20).abs() < 0.001);
            assert!((colors.warning_fg.a - 1.0).abs() < 0.001);
        });
    }

    #[gpui::test]
    fn row_state_colors_dirty_is_none_in_all_themes(cx: &mut TestAppContext) {
        cx.update(|cx| {
            assert!(RowStateColors::dark().dirty.is_none());
            assert!(RowStateColors::mirage().dirty.is_none());
            assert!(RowStateColors::light().dirty.is_none());

            // for_current also respects fallback
            assert!(RowStateColors::for_current(cx).dirty.is_none());
        });
    }

    #[gpui::test]
    fn row_state_colors_for_current_dispatches_to_correct_theme(cx: &mut TestAppContext) {
        cx.update(|cx| {
            ThemeSettingGlobal::set(cx, ThemeSetting::Mirage);
            let mirage = RowStateColors::for_current(cx);
            assert_eq!(mirage.saving.a, 0.12);

            ThemeSettingGlobal::set(cx, ThemeSetting::Light);
            let light = RowStateColors::for_current(cx);
            assert!(light.pending_insert.a > 0.0);
        });
    }

    #[test]
    fn banner_colors_info_fg_is_fully_opaque_in_dark_theme() {
        assert_eq!(BannerColors::dark().info_fg.a, 1.0);
        assert_eq!(BannerColors::mirage().info_fg.a, 1.0);
        assert_eq!(BannerColors::light().info_fg.a, 1.0);
    }

    /// All 10 ChartColors fields must be populated (non-zero alpha) for Dark.
    #[test]
    fn chart_colors_dark_all_fields_populated() {
        let c = ChartColors::dark();
        assert!(c.panel_bg.a > 0.0);
        assert!(c.panel_border.a > 0.0);
        assert!(c.label_fg.a > 0.0);
        assert!(c.value_fg.a > 0.0);
        assert!(c.muted_fg.a > 0.0);
        assert!(c.hover_bg.a > 0.0);
        assert!(c.pill_bg.a > 0.0);
        assert!(c.pill_border.a > 0.0);
        assert!(c.checkbox_checked.a > 0.0);
        assert!(c.stats_accent.a > 0.0);
    }

    /// All 10 ChartColors fields must be populated (non-zero alpha) for Mirage.
    #[test]
    fn chart_colors_mirage_all_fields_populated() {
        let c = ChartColors::mirage();
        assert!(c.panel_bg.a > 0.0);
        assert!(c.panel_border.a > 0.0);
        assert!(c.label_fg.a > 0.0);
        assert!(c.value_fg.a > 0.0);
        assert!(c.muted_fg.a > 0.0);
        assert!(c.hover_bg.a > 0.0);
        assert!(c.pill_bg.a > 0.0);
        assert!(c.pill_border.a > 0.0);
        assert!(c.checkbox_checked.a > 0.0);
        assert!(c.stats_accent.a > 0.0);
    }

    /// All 10 ChartColors fields must be populated (non-zero alpha) for Light.
    #[test]
    fn chart_colors_light_all_fields_populated() {
        let c = ChartColors::light();
        assert!(c.panel_bg.a > 0.0);
        assert!(c.panel_border.a > 0.0);
        assert!(c.label_fg.a > 0.0);
        assert!(c.value_fg.a > 0.0);
        assert!(c.muted_fg.a > 0.0);
        assert!(c.hover_bg.a > 0.0);
        assert!(c.pill_bg.a > 0.0);
        assert!(c.pill_border.a > 0.0);
        assert!(c.checkbox_checked.a > 0.0);
        assert!(c.stats_accent.a > 0.0);
    }

    /// `for_current` must dispatch to the matching constructor for each theme.
    #[gpui::test]
    fn chart_colors_for_current_dispatches_to_correct_variant(cx: &mut TestAppContext) {
        cx.update(|cx| {
            ThemeSettingGlobal::set(cx, ThemeSetting::Dark);
            assert_eq!(ChartColors::for_current(cx), ChartColors::dark());

            ThemeSettingGlobal::set(cx, ThemeSetting::Mirage);
            assert_eq!(ChartColors::for_current(cx), ChartColors::mirage());

            ThemeSettingGlobal::set(cx, ThemeSetting::Light);
            assert_eq!(ChartColors::for_current(cx), ChartColors::light());

            ThemeSettingGlobal::set(cx, ThemeSetting::Nord);
            assert_eq!(ChartColors::for_current(cx), ChartColors::nord());

            ThemeSettingGlobal::set(cx, ThemeSetting::Dracula);
            assert_eq!(ChartColors::for_current(cx), ChartColors::dracula());

            ThemeSettingGlobal::set(cx, ThemeSetting::CatppuccinLatte);
            assert_eq!(
                ChartColors::for_current(cx),
                ChartColors::catppuccin_latte()
            );

            ThemeSettingGlobal::set(cx, ThemeSetting::GitHubLight);
            assert_eq!(ChartColors::for_current(cx), ChartColors::github_light());

            ThemeSettingGlobal::set(cx, ThemeSetting::OneLight);
            assert_eq!(ChartColors::for_current(cx), ChartColors::one_light());

            ThemeSettingGlobal::set(cx, ThemeSetting::DoryDark);
            assert_eq!(ChartColors::for_current(cx), ChartColors::dory_dark());

            ThemeSettingGlobal::set(cx, ThemeSetting::DoryLight);
            assert_eq!(ChartColors::for_current(cx), ChartColors::dory_light());
        });
    }
}
