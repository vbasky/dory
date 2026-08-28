use super::SettingsSection;
use super::SettingsSectionId;
use super::section_trait::SectionFocusEvent;
use dory_components::controls::{
    Dropdown, DropdownItem, DropdownSelectionChanged, FontPicked, FontPicker,
};
use dory_components::controls::{InputEvent, InputState};
use dory_core::{
    AppStyle, FontSetting, GeneralSettings, RefreshPolicySetting, StartupFocus, ThemeModeSetting,
    ThemeSetting,
};
use dory_ui_base::AppStateEntity;
use gpui::prelude::*;
use gpui::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum GeneralFormRow {
    ThemeMode,
    DarkTheme,
    LightTheme,
    Style,
    Font,
    Language,
    RestoreSession,
    ReopenConnections,
    DefaultFocus,
    MaxHistory,
    AutoSaveInterval,
    DefaultRefreshPolicy,
    DefaultRefreshInterval,
    MaxBackgroundTasks,
    PauseRefreshOnError,
    RefreshOnlyIfVisible,
    ConfirmDangerous,
    RequiresWhere,
    RequiresPreview,
    ObjectPreviewLimit,
    ShareStableDb,
}

pub(super) struct GeneralSection {
    pub(super) app_state: Entity<AppStateEntity>,
    pub(super) gen_settings: GeneralSettings,
    pub(super) gen_form_cursor: usize,
    pub(super) gen_editing_field: bool,
    /// Nightly-only: whether this build is opted into the stable database.
    /// Backed by a pre-database marker file, applied on the next launch.
    pub(super) gen_share_stable_db: bool,
    pub(super) dropdown_theme_mode: Entity<Dropdown>,
    pub(super) dropdown_dark_theme: Entity<Dropdown>,
    pub(super) dropdown_light_theme: Entity<Dropdown>,
    pub(super) dropdown_style: Entity<Dropdown>,
    pub(super) font_picker: Entity<FontPicker>,
    pub(super) dropdown_language: Entity<Dropdown>,
    pub(super) dropdown_default_focus: Entity<Dropdown>,
    pub(super) dropdown_refresh_policy: Entity<Dropdown>,
    pub(super) input_max_history: Entity<InputState>,
    pub(super) input_auto_save: Entity<InputState>,
    pub(super) input_refresh_interval: Entity<InputState>,
    pub(super) input_max_bg_tasks: Entity<InputState>,
    pub(super) input_object_preview_limit: Entity<InputState>,
    pub(super) content_focused: bool,
    pub(super) switching_input: bool,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<SectionFocusEvent> for GeneralSection {}

impl GeneralSection {
    pub(super) fn new(
        app_state: Entity<AppStateEntity>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let settings = app_state.read(cx).general_settings().clone();
        let theme_mode_index = Self::theme_mode_index(settings.theme_mode);
        let dark_theme_index =
            Self::palette_index(ThemeSetting::dark_themes(), settings.dark_theme);
        let light_theme_index =
            Self::palette_index(ThemeSetting::light_themes(), settings.light_theme);
        let style_index = Self::style_index(settings.style);
        let font_families = Self::installed_font_families(cx);
        let current_family = if settings.ui_font.is_system() {
            String::new()
        } else {
            settings.ui_font.family.clone()
        };
        let language_index = Self::language_index(&settings.language);
        let startup_focus_index = Self::startup_focus_index(settings.default_focus_on_startup);
        let refresh_policy_index = Self::refresh_policy_index(settings.default_refresh_policy);
        let max_history = settings.max_history_entries.to_string();
        let auto_save_interval = settings.auto_save_interval_ms.to_string();
        let refresh_interval = settings.default_refresh_interval_secs.to_string();
        let max_background_tasks = settings.max_concurrent_background_tasks.to_string();
        let object_preview_limit = settings.object_preview_size_limit_mib.to_string();

        let dropdown_theme_mode = cx.new(move |_cx| {
            Dropdown::new("general-theme-mode")
                .placeholder(dory_i18n::t!("settings.general.theme_mode.label"))
                .items(Self::theme_mode_items())
                .selected_index(Some(theme_mode_index))
        });
        let dropdown_dark_theme = cx.new(move |_cx| {
            Dropdown::new("general-dark-theme")
                .placeholder(dory_i18n::t!("settings.general.dark_theme.label"))
                .items(Self::palette_items(ThemeSetting::dark_themes()))
                .selected_index(Some(dark_theme_index))
        });
        let dropdown_light_theme = cx.new(move |_cx| {
            Dropdown::new("general-light-theme")
                .placeholder(dory_i18n::t!("settings.general.light_theme.label"))
                .items(Self::palette_items(ThemeSetting::light_themes()))
                .selected_index(Some(light_theme_index))
        });
        let dropdown_style = cx.new(move |_cx| {
            Dropdown::new("general-style")
                .placeholder(dory_i18n::t!("settings.general.style.label"))
                .items(Self::style_items())
                .selected_index(Some(style_index))
        });
        let font_picker = cx.new(|cx| {
            let mut families = font_families.clone();
            if !families.iter().any(|f| f.is_empty()) {
                families.push(String::new());
            }
            let mut picker = FontPicker::new(
                window,
                cx,
                families.into_iter().map(SharedString::from).collect(),
            );
            picker.filtered = picker.families.clone();
            picker.filtered.sort_by_key(|f| f.to_lowercase());
            picker.set_committed(SharedString::from(current_family.clone()));
            if let Some(ix) = picker
                .filtered
                .iter()
                .position(|f| f.as_ref() == current_family.as_str())
            {
                picker.selected_index = ix;
            }
            picker
        });
        let font_picker_subscription = {
            let picker_entity = font_picker.clone();
            cx.subscribe(&picker_entity, |this, _, event: &FontPicked, cx| {
                let next = if event.family.is_empty() {
                    FontSetting::system()
                } else {
                    FontSetting::named(event.family.to_string())
                };
                if this.gen_settings.ui_font == next {
                    return;
                }
                this.gen_settings.ui_font = next;
                this.persist_font_change(cx);
            })
        };
        let dropdown_language = cx.new(move |_cx| {
            Dropdown::new("general-language")
                .placeholder(dory_i18n::t!("settings.general.language.label"))
                .items(Self::language_items())
                .selected_index(Some(language_index))
        });
        let dropdown_default_focus = cx.new(move |_cx| {
            Dropdown::new("general-default-focus")
                .placeholder(dory_i18n::t!("settings.general.default_focus.label"))
                .items(Self::startup_focus_items())
                .selected_index(Some(startup_focus_index))
        });
        let dropdown_refresh_policy = cx.new(move |_cx| {
            Dropdown::new("general-refresh-policy")
                .placeholder(dory_i18n::t!("settings.general.placeholder.refresh_policy"))
                .items(Self::refresh_policy_items())
                .selected_index(Some(refresh_policy_index))
        });

        let input_max_history = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("1000")
                .default_value(max_history.clone())
        });
        let input_auto_save = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("2000")
                .default_value(auto_save_interval.clone())
        });
        let input_refresh_interval = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("5")
                .default_value(refresh_interval.clone())
        });
        let input_max_bg_tasks = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("8")
                .default_value(max_background_tasks.clone())
        });

        let input_object_preview_limit = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("10")
                .default_value(object_preview_limit.clone())
        });

        let theme_mode_subscription = cx.subscribe(
            &dropdown_theme_mode,
            |this, _, event: &DropdownSelectionChanged, cx| {
                this.gen_settings.theme_mode = Self::theme_mode_for_index(event.index);
                this.persist_and_apply(cx);
            },
        );

        let dark_theme_subscription = cx.subscribe(
            &dropdown_dark_theme,
            |this, _, event: &DropdownSelectionChanged, cx| {
                this.gen_settings.dark_theme =
                    Self::palette_for_index(ThemeSetting::dark_themes(), event.index);
                this.persist_and_apply(cx);
            },
        );

        let light_theme_subscription = cx.subscribe(
            &dropdown_light_theme,
            |this, _, event: &DropdownSelectionChanged, cx| {
                this.gen_settings.light_theme =
                    Self::palette_for_index(ThemeSetting::light_themes(), event.index);
                this.persist_and_apply(cx);
            },
        );

        let style_subscription = cx.subscribe(
            &dropdown_style,
            |this, _, event: &DropdownSelectionChanged, cx| {
                this.gen_settings.style = Self::style_for_index(event.index);
                this.persist_and_apply(cx);
            },
        );

        let language_subscription = cx.subscribe(
            &dropdown_language,
            |this, _, event: &DropdownSelectionChanged, cx| {
                this.gen_settings.language = Self::language_for_index(event.index).to_string();
                this.persist_and_apply(cx);
            },
        );

        let focus_subscription = cx.subscribe(
            &dropdown_default_focus,
            |this, _, event: &DropdownSelectionChanged, cx| {
                this.gen_settings.default_focus_on_startup =
                    Self::startup_focus_for_index(event.index);
                this.persist_and_apply(cx);
            },
        );

        let refresh_policy_subscription = cx.subscribe(
            &dropdown_refresh_policy,
            |this, _, event: &DropdownSelectionChanged, cx| {
                this.gen_settings.default_refresh_policy =
                    Self::refresh_policy_for_index(event.index);
                this.persist_and_apply(cx);
            },
        );

        let blur_max_history =
            cx.subscribe(&input_max_history, |this, _, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Blur) {
                    if this.switching_input {
                        this.switching_input = false;
                        return;
                    }
                    this.toast_invalid_numeric_field(cx);
                    this.persist_and_apply(cx);
                    cx.emit(SectionFocusEvent::RequestFocusReturn);
                }
            });

        let blur_auto_save = cx.subscribe(&input_auto_save, |this, _, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Blur) {
                if this.switching_input {
                    this.switching_input = false;
                    return;
                }
                this.toast_invalid_numeric_field(cx);
                this.persist_and_apply(cx);
                cx.emit(SectionFocusEvent::RequestFocusReturn);
            }
        });

        let blur_refresh_interval = cx.subscribe(
            &input_refresh_interval,
            |this, _, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Blur) {
                    if this.switching_input {
                        this.switching_input = false;
                        return;
                    }
                    this.toast_invalid_numeric_field(cx);
                    this.persist_and_apply(cx);
                    cx.emit(SectionFocusEvent::RequestFocusReturn);
                }
            },
        );

        let blur_max_bg_tasks =
            cx.subscribe(&input_max_bg_tasks, |this, _, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Blur) {
                    if this.switching_input {
                        this.switching_input = false;
                        return;
                    }
                    this.toast_invalid_numeric_field(cx);
                    this.persist_and_apply(cx);
                    cx.emit(SectionFocusEvent::RequestFocusReturn);
                }
            });

        let blur_object_preview_limit = cx.subscribe(
            &input_object_preview_limit,
            |this, _, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Blur) {
                    if this.switching_input {
                        this.switching_input = false;
                        return;
                    }
                    this.toast_invalid_numeric_field(cx);
                    this.persist_and_apply(cx);
                    cx.emit(SectionFocusEvent::RequestFocusReturn);
                }
            },
        );

        Self {
            app_state,
            gen_settings: settings,
            gen_form_cursor: 0,
            gen_editing_field: false,
            gen_share_stable_db: dory_storage::paths::nightly_shares_stable_db(),
            dropdown_theme_mode,
            dropdown_dark_theme,
            dropdown_light_theme,
            dropdown_style,
            font_picker,
            dropdown_language,
            dropdown_default_focus,
            dropdown_refresh_policy,
            input_max_history,
            input_auto_save,
            input_refresh_interval,
            input_max_bg_tasks,
            input_object_preview_limit,
            content_focused: false,
            switching_input: false,
            _subscriptions: vec![
                theme_mode_subscription,
                dark_theme_subscription,
                light_theme_subscription,
                style_subscription,
                font_picker_subscription,
                language_subscription,
                focus_subscription,
                refresh_policy_subscription,
                blur_max_history,
                blur_auto_save,
                blur_refresh_interval,
                blur_max_bg_tasks,
                blur_object_preview_limit,
            ],
        }
    }

    fn theme_mode_items() -> Vec<DropdownItem> {
        vec![
            DropdownItem::new(dory_i18n::t!("settings.general.theme_mode.option.system")),
            DropdownItem::new(dory_i18n::t!("settings.general.theme_mode.option.dark")),
            DropdownItem::new(dory_i18n::t!("settings.general.theme_mode.option.light")),
        ]
    }

    fn palette_items(themes: &[ThemeSetting]) -> Vec<DropdownItem> {
        themes
            .iter()
            .map(|theme| DropdownItem::new(theme.label()))
            .collect()
    }

    fn style_items() -> Vec<DropdownItem> {
        vec![
            DropdownItem::new(AppStyle::Default.label()),
            DropdownItem::new(AppStyle::Compact.label()),
        ]
    }

    /// Enumerate installed font families via GPUI's text system. The empty
    /// string is the system-font sentinel.
    fn installed_font_families(cx: &App) -> Vec<String> {
        let mut families = cx.text_system().all_font_names();
        families.retain(|name| FontSetting::is_suitable_ui_family(name));
        families.sort_by_key(|name| name.to_lowercase());
        families.dedup();
        families
    }

    fn language_items() -> Vec<DropdownItem> {
        std::iter::once(DropdownItem::new(dory_i18n::t!(
            "settings.general.language.option.system"
        )))
        .chain(
            dory_i18n::Language::available()
                .iter()
                .map(|language| DropdownItem::new(language.native_name())),
        )
        .collect()
    }

    fn startup_focus_items() -> Vec<DropdownItem> {
        vec![
            DropdownItem::new(dory_i18n::t!(
                "settings.general.default_focus.option.sidebar"
            )),
            DropdownItem::new(dory_i18n::t!(
                "settings.general.default_focus.option.last_tab"
            )),
        ]
    }

    fn refresh_policy_items() -> Vec<DropdownItem> {
        vec![
            DropdownItem::new(dory_i18n::t!(
                "settings.general.refresh_policy.option.manual"
            )),
            DropdownItem::new(dory_i18n::t!(
                "settings.general.refresh_policy.option.interval"
            )),
        ]
    }

    fn theme_mode_index(mode: ThemeModeSetting) -> usize {
        match mode {
            ThemeModeSetting::System => 0,
            ThemeModeSetting::Dark => 1,
            ThemeModeSetting::Light => 2,
        }
    }

    fn theme_mode_for_index(index: usize) -> ThemeModeSetting {
        match index {
            1 => ThemeModeSetting::Dark,
            2 => ThemeModeSetting::Light,
            _ => ThemeModeSetting::System,
        }
    }

    fn palette_index(themes: &[ThemeSetting], theme: ThemeSetting) -> usize {
        themes
            .iter()
            .position(|candidate| *candidate == theme)
            .unwrap_or(0)
    }

    fn palette_for_index(themes: &[ThemeSetting], index: usize) -> ThemeSetting {
        themes.get(index).copied().unwrap_or(themes[0])
    }

    pub(super) fn style_index(style: AppStyle) -> usize {
        match style {
            AppStyle::Default => 0,
            AppStyle::Compact => 1,
        }
    }

    pub(super) fn style_for_index(index: usize) -> AppStyle {
        match index {
            1 => AppStyle::Compact,
            _ => AppStyle::Default,
        }
    }

    fn language_index(persisted: &str) -> usize {
        match dory_i18n::LanguagePreference::from_storage_str(persisted) {
            dory_i18n::LanguagePreference::System => 0,
            dory_i18n::LanguagePreference::Explicit(language) => dory_i18n::Language::available()
                .iter()
                .position(|available| *available == language)
                .map(|position| position + 1)
                .unwrap_or(0),
        }
    }

    fn language_for_index(index: usize) -> &'static str {
        let preference = match index
            .checked_sub(1)
            .and_then(|position| dory_i18n::Language::available().get(position).copied())
        {
            Some(language) => dory_i18n::LanguagePreference::Explicit(language),
            None => dory_i18n::LanguagePreference::System,
        };
        preference.as_storage_str()
    }

    fn startup_focus_index(focus: StartupFocus) -> usize {
        match focus {
            StartupFocus::Sidebar => 0,
            StartupFocus::LastTab => 1,
        }
    }

    fn startup_focus_for_index(index: usize) -> StartupFocus {
        match index {
            1 => StartupFocus::LastTab,
            _ => StartupFocus::Sidebar,
        }
    }

    fn refresh_policy_index(policy: RefreshPolicySetting) -> usize {
        match policy {
            RefreshPolicySetting::Manual => 0,
            RefreshPolicySetting::Interval => 1,
        }
    }

    fn refresh_policy_for_index(index: usize) -> RefreshPolicySetting {
        match index {
            1 => RefreshPolicySetting::Interval,
            _ => RefreshPolicySetting::Manual,
        }
    }
}

impl SettingsSection for GeneralSection {
    fn section_id(&self) -> SettingsSectionId {
        SettingsSectionId::General
    }

    fn handle_key_event(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        GeneralSection::handle_key_event(self, event, window, cx);
    }

    fn focus_in(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.content_focused = true;
        cx.notify();
    }

    fn focus_out(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.content_focused = false;
        self.gen_editing_field = false;
        self.close_open_dropdown(cx);
        cx.notify();
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.has_unsaved_general_changes(cx)
    }
}

impl Render for GeneralSection {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_general_section(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::GeneralSection;
    use dory_core::{AppStyle, ThemeModeSetting, ThemeSetting};

    #[test]
    fn dark_and_light_dropdowns_expose_polarity_palettes() {
        let dark: Vec<_> = GeneralSection::palette_items(ThemeSetting::dark_themes())
            .into_iter()
            .map(|item| item.label)
            .collect();
        let light: Vec<_> = GeneralSection::palette_items(ThemeSetting::light_themes())
            .into_iter()
            .map(|item| item.label)
            .collect();

        assert_eq!(
            dark,
            vec!["Dory Dark", "Ayu Dark", "Ayu Mirage", "Nord", "Dracula"]
        );
        assert_eq!(
            light,
            vec![
                "Dory Light",
                "Ayu Light",
                "Catppuccin Latte",
                "GitHub Light",
                "One Light",
            ]
        );
    }

    #[test]
    fn palette_index_and_reverse_mapping_cover_each_polarity() {
        for theme in ThemeSetting::dark_themes() {
            let index = GeneralSection::palette_index(ThemeSetting::dark_themes(), *theme);
            assert_eq!(
                GeneralSection::palette_for_index(ThemeSetting::dark_themes(), index),
                *theme
            );
        }
        for theme in ThemeSetting::light_themes() {
            let index = GeneralSection::palette_index(ThemeSetting::light_themes(), *theme);
            assert_eq!(
                GeneralSection::palette_for_index(ThemeSetting::light_themes(), index),
                *theme
            );
        }
        assert_eq!(
            GeneralSection::palette_for_index(ThemeSetting::dark_themes(), 99),
            ThemeSetting::DoryDark
        );
        assert_eq!(
            GeneralSection::theme_mode_index(ThemeModeSetting::System),
            0
        );
        assert_eq!(
            GeneralSection::theme_mode_for_index(1),
            ThemeModeSetting::Dark
        );
        assert_eq!(
            GeneralSection::theme_mode_for_index(2),
            ThemeModeSetting::Light
        );
        assert_eq!(
            GeneralSection::theme_mode_for_index(99),
            ThemeModeSetting::System
        );
    }

    #[test]
    fn style_dropdown_exposes_exactly_two_labels() {
        let labels: Vec<_> = GeneralSection::style_items()
            .into_iter()
            .map(|item| item.label)
            .collect();

        assert_eq!(labels, vec!["Default", "Compact"]);
    }

    #[test]
    fn style_index_and_reverse_mapping_cover_all_variants() {
        assert_eq!(GeneralSection::style_index(AppStyle::Default), 0);
        assert_eq!(GeneralSection::style_index(AppStyle::Compact), 1);

        assert_eq!(GeneralSection::style_for_index(0), AppStyle::Default);
        assert_eq!(GeneralSection::style_for_index(1), AppStyle::Compact);
        // Out-of-range falls back to Default
        assert_eq!(GeneralSection::style_for_index(99), AppStyle::Default);
    }

    #[test]
    fn language_dropdown_orders_system_then_english_then_deterministic_remainder() {
        let labels: Vec<_> = GeneralSection::language_items()
            .into_iter()
            .map(|item| item.label)
            .collect();
        let available = dory_i18n::Language::available();

        assert_eq!(labels.len(), available.len() + 1);
        assert_eq!(labels.first().map(|label| label.as_ref()), Some("System"));
        assert_eq!(labels.get(1).map(|label| label.as_ref()), Some("English"));

        let storage_ids: Vec<_> = available
            .iter()
            .skip(1)
            .map(|language| language.as_storage_str())
            .collect();
        let mut sorted_storage_ids = storage_ids.clone();
        sorted_storage_ids.sort_unstable();
        assert_eq!(storage_ids, sorted_storage_ids);

        for (label, language) in labels.iter().skip(1).zip(available) {
            assert_eq!(label, &language.native_name());
        }
    }

    #[test]
    fn language_index_and_reverse_mapping_round_trip_every_available_locale() {
        assert_eq!(GeneralSection::language_index(""), 0);
        assert_eq!(GeneralSection::language_for_index(0), "");

        let available = dory_i18n::Language::available();
        for (position, language) in available.iter().enumerate() {
            let index = position + 1;
            let storage_id = language.as_storage_str();
            assert_eq!(GeneralSection::language_index(storage_id), index);
            assert_eq!(GeneralSection::language_for_index(index), storage_id);
        }

        assert_eq!(GeneralSection::language_index("de"), 0);
        assert_eq!(GeneralSection::language_for_index(available.len() + 1), "");
    }

    #[test]
    fn dropdown_placeholders_reuse_or_extend_settings_general_catalog_keys() {
        assert_eq!(
            dory_i18n::t!("settings.general.theme_mode.label"),
            "Theme mode"
        );
        assert_eq!(
            dory_i18n::t!("settings.general.dark_theme.label"),
            "Dark theme"
        );
        assert_eq!(
            dory_i18n::t!("settings.general.light_theme.label"),
            "Light theme"
        );
        assert_eq!(dory_i18n::t!("settings.general.style.label"), "Style");
        assert_eq!(dory_i18n::t!("settings.general.font.label"), "UI Font");
        assert_eq!(dory_i18n::t!("settings.general.language.label"), "Language");
        assert_eq!(
            dory_i18n::t!("settings.general.default_focus.label"),
            "Default focus"
        );
        assert_eq!(
            dory_i18n::t!("settings.general.placeholder.refresh_policy"),
            "Refresh policy"
        );

        for locale in ["en", "es"] {
            let value = dory_i18n::t!(
                "settings.general.placeholder.refresh_policy",
                locale = locale
            );

            assert!(
                !value.is_empty(),
                "settings.general.placeholder.refresh_policy resolved empty for locale {locale}"
            );
            assert_ne!(
                value,
                format!("{locale}.settings.general.placeholder.refresh_policy"),
                "settings.general.placeholder.refresh_policy fell back to the raw key for locale {locale}"
            );
        }
    }
}
