pub use crate::typography::AppFonts;
use dory_core::{AppStyle, GeneralSettings, ThemeSetting};
use gpui::{App, Hsla, SharedString, Window, WindowAppearance, hsla, px};
use gpui_component::{
    highlighter::HighlightTheme,
    theme::{Theme, ThemeMode},
};
use std::{rc::Rc, sync::Arc};

/// Ghost border: `#524436` at 15% opacity. Felt-not-seen structural separator.
/// Use instead of solid `theme.border` when separating major UI regions.
pub fn ghost_border_color() -> Hsla {
    crate::tokens::ChromeColors::ghost_border()
}

pub fn init(cx: &mut App) {
    gpui_component::init(cx);
    crate::controls::register_input_overrides(cx);
    apply_theme(ThemeSetting::DoryDark, AppStyle::Default, None, cx);
}

/// Initialize the theme and density global from persisted settings.
///
/// Call this after `init` and after the config has been loaded, before
/// the first window opens. This sets up the correct radius tokens and
/// density global for the first frame.
pub fn init_with_settings(
    setting: ThemeSetting,
    style: AppStyle,
    font_setting: dory_core::FontSetting,
    cx: &mut App,
) {
    crate::density::init(cx, style);
    crate::semantic::ThemeSettingGlobal::set(cx, setting);
    crate::semantic::FontSettingGlobal::set(cx, font_setting);
    apply_theme(setting, style, None, cx);
}

/// Update only the UI font on the live theme. Prefer this over [`apply_theme`]
/// when the palette has not changed — rebuilding every token and the
/// highlight theme while a large family (for example a Nerd Font) is loading
/// stalls the UI thread.
pub fn apply_ui_font(cx: &mut App) {
    let ui_family = AppFonts::current_ui_family(cx);
    let theme = Theme::global_mut(cx);
    persist_font_config(theme, ui_family);
    cx.refresh_windows();
}

pub fn appearance_is_dark(appearance: WindowAppearance) -> bool {
    matches!(
        appearance,
        WindowAppearance::Dark | WindowAppearance::VibrantDark
    )
}

/// Apply the palette implied by theme mode + OS appearance.
pub fn apply_resolved(
    settings: &GeneralSettings,
    appearance_is_dark: bool,
    window: Option<&mut Window>,
    cx: &mut App,
) {
    crate::semantic::FontSettingGlobal::set(cx, settings.ui_font.clone());
    let setting = settings.resolved_theme(appearance_is_dark);
    crate::semantic::ThemeSettingGlobal::set(cx, setting);
    apply_theme(setting, settings.style, window, cx);
}

pub fn apply_theme(
    setting: ThemeSetting,
    style: AppStyle,
    window: Option<&mut Window>,
    cx: &mut App,
) {
    match setting {
        ThemeSetting::DoryDark => {
            Theme::change(ThemeMode::Dark, window, cx);
            apply_dory_dark(style, cx);
        }
        ThemeSetting::DoryLight => {
            Theme::change(ThemeMode::Light, window, cx);
            apply_dory_light(style, cx);
        }
        ThemeSetting::Dark => {
            Theme::change(ThemeMode::Dark, window, cx);
            apply_ayu_dark(style, cx);
        }
        ThemeSetting::Mirage => {
            Theme::change(ThemeMode::Dark, window, cx);
            apply_ayu_mirage(style, cx);
        }
        ThemeSetting::Light => {
            Theme::change(ThemeMode::Light, window, cx);
            apply_ayu_light(style, cx);
        }
        ThemeSetting::Nord => {
            Theme::change(ThemeMode::Dark, window, cx);
            apply_nord(style, cx);
        }
        ThemeSetting::Dracula => {
            Theme::change(ThemeMode::Dark, window, cx);
            apply_dracula(style, cx);
        }
        ThemeSetting::CatppuccinLatte => {
            Theme::change(ThemeMode::Light, window, cx);
            apply_catppuccin_latte(style, cx);
        }
        ThemeSetting::GitHubLight => {
            Theme::change(ThemeMode::Light, window, cx);
            apply_github_light(style, cx);
        }
        ThemeSetting::OneLight => {
            Theme::change(ThemeMode::Light, window, cx);
            apply_one_light(style, cx);
        }
    }
    // `Theme::change` only refreshes the window it was given, and Settings
    // applies themes with `None`. Dirty every window so the workspace
    // (and any other open window) paints the new tokens immediately.
    cx.refresh_windows();
}

fn rgb_to_hsla(hex: u32) -> Hsla {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() < f32::EPSILON {
        return hsla(0.0, 0.0, l, 1.0);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if (max - r).abs() < f32::EPSILON {
        let mut h = (g - b) / d;
        if g < b {
            h += 6.0;
        }
        h
    } else if (max - g).abs() < f32::EPSILON {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };

    hsla(h / 6.0, s, l, 1.0)
}

fn rgb_to_hsla_alpha(hex: u32, alpha: f32) -> Hsla {
    let mut hsla = rgb_to_hsla(hex);
    hsla.a = alpha;
    hsla
}

/// Persist custom font families into the stored ThemeConfig so that
/// `Theme::change()` (triggered by ThemeRegistry observer) preserves them.
/// Without this, `apply_config()` resets font_family to ".SystemUIFont".
///
/// The non-mono family follows the active `FontSetting`; the mono family is
/// always the platform system monospace font.
fn persist_font_config(theme: &mut Theme, ui_family: &'static str) {
    let mut dark = (*theme.dark_theme).clone();
    dark.font_family = Some(SharedString::from(ui_family));
    dark.mono_font_family = Some(SharedString::from(AppFonts::MONO));
    theme.dark_theme = Rc::new(dark);

    let mut light = (*theme.light_theme).clone();
    light.font_family = Some(SharedString::from(ui_family));
    light.mono_font_family = Some(SharedString::from(AppFonts::MONO));
    theme.light_theme = Rc::new(light);

    // Also set the immediate values
    theme.font_family = SharedString::from(ui_family);
    theme.mono_font_family = SharedString::from(AppFonts::MONO);
}

/// Apply border-radius values to the theme based on the active `AppStyle`.
///
/// - `AppStyle::Default` — square corners (0 px), the project's flat chrome
///   baseline.
/// - `AppStyle::Compact` — subtle radii: 2 px (`radius`) and 3 px (`radius_lg`),
///   matching the Design System token values.
fn apply_style_radius(theme: &mut Theme, style: AppStyle) {
    match style {
        AppStyle::Default => {
            theme.radius = px(0.0);
            theme.radius_lg = px(0.0);
        }
        AppStyle::Compact => {
            theme.radius = px(2.0);
            theme.radius_lg = px(3.0);
        }
    }
}

fn apply_editor_chrome(
    theme: &mut Theme,
    background: Hsla,
    active_line: Hsla,
    line_number: Hsla,
    active_line_number: Hsla,
) {
    let mut highlight_theme = (*theme.highlight_theme).clone();
    highlight_theme.style.editor_background = Some(background);
    highlight_theme.style.editor_active_line = Some(active_line);
    highlight_theme.style.editor_line_number = Some(line_number);
    highlight_theme.style.editor_active_line_number = Some(active_line_number);
    theme.highlight_theme = Arc::new(HighlightTheme {
        name: highlight_theme.name.clone(),
        appearance: highlight_theme.appearance,
        style: highlight_theme.style,
    });
}

fn apply_ayu_dark(style: AppStyle, cx: &mut App) {
    let ui_family = AppFonts::current_ui_family(cx);
    let theme = Theme::global_mut(cx);

    // Ayu Dark base colors
    let background = rgb_to_hsla(0x0A0E14);
    let panel = rgb_to_hsla(0x0F1419);
    let foreground = rgb_to_hsla(0xB3B1AD);
    let muted = rgb_to_hsla(0x5C6773);
    let accent = rgb_to_hsla(0xFFB454);
    let border = rgb_to_hsla(0x1F2430);

    let raised = rgb_to_hsla(0x151E2B);
    let selection = rgb_to_hsla(0x273747);

    let error = rgb_to_hsla(0xF07178);
    let success = rgb_to_hsla(0xAAD94C);
    let warning = rgb_to_hsla(0xFFB454);
    let info = rgb_to_hsla(0x59C2FF);

    persist_font_config(theme, ui_family);
    apply_style_radius(theme, style);

    // Core colors
    theme.background = background;
    theme.foreground = foreground;
    theme.border = border;
    theme.caret = accent;

    // Muted
    theme.muted = muted;
    theme.muted_foreground = muted;

    // Primary (accent color)
    theme.primary = accent;
    theme.primary_hover = rgb_to_hsla(0xE6A34C);
    theme.primary_active = rgb_to_hsla(0xCC9143);
    theme.primary_foreground = rgb_to_hsla(0x0A0E14);

    // Secondary
    theme.secondary = raised;
    theme.secondary_hover = rgb_to_hsla(0x1A2535);
    theme.secondary_active = rgb_to_hsla(0x1F2A3F);
    theme.secondary_foreground = foreground;

    // Accent (hover states)
    theme.accent = rgb_to_hsla_alpha(0xB3B1AD, 0.05);
    theme.accent_foreground = foreground;

    // Semantic colors - Danger
    theme.danger = error;
    theme.danger_hover = rgb_to_hsla(0xD8656B);
    theme.danger_active = rgb_to_hsla(0xC05A5E);
    // White foreground on danger red — higher contrast than dark 0x0A0E14
    theme.danger_foreground = rgb_to_hsla(0xFFFFFF);

    // Semantic colors - Success
    theme.success = success;
    theme.success_hover = rgb_to_hsla(0x99C444);
    theme.success_active = rgb_to_hsla(0x88AF3D);
    theme.success_foreground = rgb_to_hsla(0x0A0E14);

    // Semantic colors - Warning
    theme.warning = warning;
    theme.warning_hover = rgb_to_hsla(0xE6A34C);
    theme.warning_active = rgb_to_hsla(0xCC9143);
    theme.warning_foreground = rgb_to_hsla(0x0A0E14);

    // Semantic colors - Info
    theme.info = info;
    theme.info_hover = rgb_to_hsla(0x50AFE6);
    theme.info_active = rgb_to_hsla(0x479ACC);
    theme.info_foreground = rgb_to_hsla(0x0A0E14);

    // Popover / modal surface — match the shared raised chrome treatment.
    theme.popover = raised;
    theme.popover_foreground = foreground;

    // Selection
    theme.selection = selection;

    // Focus ring
    theme.ring = rgb_to_hsla_alpha(0xFFB454, 0.75);

    // Input — alpha increased from 0.10 to 0.14 for better legibility on dark bg
    theme.input = rgb_to_hsla_alpha(0xB3B1AD, 0.14);

    // Scrollbar
    theme.scrollbar = background;
    theme.scrollbar_thumb = rgb_to_hsla_alpha(0xB3B1AD, 0.15);
    theme.scrollbar_thumb_hover = rgb_to_hsla_alpha(0xB3B1AD, 0.25);

    // Sidebar tracks the primary workspace surface so nav and content stay visually aligned.
    theme.sidebar = background;
    theme.sidebar_foreground = foreground;
    theme.sidebar_border = border;
    theme.sidebar_accent = rgb_to_hsla_alpha(0xB3B1AD, 0.05);
    theme.sidebar_accent_foreground = foreground;
    theme.sidebar_primary = accent;
    theme.sidebar_primary_foreground = rgb_to_hsla(0x0A0E14);

    // Tab bar
    theme.tab = panel;
    theme.tab_bar = panel;
    theme.tab_foreground = muted;
    theme.tab_active = background;
    theme.tab_active_foreground = foreground;
    theme.tab_bar_segmented = raised;

    // Table
    theme.table = background;
    theme.table_head = panel;
    theme.table_head_foreground = muted;
    theme.table_even = rgb_to_hsla_alpha(0xB3B1AD, 0.02);
    theme.table_hover = rgb_to_hsla_alpha(0xB3B1AD, 0.05);
    theme.table_active = rgb_to_hsla_alpha(0x59C2FF, 0.15);
    theme.table_active_border = rgb_to_hsla_alpha(0x59C2FF, 0.5);
    // No row dividers — alternating tint (table_even) provides visual separation
    theme.table_row_border = hsla(0.0, 0.0, 0.0, 0.0);

    // List
    theme.list = background;
    theme.list_head = panel;
    theme.list_even = rgb_to_hsla_alpha(0xB3B1AD, 0.02);
    theme.list_hover = rgb_to_hsla_alpha(0xB3B1AD, 0.05);
    theme.list_active = selection;
    theme.list_active_border = accent;

    // Accordion
    theme.accordion = panel;
    theme.accordion_hover = raised;

    // Title bar
    theme.title_bar = panel;
    theme.title_bar_border = border;

    // Tiles
    theme.tiles = rgb_to_hsla(0x111823);

    // Overlay
    theme.overlay = rgb_to_hsla_alpha(0x000000, 0.55);

    // Window border (Linux only)
    theme.window_border = border;

    // Link
    theme.link = info;
    theme.link_hover = rgb_to_hsla(0x6BCFFF);
    theme.link_active = rgb_to_hsla(0x50AFE6);

    // Switch
    theme.switch = muted;
    theme.switch_thumb = foreground;

    // Slider
    theme.slider_bar = muted;
    theme.slider_thumb = accent;

    // Progress bar
    theme.progress_bar = accent;

    // Skeleton
    theme.skeleton = raised;

    // Description list
    theme.description_list_label = panel;
    theme.description_list_label_foreground = muted;

    // Drag and drop
    theme.drag_border = accent;
    theme.drop_target = rgb_to_hsla_alpha(0xFFB454, 0.1);

    // Group box
    theme.group_box = panel;
    theme.group_box_foreground = foreground;

    // Chart colors
    theme.chart_1 = rgb_to_hsla(0x59C2FF);
    theme.chart_2 = rgb_to_hsla(0xAAD94C);
    theme.chart_3 = rgb_to_hsla(0xFFB454);
    theme.chart_4 = rgb_to_hsla(0xF07178);
    theme.chart_5 = rgb_to_hsla(0xD2A6FF);

    // Candlestick
    theme.bullish = success;
    theme.bearish = error;

    // Base colors
    theme.red = error;
    theme.red_light = rgb_to_hsla(0xF8A5AA);
    theme.green = success;
    theme.green_light = rgb_to_hsla(0xC5E88B);
    theme.blue = info;
    theme.blue_light = rgb_to_hsla(0x8DD6FF);
    theme.yellow = warning;
    theme.yellow_light = rgb_to_hsla(0xFFCC80);
    theme.magenta = rgb_to_hsla(0xD2A6FF);
    theme.magenta_light = rgb_to_hsla(0xE4CCFF);
    theme.cyan = rgb_to_hsla(0x95E6CB);
    theme.cyan_light = rgb_to_hsla(0xBBF0DF);
}

fn apply_ayu_mirage(style: AppStyle, cx: &mut App) {
    let ui_family = AppFonts::current_ui_family(cx);
    let theme = Theme::global_mut(cx);

    let background = rgb_to_hsla(0x1F2430);
    let panel = rgb_to_hsla(0x232834);
    let foreground = rgb_to_hsla(0xCBCCC6);
    let muted = rgb_to_hsla(0x707A8C);
    let accent = rgb_to_hsla(0xFFCC66);
    let border = rgb_to_hsla(0x3A4052);

    let raised = rgb_to_hsla(0x242936);
    let selection = rgb_to_hsla(0x33415E);

    let error = rgb_to_hsla(0xF28779);
    let success = rgb_to_hsla(0xAAD94C);
    let warning = rgb_to_hsla(0xFFCC66);
    let info = rgb_to_hsla(0x73D0FF);

    persist_font_config(theme, ui_family);
    apply_style_radius(theme, style);
    apply_editor_chrome(theme, background, raised, muted, foreground);

    theme.background = background;
    theme.foreground = foreground;
    theme.border = border;
    theme.caret = accent;

    theme.muted = muted;
    theme.muted_foreground = muted;

    theme.primary = accent;
    theme.primary_hover = rgb_to_hsla(0xE6B85C);
    theme.primary_active = rgb_to_hsla(0xCCA352);
    theme.primary_foreground = background;

    theme.secondary = panel;
    theme.secondary_hover = rgb_to_hsla(0x2A3040);
    theme.secondary_active = rgb_to_hsla(0x31394C);
    theme.secondary_foreground = foreground;

    theme.accent = rgb_to_hsla_alpha(0xCBCCC6, 0.06);
    theme.accent_foreground = foreground;

    theme.danger = error;
    theme.danger_hover = rgb_to_hsla(0xDB7A6D);
    theme.danger_active = rgb_to_hsla(0xC56D61);
    // White foreground on danger red — higher contrast than dark background
    theme.danger_foreground = rgb_to_hsla(0xFFFFFF);

    theme.success = success;
    theme.success_hover = rgb_to_hsla(0x99C444);
    theme.success_active = rgb_to_hsla(0x88AF3D);
    theme.success_foreground = background;

    theme.warning = warning;
    theme.warning_hover = rgb_to_hsla(0xE6B85C);
    theme.warning_active = rgb_to_hsla(0xCCA352);
    theme.warning_foreground = background;

    theme.info = info;
    theme.info_hover = rgb_to_hsla(0x68BBE6);
    theme.info_active = rgb_to_hsla(0x5CA6CC);
    theme.info_foreground = background;

    theme.popover = raised;
    theme.popover_foreground = foreground;

    theme.selection = selection;
    theme.ring = rgb_to_hsla_alpha(0xFFCC66, 0.72);
    theme.input = rgb_to_hsla_alpha(0xCBCCC6, 0.09);

    theme.scrollbar = background;
    theme.scrollbar_thumb = rgb_to_hsla_alpha(0xCBCCC6, 0.14);
    theme.scrollbar_thumb_hover = rgb_to_hsla_alpha(0xCBCCC6, 0.22);

    theme.sidebar = background;
    theme.sidebar_foreground = foreground;
    theme.sidebar_border = border;
    theme.sidebar_accent = rgb_to_hsla_alpha(0xCBCCC6, 0.05);
    theme.sidebar_accent_foreground = foreground;
    theme.sidebar_primary = accent;
    theme.sidebar_primary_foreground = background;

    theme.tab = panel;
    theme.tab_bar = panel;
    theme.tab_foreground = muted;
    theme.tab_active = background;
    theme.tab_active_foreground = foreground;
    theme.tab_bar_segmented = raised;

    theme.table = background;
    theme.table_head = panel;
    theme.table_head_foreground = muted;
    theme.table_even = rgb_to_hsla_alpha(0xCBCCC6, 0.02);
    theme.table_hover = rgb_to_hsla_alpha(0xCBCCC6, 0.05);
    theme.table_active = rgb_to_hsla_alpha(0x73D0FF, 0.12);
    theme.table_active_border = rgb_to_hsla_alpha(0x73D0FF, 0.4);
    theme.table_row_border = hsla(0.0, 0.0, 0.0, 0.0);

    theme.list = background;
    theme.list_head = panel;
    theme.list_even = rgb_to_hsla_alpha(0xCBCCC6, 0.02);
    theme.list_hover = rgb_to_hsla_alpha(0xCBCCC6, 0.05);
    theme.list_active = selection;
    theme.list_active_border = accent;

    theme.accordion = panel;
    theme.accordion_hover = raised;

    theme.title_bar = panel;
    theme.title_bar_border = border;

    theme.tiles = rgb_to_hsla(0x202734);
    theme.overlay = rgb_to_hsla_alpha(0x000000, 0.45);
    theme.window_border = border;

    theme.link = info;
    theme.link_hover = rgb_to_hsla(0x8BD8FF);
    theme.link_active = rgb_to_hsla(0x68BBE6);

    theme.switch = muted;
    theme.switch_thumb = foreground;

    theme.slider_bar = muted;
    theme.slider_thumb = accent;

    theme.progress_bar = accent;
    theme.skeleton = raised;

    theme.description_list_label = panel;
    theme.description_list_label_foreground = muted;

    theme.drag_border = accent;
    theme.drop_target = rgb_to_hsla_alpha(0xFFCC66, 0.1);

    theme.group_box = panel;
    theme.group_box_foreground = foreground;

    theme.chart_1 = info;
    theme.chart_2 = success;
    theme.chart_3 = warning;
    theme.chart_4 = error;
    theme.chart_5 = rgb_to_hsla(0xD4BFFF);

    theme.bullish = success;
    theme.bearish = error;

    theme.red = error;
    theme.red_light = rgb_to_hsla(0xF7B3AA);
    theme.green = success;
    theme.green_light = rgb_to_hsla(0xC5E88B);
    theme.blue = info;
    theme.blue_light = rgb_to_hsla(0xA6DDFF);
    theme.yellow = warning;
    theme.yellow_light = rgb_to_hsla(0xFFE099);
    theme.magenta = rgb_to_hsla(0xD4BFFF);
    theme.magenta_light = rgb_to_hsla(0xE6D9FF);
    theme.cyan = rgb_to_hsla(0x95E6CB);
    theme.cyan_light = rgb_to_hsla(0xBBF0DF);
}

fn apply_ayu_light(style: AppStyle, cx: &mut App) {
    let ui_family = AppFonts::current_ui_family(cx);
    let theme = Theme::global_mut(cx);

    let background = rgb_to_hsla(0xFAFAFA);
    let panel = rgb_to_hsla(0xF3F3F3);
    let foreground = rgb_to_hsla(0x5C6166);
    let muted = rgb_to_hsla(0xABB0B6);
    let accent = rgb_to_hsla(0xFF9940);
    let border = rgb_to_hsla(0xD9DEE8);

    let raised = rgb_to_hsla(0xF7F8FA);
    let selection = rgb_to_hsla(0xD3E8F8);

    let error = rgb_to_hsla(0xE65050);
    let success = rgb_to_hsla(0x86B300);
    let warning = rgb_to_hsla(0xF2AE49);
    let info = rgb_to_hsla(0x399EE6);

    persist_font_config(theme, ui_family);
    apply_style_radius(theme, style);

    theme.background = background;
    theme.foreground = foreground;
    theme.border = border;
    theme.caret = accent;

    theme.muted = muted;
    theme.muted_foreground = muted;

    theme.primary = accent;
    theme.primary_hover = rgb_to_hsla(0xE68A3A);
    theme.primary_active = rgb_to_hsla(0xCC7A33);
    theme.primary_foreground = rgb_to_hsla(0x0A0E14);

    theme.secondary = raised;
    theme.secondary_hover = rgb_to_hsla(0xE4E4E4);
    theme.secondary_active = rgb_to_hsla(0xDADADA);
    theme.secondary_foreground = foreground;

    theme.accent = rgb_to_hsla_alpha(0x5C6166, 0.06);
    theme.accent_foreground = foreground;

    theme.danger = error;
    theme.danger_hover = rgb_to_hsla(0xCF4848);
    theme.danger_active = rgb_to_hsla(0xB84040);
    // White foreground on danger red — higher contrast than near-black 0x0A0E14
    theme.danger_foreground = rgb_to_hsla(0xFFFFFF);

    theme.success = success;
    theme.success_hover = rgb_to_hsla(0x79A100);
    theme.success_active = rgb_to_hsla(0x6D9000);
    theme.success_foreground = rgb_to_hsla(0x0A0E14);

    theme.warning = warning;
    theme.warning_hover = rgb_to_hsla(0xDA9D42);
    theme.warning_active = rgb_to_hsla(0xC28C3B);
    theme.warning_foreground = rgb_to_hsla(0x0A0E14);

    theme.info = info;
    theme.info_hover = rgb_to_hsla(0x338ECF);
    theme.info_active = rgb_to_hsla(0x2D7EB8);
    theme.info_foreground = rgb_to_hsla(0x0A0E14);

    theme.popover = raised;
    theme.popover_foreground = foreground;

    theme.selection = selection;

    theme.ring = rgb_to_hsla_alpha(0xFF9940, 0.5);

    theme.input = rgb_to_hsla_alpha(0x5C6166, 0.06);

    theme.scrollbar = background;
    theme.scrollbar_thumb = rgb_to_hsla_alpha(0x5C6166, 0.15);
    theme.scrollbar_thumb_hover = rgb_to_hsla_alpha(0x5C6166, 0.3);

    theme.sidebar = background;
    theme.sidebar_foreground = foreground;
    theme.sidebar_border = border;
    theme.sidebar_accent = rgb_to_hsla_alpha(0x5C6166, 0.06);
    theme.sidebar_accent_foreground = foreground;
    theme.sidebar_primary = accent;
    theme.sidebar_primary_foreground = rgb_to_hsla(0x0A0E14);

    theme.tab = panel;
    theme.tab_bar = panel;
    theme.tab_foreground = muted;
    theme.tab_active = background;
    theme.tab_active_foreground = foreground;
    theme.tab_bar_segmented = raised;

    theme.table = background;
    theme.table_head = panel;
    theme.table_head_foreground = muted;
    theme.table_even = rgb_to_hsla_alpha(0x5C6166, 0.03);
    theme.table_hover = rgb_to_hsla_alpha(0x5C6166, 0.06);
    theme.table_active = rgb_to_hsla_alpha(0x399EE6, 0.12);
    theme.table_active_border = rgb_to_hsla_alpha(0x399EE6, 0.4);
    // No row dividers — alternating tint (table_even) provides visual separation
    theme.table_row_border = hsla(0.0, 0.0, 0.0, 0.0);

    theme.list = background;
    theme.list_head = panel;
    theme.list_even = rgb_to_hsla_alpha(0x5C6166, 0.03);
    theme.list_hover = rgb_to_hsla_alpha(0x5C6166, 0.06);
    theme.list_active = selection;
    theme.list_active_border = accent;

    theme.accordion = panel;
    theme.accordion_hover = raised;

    theme.title_bar = panel;
    theme.title_bar_border = border;

    theme.tiles = rgb_to_hsla(0xE8E8E8);

    theme.overlay = rgb_to_hsla_alpha(0x000000, 0.3);

    theme.window_border = border;

    theme.link = info;
    theme.link_hover = rgb_to_hsla(0x4CADF0);
    theme.link_active = rgb_to_hsla(0x338ECF);

    theme.switch = muted;
    theme.switch_thumb = rgb_to_hsla(0xFFFFFF);

    theme.slider_bar = muted;
    theme.slider_thumb = accent;

    theme.progress_bar = accent;

    theme.skeleton = raised;

    theme.description_list_label = panel;
    theme.description_list_label_foreground = muted;

    theme.drag_border = accent;
    theme.drop_target = rgb_to_hsla_alpha(0xFF9940, 0.1);

    theme.group_box = panel;
    theme.group_box_foreground = foreground;

    theme.chart_1 = rgb_to_hsla(0x399EE6);
    theme.chart_2 = rgb_to_hsla(0x86B300);
    theme.chart_3 = rgb_to_hsla(0xFF9940);
    theme.chart_4 = rgb_to_hsla(0xE65050);
    theme.chart_5 = rgb_to_hsla(0xA37ACC);

    theme.bullish = success;
    theme.bearish = error;

    theme.red = error;
    theme.red_light = rgb_to_hsla(0xF09090);
    theme.green = success;
    theme.green_light = rgb_to_hsla(0xB8D96E);
    theme.blue = info;
    theme.blue_light = rgb_to_hsla(0x73B8F0);
    theme.yellow = warning;
    theme.yellow_light = rgb_to_hsla(0xF5C880);
    theme.magenta = rgb_to_hsla(0xA37ACC);
    theme.magenta_light = rgb_to_hsla(0xC4A6E0);
    theme.cyan = rgb_to_hsla(0x4CBF99);
    theme.cyan_light = rgb_to_hsla(0x86D9BF);
}

fn apply_nord(style: AppStyle, cx: &mut App) {
    let ui_family = AppFonts::current_ui_family(cx);
    let theme = Theme::global_mut(cx);

    // Nord palette
    let background = rgb_to_hsla(0x2E3440);
    let panel = rgb_to_hsla(0x3B4252);
    let foreground = rgb_to_hsla(0xD8DEE9);
    let muted = rgb_to_hsla(0x8A94A8);
    let accent = rgb_to_hsla(0x88C0D0);
    let border = rgb_to_hsla(0x434C5E);

    let raised = rgb_to_hsla(0x3B4252);
    let selection = rgb_to_hsla(0x4C566A);

    let error = rgb_to_hsla(0xBF616A);
    let success = rgb_to_hsla(0xA3BE8C);
    let warning = rgb_to_hsla(0xEBCB8B);
    let info = rgb_to_hsla(0x81A1C1);

    persist_font_config(theme, ui_family);
    apply_style_radius(theme, style);
    apply_editor_chrome(theme, background, raised, muted, foreground);

    theme.background = background;
    theme.foreground = foreground;
    theme.border = border;
    theme.caret = accent;

    theme.muted = muted;
    theme.muted_foreground = muted;

    theme.primary = accent;
    theme.primary_hover = rgb_to_hsla(0x98CCDC);
    theme.primary_active = rgb_to_hsla(0x7AB0C0);
    theme.primary_foreground = rgb_to_hsla(0x2E3440);

    theme.secondary = raised;
    theme.secondary_hover = rgb_to_hsla(0x465063);
    theme.secondary_active = rgb_to_hsla(0x4C566A);
    theme.secondary_foreground = foreground;

    theme.accent = rgb_to_hsla_alpha(0xD8DEE9, 0.06);
    theme.accent_foreground = foreground;

    theme.danger = error;
    theme.danger_hover = rgb_to_hsla(0xAC525A);
    theme.danger_active = rgb_to_hsla(0x9A4A51);
    theme.danger_foreground = rgb_to_hsla(0xFFFFFF);

    theme.success = success;
    theme.success_hover = rgb_to_hsla(0x93AD7E);
    theme.success_active = rgb_to_hsla(0x839C70);
    theme.success_foreground = rgb_to_hsla(0x2E3440);

    theme.warning = warning;
    theme.warning_hover = rgb_to_hsla(0xD5B87C);
    theme.warning_active = rgb_to_hsla(0xC0A66E);
    theme.warning_foreground = rgb_to_hsla(0x2E3440);

    theme.info = info;
    theme.info_hover = rgb_to_hsla(0x7490AE);
    theme.info_active = rgb_to_hsla(0x68819C);
    theme.info_foreground = rgb_to_hsla(0x2E3440);

    theme.popover = raised;
    theme.popover_foreground = foreground;

    theme.selection = selection;
    theme.ring = rgb_to_hsla_alpha(0x88C0D0, 0.72);
    theme.input = rgb_to_hsla_alpha(0xD8DEE9, 0.09);

    theme.scrollbar = background;
    theme.scrollbar_thumb = rgb_to_hsla_alpha(0xD8DEE9, 0.14);
    theme.scrollbar_thumb_hover = rgb_to_hsla_alpha(0xD8DEE9, 0.22);

    theme.sidebar = background;
    theme.sidebar_foreground = foreground;
    theme.sidebar_border = border;
    theme.sidebar_accent = rgb_to_hsla_alpha(0xD8DEE9, 0.05);
    theme.sidebar_accent_foreground = foreground;
    theme.sidebar_primary = accent;
    theme.sidebar_primary_foreground = rgb_to_hsla(0x2E3440);

    theme.tab = panel;
    theme.tab_bar = panel;
    theme.tab_foreground = muted;
    theme.tab_active = background;
    theme.tab_active_foreground = foreground;
    theme.tab_bar_segmented = raised;

    theme.table = background;
    theme.table_head = panel;
    theme.table_head_foreground = muted;
    theme.table_even = rgb_to_hsla_alpha(0xD8DEE9, 0.02);
    theme.table_hover = rgb_to_hsla_alpha(0xD8DEE9, 0.05);
    theme.table_active = rgb_to_hsla_alpha(0x88C0D0, 0.15);
    theme.table_active_border = rgb_to_hsla_alpha(0x88C0D0, 0.5);
    theme.table_row_border = hsla(0.0, 0.0, 0.0, 0.0);

    theme.list = background;
    theme.list_head = panel;
    theme.list_even = rgb_to_hsla_alpha(0xD8DEE9, 0.02);
    theme.list_hover = rgb_to_hsla_alpha(0xD8DEE9, 0.05);
    theme.list_active = selection;
    theme.list_active_border = accent;

    theme.accordion = panel;
    theme.accordion_hover = raised;

    theme.title_bar = panel;
    theme.title_bar_border = border;

    theme.tiles = rgb_to_hsla(0x3B4252);
    theme.overlay = rgb_to_hsla_alpha(0x000000, 0.45);
    theme.window_border = border;

    theme.link = info;
    theme.link_hover = rgb_to_hsla(0x8CAED0);
    theme.link_active = rgb_to_hsla(0x7490AE);

    theme.switch = muted;
    theme.switch_thumb = foreground;

    theme.slider_bar = muted;
    theme.slider_thumb = accent;

    theme.progress_bar = accent;
    theme.skeleton = raised;

    theme.description_list_label = panel;
    theme.description_list_label_foreground = muted;

    theme.drag_border = accent;
    theme.drop_target = rgb_to_hsla_alpha(0x88C0D0, 0.1);

    theme.group_box = panel;
    theme.group_box_foreground = foreground;

    theme.chart_1 = rgb_to_hsla(0x88C0D0);
    theme.chart_2 = rgb_to_hsla(0xA3BE8C);
    theme.chart_3 = rgb_to_hsla(0xEBCB8B);
    theme.chart_4 = rgb_to_hsla(0xBF616A);
    theme.chart_5 = rgb_to_hsla(0xB48EAD);

    theme.bullish = success;
    theme.bearish = error;

    theme.red = error;
    theme.red_light = rgb_to_hsla(0xD08770);
    theme.green = success;
    theme.green_light = rgb_to_hsla(0xB4CCA0);
    theme.blue = info;
    theme.blue_light = rgb_to_hsla(0xA0BCD8);
    theme.yellow = warning;
    theme.yellow_light = rgb_to_hsla(0xF0D9A5);
    theme.magenta = rgb_to_hsla(0xB48EAD);
    theme.magenta_light = rgb_to_hsla(0xC6AAC0);
    theme.cyan = rgb_to_hsla(0x8FBCBB);
    theme.cyan_light = rgb_to_hsla(0xABCFCE);
}

fn apply_dracula(style: AppStyle, cx: &mut App) {
    let ui_family = AppFonts::current_ui_family(cx);
    let theme = Theme::global_mut(cx);

    // Dracula palette
    let background = rgb_to_hsla(0x282A36);
    let panel = rgb_to_hsla(0x343746);
    let foreground = rgb_to_hsla(0xF8F8F2);
    let muted = rgb_to_hsla(0x9093A6);
    let accent = rgb_to_hsla(0xBD93F9);
    let border = rgb_to_hsla(0x44475A);

    let raised = rgb_to_hsla(0x343746);
    let selection = rgb_to_hsla(0x44475A);

    let error = rgb_to_hsla(0xFF5555);
    let success = rgb_to_hsla(0x50FA7B);
    let warning = rgb_to_hsla(0xF1FA8C);
    let info = rgb_to_hsla(0x8BE9FD);

    persist_font_config(theme, ui_family);
    apply_style_radius(theme, style);
    apply_editor_chrome(theme, background, raised, muted, foreground);

    theme.background = background;
    theme.foreground = foreground;
    theme.border = border;
    theme.caret = accent;

    theme.muted = muted;
    theme.muted_foreground = muted;

    theme.primary = accent;
    theme.primary_hover = rgb_to_hsla(0xC6A6FA);
    theme.primary_active = rgb_to_hsla(0xAC85E0);
    theme.primary_foreground = rgb_to_hsla(0x282A36);

    theme.secondary = raised;
    theme.secondary_hover = rgb_to_hsla(0x3D4050);
    theme.secondary_active = rgb_to_hsla(0x44475A);
    theme.secondary_foreground = foreground;

    theme.accent = rgb_to_hsla_alpha(0xF8F8F2, 0.06);
    theme.accent_foreground = foreground;

    theme.danger = error;
    theme.danger_hover = rgb_to_hsla(0xE04B4B);
    theme.danger_active = rgb_to_hsla(0xC64141);
    theme.danger_foreground = rgb_to_hsla(0x282A36);

    theme.success = success;
    theme.success_hover = rgb_to_hsla(0x48E170);
    theme.success_active = rgb_to_hsla(0x40C864);
    theme.success_foreground = rgb_to_hsla(0x282A36);

    theme.warning = warning;
    theme.warning_hover = rgb_to_hsla(0xDCE47E);
    theme.warning_active = rgb_to_hsla(0xC6CD70);
    theme.warning_foreground = rgb_to_hsla(0x282A36);

    theme.info = info;
    theme.info_hover = rgb_to_hsla(0x7DD2E4);
    theme.info_active = rgb_to_hsla(0x6EBDCE);
    theme.info_foreground = rgb_to_hsla(0x282A36);

    theme.popover = raised;
    theme.popover_foreground = foreground;

    theme.selection = selection;
    theme.ring = rgb_to_hsla_alpha(0xBD93F9, 0.72);
    theme.input = rgb_to_hsla_alpha(0xF8F8F2, 0.09);

    theme.scrollbar = background;
    theme.scrollbar_thumb = rgb_to_hsla_alpha(0xF8F8F2, 0.14);
    theme.scrollbar_thumb_hover = rgb_to_hsla_alpha(0xF8F8F2, 0.22);

    theme.sidebar = background;
    theme.sidebar_foreground = foreground;
    theme.sidebar_border = border;
    theme.sidebar_accent = rgb_to_hsla_alpha(0xF8F8F2, 0.05);
    theme.sidebar_accent_foreground = foreground;
    theme.sidebar_primary = accent;
    theme.sidebar_primary_foreground = rgb_to_hsla(0x282A36);

    theme.tab = panel;
    theme.tab_bar = panel;
    theme.tab_foreground = muted;
    theme.tab_active = background;
    theme.tab_active_foreground = foreground;
    theme.tab_bar_segmented = raised;

    theme.table = background;
    theme.table_head = panel;
    theme.table_head_foreground = muted;
    theme.table_even = rgb_to_hsla_alpha(0xF8F8F2, 0.02);
    theme.table_hover = rgb_to_hsla_alpha(0xF8F8F2, 0.05);
    theme.table_active = rgb_to_hsla_alpha(0xBD93F9, 0.15);
    theme.table_active_border = rgb_to_hsla_alpha(0xBD93F9, 0.5);
    theme.table_row_border = hsla(0.0, 0.0, 0.0, 0.0);

    theme.list = background;
    theme.list_head = panel;
    theme.list_even = rgb_to_hsla_alpha(0xF8F8F2, 0.02);
    theme.list_hover = rgb_to_hsla_alpha(0xF8F8F2, 0.05);
    theme.list_active = selection;
    theme.list_active_border = accent;

    theme.accordion = panel;
    theme.accordion_hover = raised;

    theme.title_bar = panel;
    theme.title_bar_border = border;

    theme.tiles = rgb_to_hsla(0x343746);
    theme.overlay = rgb_to_hsla_alpha(0x000000, 0.45);
    theme.window_border = border;

    theme.link = info;
    theme.link_hover = rgb_to_hsla(0x9BEEFE);
    theme.link_active = rgb_to_hsla(0x7DD2E4);

    theme.switch = muted;
    theme.switch_thumb = foreground;

    theme.slider_bar = muted;
    theme.slider_thumb = accent;

    theme.progress_bar = accent;
    theme.skeleton = raised;

    theme.description_list_label = panel;
    theme.description_list_label_foreground = muted;

    theme.drag_border = accent;
    theme.drop_target = rgb_to_hsla_alpha(0xBD93F9, 0.1);

    theme.group_box = panel;
    theme.group_box_foreground = foreground;

    theme.chart_1 = rgb_to_hsla(0x8BE9FD);
    theme.chart_2 = rgb_to_hsla(0x50FA7B);
    theme.chart_3 = rgb_to_hsla(0xF1FA8C);
    theme.chart_4 = rgb_to_hsla(0xFF5555);
    theme.chart_5 = rgb_to_hsla(0xFF79C6);

    theme.bullish = success;
    theme.bearish = error;

    theme.red = error;
    theme.red_light = rgb_to_hsla(0xFF7777);
    theme.green = success;
    theme.green_light = rgb_to_hsla(0x7BFB9C);
    theme.blue = info;
    theme.blue_light = rgb_to_hsla(0xA9F0FE);
    theme.yellow = warning;
    theme.yellow_light = rgb_to_hsla(0xF4FBA6);
    theme.magenta = rgb_to_hsla(0xFF79C6);
    theme.magenta_light = rgb_to_hsla(0xFF9AD4);
    theme.cyan = rgb_to_hsla(0x8BE9FD);
    theme.cyan_light = rgb_to_hsla(0xA9F0FE);
}

fn apply_catppuccin_latte(style: AppStyle, cx: &mut App) {
    let ui_family = AppFonts::current_ui_family(cx);
    let theme = Theme::global_mut(cx);

    // Catppuccin Latte palette
    let background = rgb_to_hsla(0xEFF1F5);
    let panel = rgb_to_hsla(0xE6E9EF);
    let foreground = rgb_to_hsla(0x4C4F69);
    let muted = rgb_to_hsla(0x8C8FA1);
    let accent = rgb_to_hsla(0x1E66F5);
    let border = rgb_to_hsla(0xDCE0E8);

    let raised = rgb_to_hsla(0xF4F5F8);
    let selection = rgb_to_hsla(0xDCE0E8);

    let error = rgb_to_hsla(0xD20F39);
    let success = rgb_to_hsla(0x40A02B);
    let warning = rgb_to_hsla(0xDF8E1D);
    let info = rgb_to_hsla(0x1E66F5);

    persist_font_config(theme, ui_family);
    apply_style_radius(theme, style);

    theme.background = background;
    theme.foreground = foreground;
    theme.border = border;
    theme.caret = accent;

    theme.muted = muted;
    theme.muted_foreground = muted;

    theme.primary = accent;
    theme.primary_hover = rgb_to_hsla(0x1A5AD9);
    theme.primary_active = rgb_to_hsla(0x174FC0);
    theme.primary_foreground = rgb_to_hsla(0xFFFFFF);

    theme.secondary = raised;
    theme.secondary_hover = rgb_to_hsla(0xE1E3E8);
    theme.secondary_active = rgb_to_hsla(0xD7D9E0);
    theme.secondary_foreground = foreground;

    theme.accent = rgb_to_hsla_alpha(0x4C4F69, 0.06);
    theme.accent_foreground = foreground;

    theme.danger = error;
    theme.danger_hover = rgb_to_hsla(0xBC0E33);
    theme.danger_active = rgb_to_hsla(0xA90D2E);
    theme.danger_foreground = rgb_to_hsla(0xFFFFFF);

    theme.success = success;
    theme.success_hover = rgb_to_hsla(0x399027);
    theme.success_active = rgb_to_hsla(0x328022);
    theme.success_foreground = rgb_to_hsla(0xFFFFFF);

    theme.warning = warning;
    theme.warning_hover = rgb_to_hsla(0xC87E1A);
    theme.warning_active = rgb_to_hsla(0xB27117);
    theme.warning_foreground = rgb_to_hsla(0xFFFFFF);

    theme.info = info;
    theme.info_hover = rgb_to_hsla(0x1A5AD9);
    theme.info_active = rgb_to_hsla(0x174FC0);
    theme.info_foreground = rgb_to_hsla(0xFFFFFF);

    theme.popover = raised;
    theme.popover_foreground = foreground;

    theme.selection = selection;
    theme.ring = rgb_to_hsla_alpha(0x1E66F5, 0.5);
    theme.input = rgb_to_hsla_alpha(0x4C4F69, 0.06);

    theme.scrollbar = background;
    theme.scrollbar_thumb = rgb_to_hsla_alpha(0x4C4F69, 0.15);
    theme.scrollbar_thumb_hover = rgb_to_hsla_alpha(0x4C4F69, 0.3);

    theme.sidebar = background;
    theme.sidebar_foreground = foreground;
    theme.sidebar_border = border;
    theme.sidebar_accent = rgb_to_hsla_alpha(0x4C4F69, 0.06);
    theme.sidebar_accent_foreground = foreground;
    theme.sidebar_primary = accent;
    theme.sidebar_primary_foreground = rgb_to_hsla(0xFFFFFF);

    theme.tab = panel;
    theme.tab_bar = panel;
    theme.tab_foreground = muted;
    theme.tab_active = background;
    theme.tab_active_foreground = foreground;
    theme.tab_bar_segmented = raised;

    theme.table = background;
    theme.table_head = panel;
    theme.table_head_foreground = muted;
    theme.table_even = rgb_to_hsla_alpha(0x4C4F69, 0.03);
    theme.table_hover = rgb_to_hsla_alpha(0x4C4F69, 0.06);
    theme.table_active = rgb_to_hsla_alpha(0x1E66F5, 0.12);
    theme.table_active_border = rgb_to_hsla_alpha(0x1E66F5, 0.4);
    theme.table_row_border = hsla(0.0, 0.0, 0.0, 0.0);

    theme.list = background;
    theme.list_head = panel;
    theme.list_even = rgb_to_hsla_alpha(0x4C4F69, 0.03);
    theme.list_hover = rgb_to_hsla_alpha(0x4C4F69, 0.06);
    theme.list_active = selection;
    theme.list_active_border = accent;

    theme.accordion = panel;
    theme.accordion_hover = raised;

    theme.title_bar = panel;
    theme.title_bar_border = border;

    theme.tiles = rgb_to_hsla(0xE6E9EF);
    theme.overlay = rgb_to_hsla_alpha(0x000000, 0.3);
    theme.window_border = border;

    theme.link = info;
    theme.link_hover = rgb_to_hsla(0x3B76F6);
    theme.link_active = rgb_to_hsla(0x1A5AD9);

    theme.switch = muted;
    theme.switch_thumb = rgb_to_hsla(0xFFFFFF);

    theme.slider_bar = muted;
    theme.slider_thumb = accent;

    theme.progress_bar = accent;
    theme.skeleton = raised;

    theme.description_list_label = panel;
    theme.description_list_label_foreground = muted;

    theme.drag_border = accent;
    theme.drop_target = rgb_to_hsla_alpha(0x1E66F5, 0.1);

    theme.group_box = panel;
    theme.group_box_foreground = foreground;

    theme.chart_1 = rgb_to_hsla(0x1E66F5);
    theme.chart_2 = rgb_to_hsla(0x40A02B);
    theme.chart_3 = rgb_to_hsla(0xDF8E1D);
    theme.chart_4 = rgb_to_hsla(0xD20F39);
    theme.chart_5 = rgb_to_hsla(0x8839EF);

    theme.bullish = success;
    theme.bearish = error;

    theme.red = error;
    theme.red_light = rgb_to_hsla(0xE64563);
    theme.green = success;
    theme.green_light = rgb_to_hsla(0x66B954);
    theme.blue = info;
    theme.blue_light = rgb_to_hsla(0x4A85F7);
    theme.yellow = warning;
    theme.yellow_light = rgb_to_hsla(0xE5A84B);
    theme.magenta = rgb_to_hsla(0x8839EF);
    theme.magenta_light = rgb_to_hsla(0xA26BF3);
    theme.cyan = rgb_to_hsla(0x04A5E5);
    theme.cyan_light = rgb_to_hsla(0x37B7EB);
}

fn apply_github_light(style: AppStyle, cx: &mut App) {
    let ui_family = AppFonts::current_ui_family(cx);
    let theme = Theme::global_mut(cx);

    // GitHub Light palette
    let background = rgb_to_hsla(0xFFFFFF);
    let panel = rgb_to_hsla(0xF6F8FA);
    let foreground = rgb_to_hsla(0x24292F);
    let muted = rgb_to_hsla(0x6E7781);
    let accent = rgb_to_hsla(0x0969DA);
    let border = rgb_to_hsla(0xD0D7DE);

    let raised = rgb_to_hsla(0xF6F8FA);
    let selection = rgb_to_hsla(0xAFE4FF);

    let error = rgb_to_hsla(0xCF222E);
    let success = rgb_to_hsla(0x1A7F37);
    let warning = rgb_to_hsla(0x9A6700);
    let info = rgb_to_hsla(0x0969DA);

    persist_font_config(theme, ui_family);
    apply_style_radius(theme, style);

    theme.background = background;
    theme.foreground = foreground;
    theme.border = border;
    theme.caret = accent;

    theme.muted = muted;
    theme.muted_foreground = muted;

    theme.primary = accent;
    theme.primary_hover = rgb_to_hsla(0x0860C0);
    theme.primary_active = rgb_to_hsla(0x0756A8);
    theme.primary_foreground = rgb_to_hsla(0xFFFFFF);

    theme.secondary = raised;
    theme.secondary_hover = rgb_to_hsla(0xE1E4E8);
    theme.secondary_active = rgb_to_hsla(0xD5D9DE);
    theme.secondary_foreground = foreground;

    theme.accent = rgb_to_hsla_alpha(0x24292F, 0.06);
    theme.accent_foreground = foreground;

    theme.danger = error;
    theme.danger_hover = rgb_to_hsla(0xB81F2A);
    theme.danger_active = rgb_to_hsla(0xA31D26);
    theme.danger_foreground = rgb_to_hsla(0xFFFFFF);

    theme.success = success;
    theme.success_hover = rgb_to_hsla(0x177131);
    theme.success_active = rgb_to_hsla(0x14612A);
    theme.success_foreground = rgb_to_hsla(0xFFFFFF);

    theme.warning = warning;
    theme.warning_hover = rgb_to_hsla(0x8A5B00);
    theme.warning_active = rgb_to_hsla(0x7A5000);
    theme.warning_foreground = rgb_to_hsla(0xFFFFFF);

    theme.info = info;
    theme.info_hover = rgb_to_hsla(0x0860C0);
    theme.info_active = rgb_to_hsla(0x0756A8);
    theme.info_foreground = rgb_to_hsla(0xFFFFFF);

    theme.popover = raised;
    theme.popover_foreground = foreground;

    theme.selection = selection;
    theme.ring = rgb_to_hsla_alpha(0x0969DA, 0.5);
    theme.input = rgb_to_hsla_alpha(0x24292F, 0.06);

    theme.scrollbar = background;
    theme.scrollbar_thumb = rgb_to_hsla_alpha(0x24292F, 0.15);
    theme.scrollbar_thumb_hover = rgb_to_hsla_alpha(0x24292F, 0.3);

    theme.sidebar = background;
    theme.sidebar_foreground = foreground;
    theme.sidebar_border = border;
    theme.sidebar_accent = rgb_to_hsla_alpha(0x24292F, 0.06);
    theme.sidebar_accent_foreground = foreground;
    theme.sidebar_primary = accent;
    theme.sidebar_primary_foreground = rgb_to_hsla(0xFFFFFF);

    theme.tab = panel;
    theme.tab_bar = panel;
    theme.tab_foreground = muted;
    theme.tab_active = background;
    theme.tab_active_foreground = foreground;
    theme.tab_bar_segmented = raised;

    theme.table = background;
    theme.table_head = panel;
    theme.table_head_foreground = muted;
    theme.table_even = rgb_to_hsla_alpha(0x24292F, 0.03);
    theme.table_hover = rgb_to_hsla_alpha(0x24292F, 0.06);
    theme.table_active = rgb_to_hsla_alpha(0x0969DA, 0.12);
    theme.table_active_border = rgb_to_hsla_alpha(0x0969DA, 0.4);
    theme.table_row_border = hsla(0.0, 0.0, 0.0, 0.0);

    theme.list = background;
    theme.list_head = panel;
    theme.list_even = rgb_to_hsla_alpha(0x24292F, 0.03);
    theme.list_hover = rgb_to_hsla_alpha(0x24292F, 0.06);
    theme.list_active = selection;
    theme.list_active_border = accent;

    theme.accordion = panel;
    theme.accordion_hover = raised;

    theme.title_bar = panel;
    theme.title_bar_border = border;

    theme.tiles = rgb_to_hsla(0xF6F8FA);
    theme.overlay = rgb_to_hsla_alpha(0x000000, 0.3);
    theme.window_border = border;

    theme.link = info;
    theme.link_hover = rgb_to_hsla(0x277FE8);
    theme.link_active = rgb_to_hsla(0x0860C0);

    theme.switch = muted;
    theme.switch_thumb = rgb_to_hsla(0xFFFFFF);

    theme.slider_bar = muted;
    theme.slider_thumb = accent;

    theme.progress_bar = accent;
    theme.skeleton = raised;

    theme.description_list_label = panel;
    theme.description_list_label_foreground = muted;

    theme.drag_border = accent;
    theme.drop_target = rgb_to_hsla_alpha(0x0969DA, 0.1);

    theme.group_box = panel;
    theme.group_box_foreground = foreground;

    theme.chart_1 = rgb_to_hsla(0x0969DA);
    theme.chart_2 = rgb_to_hsla(0x1A7F37);
    theme.chart_3 = rgb_to_hsla(0x9A6700);
    theme.chart_4 = rgb_to_hsla(0xCF222E);
    theme.chart_5 = rgb_to_hsla(0x8250DF);

    theme.bullish = success;
    theme.bearish = error;

    theme.red = error;
    theme.red_light = rgb_to_hsla(0xE5534B);
    theme.green = success;
    theme.green_light = rgb_to_hsla(0x4DB26C);
    theme.blue = info;
    theme.blue_light = rgb_to_hsla(0x4A93E8);
    theme.yellow = warning;
    theme.yellow_light = rgb_to_hsla(0xBF8700);
    theme.magenta = rgb_to_hsla(0x8250DF);
    theme.magenta_light = rgb_to_hsla(0x9E7AE8);
    theme.cyan = rgb_to_hsla(0x1B7C83);
    theme.cyan_light = rgb_to_hsla(0x499EA4);
}

fn apply_one_light(style: AppStyle, cx: &mut App) {
    let ui_family = AppFonts::current_ui_family(cx);
    let theme = Theme::global_mut(cx);

    // One Light palette
    let background = rgb_to_hsla(0xFAFAFA);
    let panel = rgb_to_hsla(0xF0F0F1);
    let foreground = rgb_to_hsla(0x383A42);
    let muted = rgb_to_hsla(0xA0A1A7);
    let accent = rgb_to_hsla(0x4078F2);
    let border = rgb_to_hsla(0xE5E5E6);

    let raised = rgb_to_hsla(0xF7F7F8);
    let selection = rgb_to_hsla(0xD7E2F9);

    let error = rgb_to_hsla(0xE45649);
    let success = rgb_to_hsla(0x50A14F);
    let warning = rgb_to_hsla(0xC18401);
    let info = rgb_to_hsla(0x4078F2);

    persist_font_config(theme, ui_family);
    apply_style_radius(theme, style);

    theme.background = background;
    theme.foreground = foreground;
    theme.border = border;
    theme.caret = accent;

    theme.muted = muted;
    theme.muted_foreground = muted;

    theme.primary = accent;
    theme.primary_hover = rgb_to_hsla(0x396CD7);
    theme.primary_active = rgb_to_hsla(0x3260C0);
    theme.primary_foreground = rgb_to_hsla(0xFFFFFF);

    theme.secondary = raised;
    theme.secondary_hover = rgb_to_hsla(0xE5E5E6);
    theme.secondary_active = rgb_to_hsla(0xDADADC);
    theme.secondary_foreground = foreground;

    theme.accent = rgb_to_hsla_alpha(0x383A42, 0.06);
    theme.accent_foreground = foreground;

    theme.danger = error;
    theme.danger_hover = rgb_to_hsla(0xCC4D41);
    theme.danger_active = rgb_to_hsla(0xB8453A);
    theme.danger_foreground = rgb_to_hsla(0xFFFFFF);

    theme.success = success;
    theme.success_hover = rgb_to_hsla(0x479047);
    theme.success_active = rgb_to_hsla(0x3F803F);
    theme.success_foreground = rgb_to_hsla(0xFFFFFF);

    theme.warning = warning;
    theme.warning_hover = rgb_to_hsla(0xAC7701);
    theme.warning_active = rgb_to_hsla(0x976A01);
    theme.warning_foreground = rgb_to_hsla(0xFFFFFF);

    theme.info = info;
    theme.info_hover = rgb_to_hsla(0x396CD7);
    theme.info_active = rgb_to_hsla(0x3260C0);
    theme.info_foreground = rgb_to_hsla(0xFFFFFF);

    theme.popover = raised;
    theme.popover_foreground = foreground;

    theme.selection = selection;
    theme.ring = rgb_to_hsla_alpha(0x4078F2, 0.5);
    theme.input = rgb_to_hsla_alpha(0x383A42, 0.06);

    theme.scrollbar = background;
    theme.scrollbar_thumb = rgb_to_hsla_alpha(0x383A42, 0.15);
    theme.scrollbar_thumb_hover = rgb_to_hsla_alpha(0x383A42, 0.3);

    theme.sidebar = background;
    theme.sidebar_foreground = foreground;
    theme.sidebar_border = border;
    theme.sidebar_accent = rgb_to_hsla_alpha(0x383A42, 0.06);
    theme.sidebar_accent_foreground = foreground;
    theme.sidebar_primary = accent;
    theme.sidebar_primary_foreground = rgb_to_hsla(0xFFFFFF);

    theme.tab = panel;
    theme.tab_bar = panel;
    theme.tab_foreground = muted;
    theme.tab_active = background;
    theme.tab_active_foreground = foreground;
    theme.tab_bar_segmented = raised;

    theme.table = background;
    theme.table_head = panel;
    theme.table_head_foreground = muted;
    theme.table_even = rgb_to_hsla_alpha(0x383A42, 0.03);
    theme.table_hover = rgb_to_hsla_alpha(0x383A42, 0.06);
    theme.table_active = rgb_to_hsla_alpha(0x4078F2, 0.12);
    theme.table_active_border = rgb_to_hsla_alpha(0x4078F2, 0.4);
    theme.table_row_border = hsla(0.0, 0.0, 0.0, 0.0);

    theme.list = background;
    theme.list_head = panel;
    theme.list_even = rgb_to_hsla_alpha(0x383A42, 0.03);
    theme.list_hover = rgb_to_hsla_alpha(0x383A42, 0.06);
    theme.list_active = selection;
    theme.list_active_border = accent;

    theme.accordion = panel;
    theme.accordion_hover = raised;

    theme.title_bar = panel;
    theme.title_bar_border = border;

    theme.tiles = rgb_to_hsla(0xF0F0F1);
    theme.overlay = rgb_to_hsla_alpha(0x000000, 0.3);
    theme.window_border = border;

    theme.link = info;
    theme.link_hover = rgb_to_hsla(0x5B8DF4);
    theme.link_active = rgb_to_hsla(0x396CD7);

    theme.switch = muted;
    theme.switch_thumb = rgb_to_hsla(0xFFFFFF);

    theme.slider_bar = muted;
    theme.slider_thumb = accent;

    theme.progress_bar = accent;
    theme.skeleton = raised;

    theme.description_list_label = panel;
    theme.description_list_label_foreground = muted;

    theme.drag_border = accent;
    theme.drop_target = rgb_to_hsla_alpha(0x4078F2, 0.1);

    theme.group_box = panel;
    theme.group_box_foreground = foreground;

    theme.chart_1 = rgb_to_hsla(0x4078F2);
    theme.chart_2 = rgb_to_hsla(0x50A14F);
    theme.chart_3 = rgb_to_hsla(0xC18401);
    theme.chart_4 = rgb_to_hsla(0xE45649);
    theme.chart_5 = rgb_to_hsla(0xA626A4);

    theme.bullish = success;
    theme.bearish = error;

    theme.red = error;
    theme.red_light = rgb_to_hsla(0xEC7A70);
    theme.green = success;
    theme.green_light = rgb_to_hsla(0x74B873);
    theme.blue = info;
    theme.blue_light = rgb_to_hsla(0x6A96F5);
    theme.yellow = warning;
    theme.yellow_light = rgb_to_hsla(0xD2A334);
    theme.magenta = rgb_to_hsla(0xA626A4);
    theme.magenta_light = rgb_to_hsla(0xBA52B8);
    theme.cyan = rgb_to_hsla(0x0184BC);
    theme.cyan_light = rgb_to_hsla(0x349DCD);
}

/// VS Code Dark Modern chrome, shipped as Dory Dark.
fn apply_dory_dark(style: AppStyle, cx: &mut App) {
    let ui_family = AppFonts::current_ui_family(cx);
    let theme = Theme::global_mut(cx);

    let background = rgb_to_hsla(0x1F1F1F);
    let panel = rgb_to_hsla(0x181818);
    let foreground = rgb_to_hsla(0xCCCCCC);
    let muted = rgb_to_hsla(0x9D9D9D);
    let accent = rgb_to_hsla(0x0078D4);
    let border = rgb_to_hsla(0x2B2B2B);

    let raised = rgb_to_hsla(0x222222);
    let selection = rgb_to_hsla(0x04395E);

    let error = rgb_to_hsla(0xF85149);
    let success = rgb_to_hsla(0x2EA043);
    let warning = rgb_to_hsla(0xCCA700);
    let info = rgb_to_hsla(0x4DAAFC);

    persist_font_config(theme, ui_family);
    apply_style_radius(theme, style);
    apply_editor_chrome(theme, background, raised, muted, foreground);

    theme.background = background;
    theme.foreground = foreground;
    theme.border = border;
    theme.caret = accent;

    theme.muted = muted;
    theme.muted_foreground = muted;

    theme.primary = accent;
    theme.primary_hover = rgb_to_hsla(0x026EC1);
    theme.primary_active = rgb_to_hsla(0x025EA8);
    theme.primary_foreground = rgb_to_hsla(0xFFFFFF);

    theme.secondary = rgb_to_hsla(0x313131);
    theme.secondary_hover = rgb_to_hsla(0x3C3C3C);
    theme.secondary_active = rgb_to_hsla(0x454545);
    theme.secondary_foreground = foreground;

    theme.accent = rgb_to_hsla_alpha(0xFFFFFF, 0.06);
    theme.accent_foreground = foreground;

    theme.danger = error;
    theme.danger_hover = rgb_to_hsla(0xE04640);
    theme.danger_active = rgb_to_hsla(0xC93C36);
    theme.danger_foreground = rgb_to_hsla(0xFFFFFF);

    theme.success = success;
    theme.success_hover = rgb_to_hsla(0x3FB950);
    theme.success_active = rgb_to_hsla(0x238636);
    theme.success_foreground = rgb_to_hsla(0xFFFFFF);

    theme.warning = warning;
    theme.warning_hover = rgb_to_hsla(0xD4B106);
    theme.warning_active = rgb_to_hsla(0xBB8009);
    theme.warning_foreground = rgb_to_hsla(0x1F1F1F);

    theme.info = info;
    theme.info_hover = rgb_to_hsla(0x79C0FF);
    theme.info_active = rgb_to_hsla(0x388BFD);
    theme.info_foreground = rgb_to_hsla(0x1F1F1F);

    theme.popover = raised;
    theme.popover_foreground = foreground;

    theme.selection = selection;
    theme.ring = rgb_to_hsla_alpha(0x0078D4, 0.72);
    theme.input = rgb_to_hsla(0x313131);

    theme.scrollbar = background;
    theme.scrollbar_thumb = rgb_to_hsla_alpha(0xCCCCCC, 0.14);
    theme.scrollbar_thumb_hover = rgb_to_hsla_alpha(0xCCCCCC, 0.22);

    theme.sidebar = panel;
    theme.sidebar_foreground = foreground;
    theme.sidebar_border = border;
    theme.sidebar_accent = rgb_to_hsla_alpha(0xFFFFFF, 0.05);
    theme.sidebar_accent_foreground = foreground;
    theme.sidebar_primary = accent;
    theme.sidebar_primary_foreground = rgb_to_hsla(0xFFFFFF);

    theme.tab = panel;
    theme.tab_bar = panel;
    theme.tab_foreground = muted;
    theme.tab_active = background;
    theme.tab_active_foreground = rgb_to_hsla(0xFFFFFF);
    theme.tab_bar_segmented = raised;

    theme.table = background;
    theme.table_head = panel;
    theme.table_head_foreground = muted;
    theme.table_even = rgb_to_hsla_alpha(0xFFFFFF, 0.02);
    theme.table_hover = rgb_to_hsla_alpha(0xFFFFFF, 0.05);
    theme.table_active = rgb_to_hsla_alpha(0x0078D4, 0.18);
    theme.table_active_border = rgb_to_hsla_alpha(0x0078D4, 0.5);
    theme.table_row_border = hsla(0.0, 0.0, 0.0, 0.0);

    theme.list = background;
    theme.list_head = panel;
    theme.list_even = rgb_to_hsla_alpha(0xFFFFFF, 0.02);
    theme.list_hover = rgb_to_hsla_alpha(0xFFFFFF, 0.05);
    theme.list_active = selection;
    theme.list_active_border = accent;

    theme.accordion = panel;
    theme.accordion_hover = raised;

    theme.title_bar = panel;
    theme.title_bar_border = border;

    theme.tiles = rgb_to_hsla(0x181818);
    theme.overlay = rgb_to_hsla_alpha(0x000000, 0.55);
    theme.window_border = border;

    theme.link = info;
    theme.link_hover = rgb_to_hsla(0x79C0FF);
    theme.link_active = rgb_to_hsla(0x388BFD);

    theme.switch = muted;
    theme.switch_thumb = foreground;

    theme.slider_bar = muted;
    theme.slider_thumb = accent;

    theme.progress_bar = accent;
    theme.skeleton = raised;

    theme.description_list_label = panel;
    theme.description_list_label_foreground = muted;

    theme.drag_border = accent;
    theme.drop_target = rgb_to_hsla_alpha(0x0078D4, 0.1);

    theme.group_box = panel;
    theme.group_box_foreground = foreground;

    theme.chart_1 = info;
    theme.chart_2 = success;
    theme.chart_3 = warning;
    theme.chart_4 = error;
    theme.chart_5 = rgb_to_hsla(0xC586C0);

    theme.bullish = success;
    theme.bearish = error;

    theme.red = error;
    theme.red_light = rgb_to_hsla(0xFF7B72);
    theme.green = success;
    theme.green_light = rgb_to_hsla(0x56D364);
    theme.blue = info;
    theme.blue_light = rgb_to_hsla(0x79C0FF);
    theme.yellow = warning;
    theme.yellow_light = rgb_to_hsla(0xE3B341);
    theme.magenta = rgb_to_hsla(0xC586C0);
    theme.magenta_light = rgb_to_hsla(0xD2A8D0);
    theme.cyan = rgb_to_hsla(0x4EC9B0);
    theme.cyan_light = rgb_to_hsla(0x7AD7C4);
}

/// Zed One Light chrome, shipped as Dory Light.
fn apply_dory_light(style: AppStyle, cx: &mut App) {
    let ui_family = AppFonts::current_ui_family(cx);
    let theme = Theme::global_mut(cx);

    let background = rgb_to_hsla(0xFAFAFA);
    let panel = rgb_to_hsla(0xEBEBEC);
    let foreground = rgb_to_hsla(0x242529);
    let muted = rgb_to_hsla(0x58585A);
    let accent = rgb_to_hsla(0x5C78E2);
    let border = rgb_to_hsla(0xC9C9CA);

    let raised = rgb_to_hsla(0xDCDCDD);
    let selection = rgb_to_hsla(0xD7DDF6);

    let error = rgb_to_hsla(0xD36151);
    let success = rgb_to_hsla(0x669F59);
    let warning = rgb_to_hsla(0xA48819);
    let info = rgb_to_hsla(0x5C78E2);

    persist_font_config(theme, ui_family);
    apply_style_radius(theme, style);
    apply_editor_chrome(theme, background, raised, muted, foreground);

    theme.background = background;
    theme.foreground = foreground;
    theme.border = border;
    theme.caret = accent;

    theme.muted = muted;
    theme.muted_foreground = muted;

    theme.primary = accent;
    theme.primary_hover = rgb_to_hsla(0x4F68C8);
    theme.primary_active = rgb_to_hsla(0x455BB4);
    theme.primary_foreground = rgb_to_hsla(0xFFFFFF);

    theme.secondary = raised;
    theme.secondary_hover = rgb_to_hsla(0xD2D2D3);
    theme.secondary_active = rgb_to_hsla(0xC8C8C9);
    theme.secondary_foreground = foreground;

    theme.accent = rgb_to_hsla_alpha(0x242529, 0.06);
    theme.accent_foreground = foreground;

    theme.danger = error;
    theme.danger_hover = rgb_to_hsla(0xBE5749);
    theme.danger_active = rgb_to_hsla(0xA94D41);
    theme.danger_foreground = rgb_to_hsla(0xFFFFFF);

    theme.success = success;
    theme.success_hover = rgb_to_hsla(0x5C8F50);
    theme.success_active = rgb_to_hsla(0x527F47);
    theme.success_foreground = rgb_to_hsla(0xFFFFFF);

    theme.warning = warning;
    theme.warning_hover = rgb_to_hsla(0x937A16);
    theme.warning_active = rgb_to_hsla(0x826C14);
    theme.warning_foreground = rgb_to_hsla(0xFFFFFF);

    theme.info = info;
    theme.info_hover = rgb_to_hsla(0x4F68C8);
    theme.info_active = rgb_to_hsla(0x455BB4);
    theme.info_foreground = rgb_to_hsla(0xFFFFFF);

    theme.popover = raised;
    theme.popover_foreground = foreground;

    theme.selection = selection;
    theme.ring = rgb_to_hsla_alpha(0x5C78E2, 0.5);
    theme.input = rgb_to_hsla_alpha(0x242529, 0.06);

    theme.scrollbar = background;
    theme.scrollbar_thumb = rgb_to_hsla_alpha(0x242529, 0.15);
    theme.scrollbar_thumb_hover = rgb_to_hsla_alpha(0x242529, 0.3);

    theme.sidebar = background;
    theme.sidebar_foreground = foreground;
    theme.sidebar_border = border;
    theme.sidebar_accent = rgb_to_hsla_alpha(0x242529, 0.06);
    theme.sidebar_accent_foreground = foreground;
    theme.sidebar_primary = accent;
    theme.sidebar_primary_foreground = rgb_to_hsla(0xFFFFFF);

    theme.tab = panel;
    theme.tab_bar = panel;
    theme.tab_foreground = muted;
    theme.tab_active = background;
    theme.tab_active_foreground = foreground;
    theme.tab_bar_segmented = raised;

    theme.table = background;
    theme.table_head = panel;
    theme.table_head_foreground = muted;
    theme.table_even = rgb_to_hsla_alpha(0x242529, 0.03);
    theme.table_hover = rgb_to_hsla_alpha(0x242529, 0.06);
    theme.table_active = rgb_to_hsla_alpha(0x5C78E2, 0.12);
    theme.table_active_border = rgb_to_hsla_alpha(0x5C78E2, 0.4);
    theme.table_row_border = hsla(0.0, 0.0, 0.0, 0.0);

    theme.list = background;
    theme.list_head = panel;
    theme.list_even = rgb_to_hsla_alpha(0x242529, 0.03);
    theme.list_hover = rgb_to_hsla_alpha(0x242529, 0.06);
    theme.list_active = selection;
    theme.list_active_border = accent;

    theme.accordion = panel;
    theme.accordion_hover = raised;

    theme.title_bar = panel;
    theme.title_bar_border = border;

    theme.tiles = rgb_to_hsla(0xEBEBEC);
    theme.overlay = rgb_to_hsla_alpha(0x000000, 0.3);
    theme.window_border = border;

    theme.link = info;
    theme.link_hover = rgb_to_hsla(0x7A91E8);
    theme.link_active = rgb_to_hsla(0x4F68C8);

    theme.switch = muted;
    theme.switch_thumb = rgb_to_hsla(0xFFFFFF);

    theme.slider_bar = muted;
    theme.slider_thumb = accent;

    theme.progress_bar = accent;
    theme.skeleton = raised;

    theme.description_list_label = panel;
    theme.description_list_label_foreground = muted;

    theme.drag_border = accent;
    theme.drop_target = rgb_to_hsla_alpha(0x5C78E2, 0.1);

    theme.group_box = panel;
    theme.group_box_foreground = foreground;

    theme.chart_1 = info;
    theme.chart_2 = success;
    theme.chart_3 = warning;
    theme.chart_4 = error;
    theme.chart_5 = rgb_to_hsla(0xA65EB4);

    theme.bullish = success;
    theme.bearish = error;

    theme.red = error;
    theme.red_light = rgb_to_hsla(0xE08A7C);
    theme.green = success;
    theme.green_light = rgb_to_hsla(0x88C07A);
    theme.blue = info;
    theme.blue_light = rgb_to_hsla(0x8498EC);
    theme.yellow = warning;
    theme.yellow_light = rgb_to_hsla(0xC4A84A);
    theme.magenta = rgb_to_hsla(0xA65EB4);
    theme.magenta_light = rgb_to_hsla(0xC080CC);
    theme.cyan = rgb_to_hsla(0x3B7EA8);
    theme.cyan_light = rgb_to_hsla(0x5A9BC0);
}
