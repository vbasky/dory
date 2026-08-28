use dory_app::keymap::{KeyChord, Modifiers};
use dory_components::controls::Dropdown;
use dory_components::controls::FontPicker;
use dory_components::controls::{GpuiInput as Input, InputState};
use dory_components::tokens::Radii;
use dory_components::typography::{Body, Caption, FieldLabel};
use dory_ui_base::keymap::key_chord_from_gpui;
use dory_ui_base::toast::{Toast, copy_action, now_hms};
use dory_ui_base::user_error::{ErrorKind, UserFacingError, report_error};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::ActiveTheme;
use gpui_component::Sizable;
use gpui_component::checkbox::Checkbox;

use super::general_section::{GeneralFormRow, GeneralSection};
use super::layout;
use super::section_trait::SectionFocusEvent;

impl GeneralSection {
    pub(super) fn has_unsaved_general_changes(&self, _cx: &App) -> bool {
        false
    }

    pub(super) fn gen_form_rows(&self) -> Vec<GeneralFormRow> {
        let mut rows = vec![
            GeneralFormRow::ThemeMode,
            GeneralFormRow::DarkTheme,
            GeneralFormRow::LightTheme,
            GeneralFormRow::Style,
            GeneralFormRow::Font,
            GeneralFormRow::Language,
            GeneralFormRow::RestoreSession,
            GeneralFormRow::ReopenConnections,
            GeneralFormRow::DefaultFocus,
            GeneralFormRow::MaxHistory,
            GeneralFormRow::AutoSaveInterval,
            GeneralFormRow::DefaultRefreshPolicy,
            GeneralFormRow::DefaultRefreshInterval,
            GeneralFormRow::MaxBackgroundTasks,
            GeneralFormRow::PauseRefreshOnError,
            GeneralFormRow::RefreshOnlyIfVisible,
            GeneralFormRow::ConfirmDangerous,
            GeneralFormRow::RequiresWhere,
            GeneralFormRow::RequiresPreview,
            GeneralFormRow::ObjectPreviewLimit,
        ];

        // The shared-database toggle only makes sense on nightly, which is the
        // only channel that uses a separate database by default.
        if Self::is_nightly() {
            rows.push(GeneralFormRow::ShareStableDb);
        }

        rows
    }

    fn is_nightly() -> bool {
        dory_core::ReleaseChannel::current() == dory_core::ReleaseChannel::Nightly
    }

    /// Toggles whether this nightly build shares the stable database. The change
    /// is persisted to the pre-database marker immediately and applies on the
    /// next launch; a write failure is surfaced to the user and leaves the toggle
    /// unchanged.
    fn set_share_stable_db(&mut self, value: bool, cx: &mut Context<Self>) {
        match dory_storage::paths::set_nightly_shares_stable_db(value) {
            Ok(()) => self.gen_share_stable_db = value,
            Err(error) => {
                report_error(
                    UserFacingError::new(
                        ErrorKind::Config,
                        dory_i18n::t!("settings.general.share_stable_db.error"),
                    )
                    .with_cause(format!("{error}")),
                    cx,
                );
            }
        }
    }

    fn gen_current_row(&self) -> Option<GeneralFormRow> {
        self.gen_form_rows().get(self.gen_form_cursor).copied()
    }

    pub(super) fn gen_move_down(&mut self) {
        let count = self.gen_form_rows().len();
        if self.gen_form_cursor + 1 < count {
            self.gen_form_cursor += 1;
        }
    }

    pub(super) fn gen_move_up(&mut self) {
        if self.gen_form_cursor > 0 {
            self.gen_form_cursor -= 1;
        }
    }

    fn gen_move_first(&mut self) {
        self.gen_form_cursor = 0;
    }

    fn gen_move_last(&mut self) {
        self.gen_form_cursor = self.gen_form_rows().len().saturating_sub(1);
    }

    pub(super) fn gen_activate_current_field(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.gen_current_row() {
            Some(GeneralFormRow::ThemeMode) => {
                self.dropdown_theme_mode
                    .update(cx, |dropdown, cx| dropdown.toggle_open(cx));
                cx.notify();
            }
            Some(GeneralFormRow::DarkTheme) => {
                self.dropdown_dark_theme
                    .update(cx, |dropdown, cx| dropdown.toggle_open(cx));
                cx.notify();
            }
            Some(GeneralFormRow::LightTheme) => {
                self.dropdown_light_theme
                    .update(cx, |dropdown, cx| dropdown.toggle_open(cx));
                cx.notify();
            }
            Some(GeneralFormRow::Style) => {
                self.dropdown_style
                    .update(cx, |dropdown, cx| dropdown.toggle_open(cx));
                cx.notify();
            }
            Some(GeneralFormRow::Font) => {
                self.font_picker
                    .update(cx, |picker, cx| picker.toggle_open(window, cx));
                cx.notify();
            }
            Some(GeneralFormRow::Language) => {
                self.dropdown_language
                    .update(cx, |dropdown, cx| dropdown.toggle_open(cx));
                cx.notify();
            }
            Some(GeneralFormRow::RestoreSession) => {
                self.gen_settings.restore_session_on_startup =
                    !self.gen_settings.restore_session_on_startup;
                self.persist_and_apply(cx);
            }
            Some(GeneralFormRow::ReopenConnections) => {
                self.gen_settings.reopen_last_connections =
                    !self.gen_settings.reopen_last_connections;
                self.persist_and_apply(cx);
            }
            Some(GeneralFormRow::DefaultFocus) => {
                self.dropdown_default_focus
                    .update(cx, |dropdown, cx| dropdown.toggle_open(cx));
                cx.notify();
            }
            Some(GeneralFormRow::DefaultRefreshPolicy) => {
                self.dropdown_refresh_policy
                    .update(cx, |dropdown, cx| dropdown.toggle_open(cx));
                cx.notify();
            }
            Some(GeneralFormRow::PauseRefreshOnError) => {
                self.gen_settings.auto_refresh_pause_on_error =
                    !self.gen_settings.auto_refresh_pause_on_error;
                self.persist_and_apply(cx);
            }
            Some(GeneralFormRow::RefreshOnlyIfVisible) => {
                self.gen_settings.auto_refresh_only_if_visible =
                    !self.gen_settings.auto_refresh_only_if_visible;
                self.persist_and_apply(cx);
            }
            Some(GeneralFormRow::ConfirmDangerous) => {
                self.gen_settings.confirm_dangerous_queries =
                    !self.gen_settings.confirm_dangerous_queries;
                self.persist_and_apply(cx);
            }
            Some(GeneralFormRow::RequiresWhere) => {
                self.gen_settings.dangerous_requires_where =
                    !self.gen_settings.dangerous_requires_where;
                self.persist_and_apply(cx);
            }
            Some(GeneralFormRow::RequiresPreview) => {
                self.gen_settings.dangerous_requires_preview =
                    !self.gen_settings.dangerous_requires_preview;
                self.persist_and_apply(cx);
            }
            Some(GeneralFormRow::ShareStableDb) => {
                self.set_share_stable_db(!self.gen_share_stable_db, cx);
                cx.notify();
            }
            Some(GeneralFormRow::MaxHistory)
            | Some(GeneralFormRow::AutoSaveInterval)
            | Some(GeneralFormRow::DefaultRefreshInterval)
            | Some(GeneralFormRow::MaxBackgroundTasks)
            | Some(GeneralFormRow::ObjectPreviewLimit) => {
                self.gen_focus_current_input(window, cx);
            }
            None => {}
        }
    }

    fn gen_focus_current_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.gen_editing_field = true;

        match self.gen_current_row() {
            Some(GeneralFormRow::MaxHistory) => {
                self.input_max_history
                    .update(cx, |state, cx| state.focus(window, cx));
            }
            Some(GeneralFormRow::AutoSaveInterval) => {
                self.input_auto_save
                    .update(cx, |state, cx| state.focus(window, cx));
            }
            Some(GeneralFormRow::DefaultRefreshInterval) => {
                self.input_refresh_interval
                    .update(cx, |state, cx| state.focus(window, cx));
            }
            Some(GeneralFormRow::MaxBackgroundTasks) => {
                self.input_max_bg_tasks
                    .update(cx, |state, cx| state.focus(window, cx));
            }
            Some(GeneralFormRow::ObjectPreviewLimit) => {
                self.input_object_preview_limit
                    .update(cx, |state, cx| state.focus(window, cx));
            }
            _ => {
                self.gen_editing_field = false;
            }
        }
    }

    pub(super) fn close_open_dropdown(&mut self, cx: &mut Context<Self>) {
        if let Some(dropdown) = self.current_dropdown() {
            dropdown.update(cx, |dropdown, cx| {
                if dropdown.is_open() {
                    dropdown.close(cx);
                }
            });
        }
    }

    fn current_dropdown(&self) -> Option<&Entity<Dropdown>> {
        match self.gen_current_row() {
            Some(GeneralFormRow::ThemeMode) => Some(&self.dropdown_theme_mode),
            Some(GeneralFormRow::DarkTheme) => Some(&self.dropdown_dark_theme),
            Some(GeneralFormRow::LightTheme) => Some(&self.dropdown_light_theme),
            Some(GeneralFormRow::Style) => Some(&self.dropdown_style),
            // The font row uses the searchable FontPicker, not a Dropdown.
            Some(GeneralFormRow::Font) => None,
            Some(GeneralFormRow::Language) => Some(&self.dropdown_language),
            Some(GeneralFormRow::DefaultFocus) => Some(&self.dropdown_default_focus),
            Some(GeneralFormRow::DefaultRefreshPolicy) => Some(&self.dropdown_refresh_policy),
            _ => None,
        }
    }

    fn handle_open_dropdown(
        &mut self,
        chord: &KeyChord,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(dropdown_entity) = self.current_dropdown().cloned() else {
            return false;
        };

        if !dropdown_entity.read(cx).is_open() {
            return false;
        }

        match (chord.key.as_str(), chord.modifiers) {
            ("j", modifiers) | ("down", modifiers) if modifiers == Modifiers::none() => {
                dropdown_entity.update(cx, |dropdown, cx| dropdown.select_next_item(cx));
            }
            ("k", modifiers) | ("up", modifiers) if modifiers == Modifiers::none() => {
                dropdown_entity.update(cx, |dropdown, cx| dropdown.select_prev_item(cx));
            }
            ("enter", modifiers) if modifiers == Modifiers::none() => {
                dropdown_entity.update(cx, |dropdown, cx| dropdown.accept_selection(cx));
            }
            ("escape", modifiers) if modifiers == Modifiers::none() => {
                dropdown_entity.update(cx, |dropdown, cx| dropdown.close(cx));
            }
            ("tab", modifiers) if modifiers == Modifiers::none() => {
                dropdown_entity.update(cx, |dropdown, cx| dropdown.accept_selection(cx));
                self.gen_move_down();
                self.gen_focus_current_input(window, cx);
            }
            ("tab", modifiers) if modifiers == Modifiers::shift() => {
                dropdown_entity.update(cx, |dropdown, cx| dropdown.accept_selection(cx));
                self.gen_move_up();
                self.gen_focus_current_input(window, cx);
            }
            _ => return false,
        }

        cx.notify();
        true
    }

    pub(super) fn handle_key_event(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let chord = key_chord_from_gpui(&event.keystroke);

        if self.gen_editing_field {
            match (chord.key.as_str(), chord.modifiers) {
                ("escape", modifiers) if modifiers == Modifiers::none() => {
                    self.gen_editing_field = false;
                    cx.emit(SectionFocusEvent::RequestFocusReturn);
                    cx.notify();
                }
                ("enter", modifiers) if modifiers == Modifiers::none() => {
                    self.gen_editing_field = false;
                    self.persist_and_apply(cx);
                    self.gen_move_down();
                    cx.notify();
                }
                ("tab", modifiers) if modifiers == Modifiers::none() => {
                    self.gen_editing_field = false;
                    self.gen_move_down();
                    self.gen_focus_current_input(window, cx);
                    cx.notify();
                }
                ("tab", modifiers) if modifiers == Modifiers::shift() => {
                    self.gen_editing_field = false;
                    self.gen_move_up();
                    self.gen_focus_current_input(window, cx);
                    cx.notify();
                }
                _ => {}
            }

            return;
        }

        if self.handle_open_dropdown(&chord, window, cx) {
            return;
        }

        match (chord.key.as_str(), chord.modifiers) {
            ("j", modifiers) | ("down", modifiers) if modifiers == Modifiers::none() => {
                self.gen_move_down();
                cx.notify();
            }
            ("k", modifiers) | ("up", modifiers) if modifiers == Modifiers::none() => {
                self.gen_move_up();
                cx.notify();
            }
            ("l", modifiers) | ("right", modifiers) | ("enter", modifiers)
                if modifiers == Modifiers::none() =>
            {
                self.gen_activate_current_field(window, cx);
            }
            ("tab", modifiers) if modifiers == Modifiers::none() => {
                self.gen_move_down();
                cx.notify();
            }
            ("tab", modifiers) if modifiers == Modifiers::shift() => {
                self.gen_move_up();
                cx.notify();
            }
            ("g", modifiers) if modifiers == Modifiers::none() => {
                self.gen_move_first();
                cx.notify();
            }
            ("G", modifiers) if modifiers == Modifiers::none() => {
                self.gen_move_last();
                cx.notify();
            }
            _ => {}
        }
    }

    fn sync_valid_numeric_fields(&mut self, cx: &App) {
        if let Ok(value) = self
            .input_max_history
            .read(cx)
            .value()
            .trim()
            .parse::<usize>()
        {
            if value >= 10 {
                self.gen_settings.max_history_entries = value;
            }
        }
        if let Ok(value) = self.input_auto_save.read(cx).value().trim().parse::<u64>() {
            if value >= 500 {
                self.gen_settings.auto_save_interval_ms = value;
            }
        }
        if let Ok(value) = self
            .input_refresh_interval
            .read(cx)
            .value()
            .trim()
            .parse::<u32>()
        {
            if value >= 1 {
                self.gen_settings.default_refresh_interval_secs = value;
            }
        }
        if let Ok(value) = self
            .input_max_bg_tasks
            .read(cx)
            .value()
            .trim()
            .parse::<usize>()
        {
            if value >= 1 {
                self.gen_settings.max_concurrent_background_tasks = value;
            }
        }
        if let Ok(value) = self
            .input_object_preview_limit
            .read(cx)
            .value()
            .trim()
            .parse::<u64>()
        {
            if value >= 1 {
                self.gen_settings.object_preview_size_limit_mib = value;
            }
        }
    }

    pub(super) fn toast_invalid_numeric_field(&self, cx: &mut Context<Self>) {
        let message = match self.gen_current_row() {
            Some(GeneralFormRow::MaxHistory)
                if self
                    .input_max_history
                    .read(cx)
                    .value()
                    .trim()
                    .parse::<usize>()
                    .ok()
                    .filter(|value| *value >= 10)
                    .is_none() =>
            {
                Some(dory_i18n::t!("settings.general.max_history.error"))
            }
            Some(GeneralFormRow::AutoSaveInterval)
                if self
                    .input_auto_save
                    .read(cx)
                    .value()
                    .trim()
                    .parse::<u64>()
                    .ok()
                    .filter(|value| *value >= 500)
                    .is_none() =>
            {
                Some(dory_i18n::t!("settings.general.auto_save_interval.error"))
            }
            Some(GeneralFormRow::DefaultRefreshInterval)
                if self
                    .input_refresh_interval
                    .read(cx)
                    .value()
                    .trim()
                    .parse::<u32>()
                    .ok()
                    .filter(|value| *value >= 1)
                    .is_none() =>
            {
                Some(dory_i18n::t!("settings.general.refresh_interval.error"))
            }
            Some(GeneralFormRow::MaxBackgroundTasks)
                if self
                    .input_max_bg_tasks
                    .read(cx)
                    .value()
                    .trim()
                    .parse::<usize>()
                    .ok()
                    .filter(|value| *value >= 1)
                    .is_none() =>
            {
                Some(dory_i18n::t!("settings.general.max_background_tasks.error"))
            }
            Some(GeneralFormRow::ObjectPreviewLimit)
                if self
                    .input_object_preview_limit
                    .read(cx)
                    .value()
                    .trim()
                    .parse::<u64>()
                    .ok()
                    .filter(|value| *value >= 1)
                    .is_none() =>
            {
                Some(dory_i18n::t!("settings.general.object_preview_limit.error"))
            }
            _ => None,
        };

        if let Some(message) = message {
            Toast::error(message.clone())
                .meta_right(now_hms())
                .action(copy_action(message))
                .push(cx);
        }
    }

    pub(super) fn persist_and_apply(&mut self, cx: &mut Context<Self>) {
        self.sync_valid_numeric_fields(cx);
        self.gen_settings.ui_font = self.gen_settings.ui_font.clone().sanitize_for_ui();
        let appearance_is_dark = dory_components::theme::appearance_is_dark(cx.window_appearance());
        self.gen_settings.theme = self.gen_settings.resolved_theme(appearance_is_dark);

        let runtime = self.app_state.read(cx).storage_runtime();
        if let Err(e) = dory_app::config_loader::save_general_settings(runtime, &self.gen_settings)
        {
            report_error(
                UserFacingError::new(
                    ErrorKind::Storage,
                    dory_i18n::t!("settings.general.save.error", error = e),
                ),
                cx,
            );
            return;
        }

        self.app_state.update(cx, |state, _cx| {
            state.update_general_settings(self.gen_settings.clone());
        });

        dory_components::density::set_style(cx, self.gen_settings.style);
        dory_components::semantic::ThemeSettingGlobal::set(cx, self.gen_settings.theme);
        dory_components::semantic::FontSettingGlobal::set(cx, self.gen_settings.ui_font.clone());
        dory_components::theme::apply_theme(
            self.gen_settings.theme,
            self.gen_settings.style,
            None,
            cx,
        );
        cx.notify();
    }

    /// Persist a UI-font change without rebuilding the color palette.
    pub(super) fn persist_font_change(&mut self, cx: &mut Context<Self>) {
        self.gen_settings.ui_font = self.gen_settings.ui_font.clone().sanitize_for_ui();
        let runtime = self.app_state.read(cx).storage_runtime();
        if let Err(e) = dory_app::config_loader::save_general_settings(runtime, &self.gen_settings)
        {
            report_error(
                UserFacingError::new(
                    ErrorKind::Storage,
                    dory_i18n::t!("settings.general.save.error", error = e),
                ),
                cx,
            );
            return;
        }

        self.app_state.update(cx, |state, _cx| {
            state.update_general_settings(self.gen_settings.clone());
        });
        dory_components::semantic::FontSettingGlobal::set(cx, self.gen_settings.ui_font.clone());
        dory_components::theme::apply_ui_font(cx);
        cx.notify();
    }

    pub(super) fn render_general_section(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let muted_fg = cx.theme().muted_foreground;
        let border = cx.theme().border;
        let is_focused = self.content_focused;
        let cursor = self.gen_form_cursor;
        let rows = self.gen_form_rows();

        let is_at =
            |row: GeneralFormRow| -> bool { is_focused && rows.get(cursor).copied() == Some(row) };

        layout::single_form_section_shell(
            dory_components::composites::section_header(
                dory_i18n::t!("settings.general.header.title"),
                dory_i18n::t!("settings.general.header.subtitle"),
                cx,
            ),
            div()
                .flex()
                .flex_col()
                .child(self.render_gen_section_header(
                    dory_i18n::t!("settings.general.appearance.group"),
                    muted_fg,
                    border,
                ))
                .child(self.render_gen_dropdown(
                    dory_i18n::t!("settings.general.theme_mode.label"),
                    dory_i18n::t!("settings.general.theme_mode.description"),
                    &self.dropdown_theme_mode,
                    is_at(GeneralFormRow::ThemeMode),
                    GeneralFormRow::ThemeMode,
                    cx,
                ))
                .child(self.render_gen_dropdown(
                    dory_i18n::t!("settings.general.dark_theme.label"),
                    dory_i18n::t!("settings.general.dark_theme.description"),
                    &self.dropdown_dark_theme,
                    is_at(GeneralFormRow::DarkTheme),
                    GeneralFormRow::DarkTheme,
                    cx,
                ))
                .child(self.render_gen_dropdown(
                    dory_i18n::t!("settings.general.light_theme.label"),
                    dory_i18n::t!("settings.general.light_theme.description"),
                    &self.dropdown_light_theme,
                    is_at(GeneralFormRow::LightTheme),
                    GeneralFormRow::LightTheme,
                    cx,
                ))
                .child(self.render_gen_dropdown(
                    dory_i18n::t!("settings.general.style.label"),
                    dory_i18n::t!("settings.general.style.description"),
                    &self.dropdown_style,
                    is_at(GeneralFormRow::Style),
                    GeneralFormRow::Style,
                    cx,
                ))
                .child(self.render_gen_font_row(
                    dory_i18n::t!("settings.general.font.label"),
                    dory_i18n::t!("settings.general.font.description"),
                    &self.font_picker,
                    is_at(GeneralFormRow::Font),
                    GeneralFormRow::Font,
                    cx,
                ))
                .child(self.render_gen_dropdown(
                    dory_i18n::t!("settings.general.language.label"),
                    dory_i18n::t!("settings.general.language.description"),
                    &self.dropdown_language,
                    is_at(GeneralFormRow::Language),
                    GeneralFormRow::Language,
                    cx,
                ))
                .child(self.render_gen_section_header(
                    dory_i18n::t!("settings.general.startup.group"),
                    muted_fg,
                    border,
                ))
                .child(self.render_gen_checkbox(
                    "restore-session",
                    dory_i18n::t!("settings.general.restore_session.label"),
                    dory_i18n::t!("settings.general.restore_session.description"),
                    self.gen_settings.restore_session_on_startup,
                    is_at(GeneralFormRow::RestoreSession),
                    GeneralFormRow::RestoreSession,
                    |this, value, _cx| this.gen_settings.restore_session_on_startup = value,
                    cx,
                ))
                .child(self.render_gen_checkbox(
                    "reopen-conns",
                    dory_i18n::t!("settings.general.reopen_connections.label"),
                    dory_i18n::t!("settings.general.reopen_connections.description"),
                    self.gen_settings.reopen_last_connections,
                    is_at(GeneralFormRow::ReopenConnections),
                    GeneralFormRow::ReopenConnections,
                    |this, value, _cx| this.gen_settings.reopen_last_connections = value,
                    cx,
                ))
                .child(self.render_gen_dropdown(
                    dory_i18n::t!("settings.general.default_focus.label"),
                    dory_i18n::t!("settings.general.default_focus.description"),
                    &self.dropdown_default_focus,
                    is_at(GeneralFormRow::DefaultFocus),
                    GeneralFormRow::DefaultFocus,
                    cx,
                ))
                .child(self.render_gen_input_field(
                    dory_i18n::t!("settings.general.max_history.label"),
                    dory_i18n::t!("settings.general.max_history.description"),
                    &self.input_max_history,
                    is_at(GeneralFormRow::MaxHistory),
                    GeneralFormRow::MaxHistory,
                    cx,
                ))
                .child(self.render_gen_input_field(
                    dory_i18n::t!("settings.general.auto_save_interval.label"),
                    dory_i18n::t!("settings.general.auto_save_interval.description"),
                    &self.input_auto_save,
                    is_at(GeneralFormRow::AutoSaveInterval),
                    GeneralFormRow::AutoSaveInterval,
                    cx,
                ))
                .child(self.render_gen_section_header(
                    dory_i18n::t!("settings.general.refresh.group"),
                    muted_fg,
                    border,
                ))
                .child(self.render_gen_dropdown(
                    dory_i18n::t!("settings.general.refresh_policy.label"),
                    dory_i18n::t!("settings.general.refresh_policy.description"),
                    &self.dropdown_refresh_policy,
                    is_at(GeneralFormRow::DefaultRefreshPolicy),
                    GeneralFormRow::DefaultRefreshPolicy,
                    cx,
                ))
                .child(self.render_gen_input_field(
                    dory_i18n::t!("settings.general.refresh_interval.label"),
                    dory_i18n::t!("settings.general.refresh_interval.description"),
                    &self.input_refresh_interval,
                    is_at(GeneralFormRow::DefaultRefreshInterval),
                    GeneralFormRow::DefaultRefreshInterval,
                    cx,
                ))
                .child(self.render_gen_input_field(
                    dory_i18n::t!("settings.general.max_background_tasks.label"),
                    dory_i18n::t!("settings.general.max_background_tasks.description"),
                    &self.input_max_bg_tasks,
                    is_at(GeneralFormRow::MaxBackgroundTasks),
                    GeneralFormRow::MaxBackgroundTasks,
                    cx,
                ))
                .child(self.render_gen_checkbox(
                    "pause-on-error",
                    dory_i18n::t!("settings.general.pause_refresh_on_error.label"),
                    dory_i18n::t!("settings.general.pause_refresh_on_error.description"),
                    self.gen_settings.auto_refresh_pause_on_error,
                    is_at(GeneralFormRow::PauseRefreshOnError),
                    GeneralFormRow::PauseRefreshOnError,
                    |this, value, _cx| this.gen_settings.auto_refresh_pause_on_error = value,
                    cx,
                ))
                .child(self.render_gen_checkbox(
                    "refresh-visible",
                    dory_i18n::t!("settings.general.refresh_only_if_visible.label"),
                    dory_i18n::t!("settings.general.refresh_only_if_visible.description"),
                    self.gen_settings.auto_refresh_only_if_visible,
                    is_at(GeneralFormRow::RefreshOnlyIfVisible),
                    GeneralFormRow::RefreshOnlyIfVisible,
                    |this, value, _cx| this.gen_settings.auto_refresh_only_if_visible = value,
                    cx,
                ))
                .child(self.render_gen_section_header(
                    dory_i18n::t!("settings.general.safety.group"),
                    muted_fg,
                    border,
                ))
                .child(self.render_gen_checkbox(
                    "confirm-dangerous",
                    dory_i18n::t!("settings.general.confirm_dangerous.label"),
                    dory_i18n::t!("settings.general.confirm_dangerous.description"),
                    self.gen_settings.confirm_dangerous_queries,
                    is_at(GeneralFormRow::ConfirmDangerous),
                    GeneralFormRow::ConfirmDangerous,
                    |this, value, _cx| this.gen_settings.confirm_dangerous_queries = value,
                    cx,
                ))
                .child(self.render_gen_checkbox(
                    "requires-where",
                    dory_i18n::t!("settings.general.requires_where.label"),
                    dory_i18n::t!("settings.general.requires_where.description"),
                    self.gen_settings.dangerous_requires_where,
                    is_at(GeneralFormRow::RequiresWhere),
                    GeneralFormRow::RequiresWhere,
                    |this, value, _cx| this.gen_settings.dangerous_requires_where = value,
                    cx,
                ))
                .child(self.render_gen_checkbox(
                    "requires-preview",
                    dory_i18n::t!("settings.general.requires_preview.label"),
                    dory_i18n::t!("settings.general.requires_preview.description"),
                    self.gen_settings.dangerous_requires_preview,
                    is_at(GeneralFormRow::RequiresPreview),
                    GeneralFormRow::RequiresPreview,
                    |this, value, _cx| this.gen_settings.dangerous_requires_preview = value,
                    cx,
                ))
                .child(self.render_gen_section_header(
                    dory_i18n::t!("settings.general.object_storage.group"),
                    muted_fg,
                    border,
                ))
                .child(self.render_gen_input_field(
                    dory_i18n::t!("settings.general.object_preview_limit.label"),
                    dory_i18n::t!("settings.general.object_preview_limit.description"),
                    &self.input_object_preview_limit,
                    is_at(GeneralFormRow::ObjectPreviewLimit),
                    GeneralFormRow::ObjectPreviewLimit,
                    cx,
                ))
                .when(Self::is_nightly(), |column| {
                    column
                        .child(self.render_gen_section_header(
                            dory_i18n::t!("settings.general.storage.group"),
                            muted_fg,
                            border,
                        ))
                        .child(self.render_gen_checkbox(
                            "share-stable-db",
                            dory_i18n::t!("settings.general.share_stable_db.label"),
                            dory_i18n::t!("settings.general.share_stable_db.description"),
                            self.gen_share_stable_db,
                            is_at(GeneralFormRow::ShareStableDb),
                            GeneralFormRow::ShareStableDb,
                            |this, value, cx| this.set_share_stable_db(value, cx),
                            cx,
                        ))
                }),
        )
    }

    /// Shared width for every General control so dropdowns, the font picker,
    /// and checkboxes share the same left and right edges.
    const CONTROL_COLUMN_WIDTH: Pixels = px(240.0);

    fn render_gen_section_header(
        &self,
        label: impl Into<SharedString>,
        muted_fg: Hsla,
        border: Hsla,
    ) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap_1()
            .pt_4()
            .pb_1()
            .child(Body::new(label).color(muted_fg))
            .child(div().w_full().h_px().bg(border))
    }

    fn render_setting_row(
        &self,
        title: impl Into<SharedString>,
        description: impl Into<SharedString>,
        is_focused: bool,
        row: GeneralFormRow,
        control: impl IntoElement,
        cx: &mut Context<Self>,
    ) -> Div {
        let muted_fg = cx.theme().muted_foreground;
        let accent = cx.theme().accent;

        div()
            .w_full()
            .flex()
            .items_start()
            .justify_between()
            .gap_6()
            .px_2()
            .py_3()
            .rounded(Radii::SM)
            .bg(if is_focused {
                accent.opacity(0.12)
            } else {
                gpui::transparent_black()
            })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.content_focused = true;
                    if let Some(position) = this
                        .gen_form_rows()
                        .iter()
                        .position(|candidate| *candidate == row)
                    {
                        this.gen_form_cursor = position;
                    }
                    cx.notify();
                }),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(FieldLabel::new(title))
                    .child(Caption::new(description).color(muted_fg)),
            )
            .child(
                div()
                    .w(Self::CONTROL_COLUMN_WIDTH)
                    .min_w(Self::CONTROL_COLUMN_WIDTH)
                    .flex_shrink_0()
                    .min_w_0()
                    .child(control),
            )
    }

    #[allow(clippy::too_many_arguments)]
    fn render_gen_checkbox(
        &self,
        id: &'static str,
        label: impl Into<SharedString>,
        description: impl Into<SharedString>,
        checked: bool,
        is_focused: bool,
        row: GeneralFormRow,
        setter: fn(&mut Self, bool, &mut Context<Self>),
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        self.render_setting_row(
            label,
            description,
            is_focused,
            row,
            div()
                .w_full()
                .flex()
                .justify_end()
                .child(Checkbox::new(id).checked(checked).on_click(cx.listener(
                    move |this, value: &bool, _, cx| {
                        setter(this, *value, cx);
                        if row != GeneralFormRow::ShareStableDb {
                            this.persist_and_apply(cx);
                        }
                    },
                ))),
            cx,
        )
    }

    fn render_gen_dropdown(
        &self,
        label: impl Into<SharedString>,
        description: impl Into<SharedString>,
        dropdown: &Entity<Dropdown>,
        is_focused: bool,
        row: GeneralFormRow,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        self.render_setting_row(
            label,
            description,
            is_focused,
            row,
            div().w_full().min_w_0().child(dropdown.clone()),
            cx,
        )
    }

    fn render_gen_font_row(
        &self,
        label: impl Into<SharedString>,
        description: impl Into<SharedString>,
        picker: &Entity<FontPicker>,
        is_focused: bool,
        row: GeneralFormRow,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        self.render_setting_row(
            label,
            description,
            is_focused,
            row,
            div().w_full().min_w_0().child(picker.clone()),
            cx,
        )
    }

    fn render_gen_input_field(
        &self,
        label: impl Into<SharedString>,
        description: impl Into<SharedString>,
        input: &Entity<InputState>,
        is_focused: bool,
        row: GeneralFormRow,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        self.render_setting_row(
            label,
            description,
            is_focused,
            row,
            div()
                .w_full()
                .min_w_0()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, window, cx| {
                        this.switching_input = true;
                        this.content_focused = true;
                        if let Some(position) = this
                            .gen_form_rows()
                            .iter()
                            .position(|candidate| *candidate == row)
                        {
                            this.gen_form_cursor = position;
                        }
                        this.gen_focus_current_input(window, cx);
                        cx.notify();
                    }),
                )
                .child(Input::new(input).small().w_full()),
            cx,
        )
    }
}
