//! Native application menus (macOS menu bar, and GPUI menus on other platforms).
//!
//! Actions here must already be handled on the focused workspace (or registered
//! with `cx.on_action` at app start for process-level items such as Quit).

use crate::keymap::{
    CancelQuery, CloseCurrentTab, Disconnect, ExportResults, FocusBackgroundTasks, FocusEditor,
    FocusResults, FocusSidebar, HideApp, HideOtherApps, ImportDashboard, NewDashboard, NewQueryTab,
    OpenAuditViewer, OpenConnectionManager, OpenSavedChart, OpenSavedQueries, OpenScriptFile,
    OpenSettings, Quit, RefreshSchema, ResultsAddRow, ResultsCopyCell, ResultsDeleteRow,
    ResultsDuplicateRow, RunQuery, RunQueryInNewTab, SaveFileAs, SaveQuery, ToggleCommandPalette,
    ToggleResults, ToggleSidebar, ToggleTasks,
};
use gpui::{Menu, MenuItem, OsAction, SystemMenuType};
use gpui_component::input::{Copy, Cut, Paste, Redo, Search as InputSearch, SelectAll, Undo};

/// Menus shown in the platform application menu bar.
pub fn app_menus() -> Vec<Menu> {
    vec![
        Menu {
            name: "Dory".into(),
            items: vec![
                MenuItem::action("Settings…", OpenSettings),
                MenuItem::separator(),
                MenuItem::os_submenu("Services", SystemMenuType::Services),
                MenuItem::separator(),
                MenuItem::action("Hide Dory", HideApp),
                MenuItem::action("Hide Others", HideOtherApps),
                MenuItem::separator(),
                MenuItem::action("Quit Dory", Quit),
            ],
        },
        Menu {
            name: "File".into(),
            items: vec![
                MenuItem::action("New Query", NewQueryTab),
                MenuItem::action("New Dashboard…", NewDashboard),
                MenuItem::action("Open Script…", OpenScriptFile),
                MenuItem::separator(),
                MenuItem::action("Save", SaveQuery),
                MenuItem::action("Save As…", SaveFileAs),
                MenuItem::action("Close Tab", CloseCurrentTab),
                MenuItem::separator(),
                MenuItem::action("Export Results…", ExportResults),
            ],
        },
        Menu {
            name: "Edit".into(),
            items: vec![
                MenuItem::os_action("Undo", Undo, OsAction::Undo),
                MenuItem::os_action("Redo", Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action("Cut", Cut, OsAction::Cut),
                MenuItem::os_action("Copy", Copy, OsAction::Copy),
                MenuItem::os_action("Paste", Paste, OsAction::Paste),
                MenuItem::os_action("Select All", SelectAll, OsAction::SelectAll),
                MenuItem::separator(),
                MenuItem::action("Add Row", ResultsAddRow),
                MenuItem::action("Duplicate Rows", ResultsDuplicateRow),
                MenuItem::action("Copy Selected Cells", ResultsCopyCell),
                MenuItem::action("Delete Rows", ResultsDeleteRow),
                MenuItem::separator(),
                MenuItem::action("Find…", InputSearch),
            ],
        },
        Menu {
            name: "View".into(),
            items: vec![
                MenuItem::action("Toggle Sidebar", ToggleSidebar),
                MenuItem::action("Toggle Background Tasks", ToggleTasks),
                MenuItem::action("Toggle Results", ToggleResults),
                MenuItem::separator(),
                MenuItem::action("Focus Sidebar", FocusSidebar),
                MenuItem::action("Focus Editor", FocusEditor),
                MenuItem::action("Focus Results", FocusResults),
                MenuItem::action("Focus Background Tasks", FocusBackgroundTasks),
                MenuItem::separator(),
                MenuItem::action("Command Palette…", ToggleCommandPalette),
            ],
        },
        Menu {
            name: "Query".into(),
            items: vec![
                MenuItem::action("Run Query", RunQuery),
                MenuItem::action("Run Query in New Tab", RunQueryInNewTab),
                MenuItem::action("Cancel Query", CancelQuery),
                MenuItem::separator(),
                MenuItem::action("Saved Queries…", OpenSavedQueries),
            ],
        },
        Menu {
            name: "Connection".into(),
            items: vec![
                MenuItem::action("Connection Manager…", OpenConnectionManager),
                MenuItem::action("Refresh Schema", RefreshSchema),
                MenuItem::action("Disconnect", Disconnect),
            ],
        },
        Menu {
            name: "Tools".into(),
            items: vec![
                MenuItem::action("Audit Viewer…", OpenAuditViewer),
                MenuItem::action("Open Saved Chart…", OpenSavedChart),
                MenuItem::action("Import Dashboard…", ImportDashboard),
            ],
        },
    ]
}

/// Register process-level menu actions (Quit / Hide) and install the menu bar.
pub fn install_app_menus(cx: &mut gpui::App) {
    cx.on_action(|_: &Quit, cx| cx.quit());
    cx.on_action(|_: &HideApp, cx| cx.hide());
    cx.on_action(|_: &HideOtherApps, cx| cx.hide_other_apps());
    cx.set_menus(app_menus());
}

#[cfg(test)]
mod tests {
    use super::app_menus;

    #[test]
    fn app_menus_expose_file_edit_view_query_connection() {
        let names: Vec<_> = app_menus().into_iter().map(|menu| menu.name).collect();
        assert_eq!(
            names,
            vec![
                "Dory",
                "File",
                "Edit",
                "View",
                "Query",
                "Connection",
                "Tools",
            ]
        );
    }
}
