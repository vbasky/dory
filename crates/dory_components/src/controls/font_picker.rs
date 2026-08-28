//! Searchable font picker.
//!
//! The popover filters installed families as the user types. The unfiltered
//! catalog is not painted in full — macOS can report hundreds of families
//! (Nerd Fonts especially), and building an element per family freezes the
//! UI thread.

use gpui::prelude::*;
use gpui::{
    AnyElement, Context, Entity, EventEmitter, FocusHandle, Hsla, MouseButton, MouseDownEvent,
    SharedString, Window, div, px,
};
use gpui_component::ActiveTheme;
use gpui_component::scroll::ScrollableElement;

use crate::controls::input::{Input, InputEvent, InputState};
use crate::icons::AppIcon;
use crate::primitives::{Icon, Text};
use crate::tokens::{FontSizes, Heights, Radii, Spacing};
use crate::typography::AppFonts;

/// Hard cap on painted rows. Type-to-search is how the rest of the catalog
/// is reached; painting every installed family is what froze Settings.
const MAX_VISIBLE_ROWS: usize = 48;

#[derive(Debug, Clone, PartialEq)]
pub struct FontPicked {
    pub family: SharedString,
}

impl EventEmitter<FontPicked> for FontPicker {}

/// State for the searchable font picker popover.
pub struct FontPicker {
    pub open: bool,
    pub query: String,
    pub families: Vec<SharedString>,
    pub filtered: Vec<SharedString>,
    pub selected_index: usize,
    /// Currently applied family. Empty string = system font.
    pub committed: SharedString,
    pub search_input: Entity<InputState>,
    pub focus_handle: FocusHandle,
    _subscriptions: Vec<gpui::Subscription>,
}

impl FontPicker {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, families: Vec<SharedString>) -> Self {
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("Search fonts…"));
        let focus_handle = cx.focus_handle();
        let families: Vec<SharedString> = families
            .into_iter()
            .filter(|family| dory_core::FontSetting::is_suitable_ui_family(family))
            .collect();
        let mut filtered = families.clone();
        filtered.sort_by_key(|f| f.to_lowercase());
        let mut this = Self {
            open: false,
            query: String::new(),
            families,
            filtered,
            selected_index: 0,
            committed: SharedString::from(""),
            search_input,
            focus_handle,
            _subscriptions: Vec::new(),
        };
        let subscription = this.bind_search_input(cx);
        this._subscriptions.push(subscription);
        this
    }

    pub fn set_committed(&mut self, family: impl Into<SharedString>) {
        self.committed = family.into();
    }

    pub fn toggle_open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.open = !self.open;
        if self.open {
            self.query.clear();
            self.filtered = self.families.clone();
            self.selected_index = self
                .filtered
                .iter()
                .position(|family| family == &self.committed)
                .unwrap_or(0);
            self.search_input.update(cx, |input, cx| {
                input.set_value("", window, cx);
                input.focus(window, cx);
            });
        }
        cx.notify();
    }

    pub fn close(&mut self, cx: &mut Context<Self>) {
        self.open = false;
        cx.notify();
    }

    fn set_query(&mut self, query: &str, cx: &mut Context<Self>) {
        self.query = query.to_string();
        let q = query.to_lowercase();
        if q.is_empty() {
            self.filtered = self.families.clone();
        } else {
            self.filtered = self
                .families
                .iter()
                .filter(|f| f.to_lowercase().contains(&q))
                .cloned()
                .collect();
        }
        self.selected_index = 0;
        cx.notify();
    }

    fn confirm_index(&mut self, index: usize, cx: &mut Context<Self>) {
        if index >= self.filtered.len() {
            return;
        }
        self.selected_index = index;
        self.confirm_selection(cx);
    }

    fn confirm_selection(&mut self, cx: &mut Context<Self>) {
        if let Some(family) = self.filtered.get(self.selected_index) {
            let family = family.clone();
            self.committed = family.clone();
            self.open = false;
            self.query.clear();
            cx.emit(FontPicked { family });
            cx.notify();
        }
    }

    fn move_selection(&mut self, delta: isize, cx: &mut Context<Self>) {
        if self.filtered.is_empty() {
            return;
        }
        let len = self.filtered.len();
        let next = self.selected_index as isize + delta;
        self.selected_index = next.rem_euclid(len as isize) as usize;
        cx.notify();
    }

    fn trigger_label(&self) -> SharedString {
        if self.open {
            return self
                .filtered
                .get(self.selected_index)
                .cloned()
                .map(|family| display_name(&family))
                .unwrap_or_else(|| SharedString::from("No match"));
        }
        display_name(&self.committed)
    }
}

fn display_name(family: &SharedString) -> SharedString {
    if family.is_empty() {
        SharedString::from("System Font")
    } else {
        family.clone()
    }
}

impl Render for FontPicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let primary = theme.primary;
        let border = theme.border;
        let trigger_label = self.trigger_label();

        let trigger = div()
            .id("font-picker-trigger")
            .w_full()
            .min_w_0()
            .h(Heights::BUTTON)
            .flex()
            .items_center()
            .justify_between()
            .gap(Spacing::SM)
            .px(Spacing::SM)
            .rounded(Radii::SM)
            .border_1()
            .border_color(border)
            .overflow_hidden()
            .cursor_pointer()
            .font_family(AppFonts::current_ui_family(cx))
            .on_click(cx.listener(|this, _, window, cx| {
                this.toggle_open(window, cx);
            }))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .text_ellipsis()
                    .child(Text::body(trigger_label).color(theme.foreground)),
            )
            .child(
                Icon::new(AppIcon::ChevronDown)
                    .size(Heights::ICON_SM)
                    .color(theme.muted_foreground),
            );

        let panel = div()
            .id("font-picker-panel")
            .w_full()
            .mt_1()
            .rounded(Radii::MD)
            .border_1()
            .border_color(border)
            .bg(theme.popover)
            .overflow_hidden()
            .on_mouse_down(MouseButton::Left, |_event: &MouseDownEvent, _, cx| {
                cx.stop_propagation();
            })
            .child(
                div()
                    .px(Spacing::SM)
                    .py(Spacing::XS)
                    .border_b_1()
                    .border_color(border)
                    .child(Input::new(&self.search_input).cleanable(true)),
            )
            .child(
                div()
                    .max_h(px(280.0))
                    .overflow_y_scrollbar()
                    .child(self.render_font_list(primary, cx)),
            );

        div()
            .id("font-picker")
            .w_full()
            .min_w_0()
            .relative()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &gpui::KeyDownEvent, window, cx| {
                if !this.open {
                    return;
                }
                match event.keystroke.key.as_ref() {
                    "Escape" => {
                        this.open = false;
                        cx.notify();
                    }
                    "Enter" => this.confirm_selection(cx),
                    "ArrowDown" => this.move_selection(1, cx),
                    "ArrowUp" => this.move_selection(-1, cx),
                    _ => {}
                }
                let _ = window;
            }))
            .child(trigger)
            .when(self.open, |el| el.child(panel))
    }
}

impl FontPicker {
    fn render_font_list(&self, primary: Hsla, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let total = self.filtered.len();
        let visible_end = total.min(MAX_VISIBLE_ROWS);
        let truncated = total > MAX_VISIBLE_ROWS;
        let needs_query_hint = self.query.trim().is_empty() && total > MAX_VISIBLE_ROWS;

        let mut rows: Vec<AnyElement> = Vec::with_capacity(visible_end + 1);

        if needs_query_hint {
            rows.push(
                div()
                    .px(Spacing::SM)
                    .py(Spacing::XS)
                    .child(
                        Text::caption(format!(
                            "Type to search {total} fonts — showing the first {MAX_VISIBLE_ROWS}"
                        ))
                        .color(theme.muted_foreground),
                    )
                    .into_any_element(),
            );
        }

        for ix in 0..visible_end {
            let family = self.filtered[ix].clone();
            let is_selected = ix == self.selected_index;
            let is_committed = family == self.committed;
            let label = display_name(&family);
            rows.push(
                div()
                    .id(SharedString::from(format!("font-item-{ix}")))
                    .w_full()
                    .px(Spacing::SM)
                    .py(Spacing::XS)
                    .flex()
                    .items_center()
                    .justify_between()
                    .text_color(if is_selected || is_committed {
                        primary
                    } else {
                        theme.foreground
                    })
                    .bg(if is_selected {
                        theme.accent.opacity(0.35)
                    } else {
                        gpui::transparent_black()
                    })
                    .font_family(AppFonts::current_ui_family(cx))
                    .text_size(FontSizes::SM)
                    .cursor_pointer()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _event: &MouseDownEvent, _, cx| {
                            cx.stop_propagation();
                            this.confirm_index(ix, cx);
                        }),
                    )
                    .child(label)
                    .into_any_element(),
            );
        }

        if truncated && !needs_query_hint {
            rows.push(
                div()
                    .px(Spacing::SM)
                    .py(Spacing::XS)
                    .child(
                        Text::caption(format!(
                            "Showing {MAX_VISIBLE_ROWS} of {total} — type more to narrow"
                        ))
                        .color(theme.muted_foreground),
                    )
                    .into_any_element(),
            );
        }

        div().flex().flex_col().children(rows)
    }

    fn bind_search_input(&mut self, cx: &mut Context<Self>) -> gpui::Subscription {
        let input = self.search_input.clone();
        cx.subscribe(&input, {
            let input = input.clone();
            move |this, _, event: &InputEvent, cx| match event {
                InputEvent::Change => {
                    let q = input.read(cx).value().to_string();
                    this.set_query(&q, cx);
                }
                InputEvent::PressEnter { .. } => {
                    this.confirm_selection(cx);
                }
                _ => {}
            }
        })
    }
}
