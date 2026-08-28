use gpui::actions;

actions!(
    dory,
    [
        Cancel,
        // List navigation
        SelectNext,
        SelectPrev,
        Execute,
        // CRUD / item actions
        Delete,
        ToggleFavorite,
        Rename,
        // Search / saved queries
        FocusSearch,
        SaveQuery,
    ]
);
