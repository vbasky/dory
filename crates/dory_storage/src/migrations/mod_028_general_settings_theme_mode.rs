//! Migration 028: Theme mode plus default dark/light palettes.
//!
//! Adds `theme_mode`, `dark_theme`, and `light_theme` so the workspace can
//! follow the OS appearance while letting the user pick which dark and light
//! palettes to use.

use rusqlite::Transaction;

use crate::migrations::{Migration, MigrationError};

pub struct MigrationImpl;

impl Migration for MigrationImpl {
    fn name(&self) -> &str {
        "028_general_settings_theme_mode"
    }

    fn run(&self, tx: &Transaction) -> Result<(), MigrationError> {
        let table_exists: bool = tx
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='cfg_general_settings'",
                [],
                |row| row.get::<_, i64>(0),
            )
            .map(|n| n > 0)
            .map_err(sqlite_err)?;

        if !table_exists {
            return Ok(());
        }

        add_column_if_missing(
            tx,
            "theme_mode",
            "ALTER TABLE cfg_general_settings ADD COLUMN theme_mode TEXT NOT NULL DEFAULT 'system';",
        )?;
        add_column_if_missing(
            tx,
            "dark_theme",
            "ALTER TABLE cfg_general_settings ADD COLUMN dark_theme TEXT NOT NULL DEFAULT 'dory_dark';",
        )?;
        add_column_if_missing(
            tx,
            "light_theme",
            "ALTER TABLE cfg_general_settings ADD COLUMN light_theme TEXT NOT NULL DEFAULT 'dory_light';",
        )?;

        // Preserve an explicit non-default palette as the matching polarity pick.
        tx.execute(
            "UPDATE cfg_general_settings SET dark_theme = theme
             WHERE theme IN ('mirage', 'nord', 'dracula', 'dory_dark')",
            [],
        )
        .map_err(sqlite_err)?;
        tx.execute(
            "UPDATE cfg_general_settings SET light_theme = theme
             WHERE theme IN ('light', 'catppuccin_latte', 'github_light', 'one_light', 'dory_light')",
            [],
        )
        .map_err(sqlite_err)?;

        Ok(())
    }
}

fn add_column_if_missing(tx: &Transaction, column: &str, ddl: &str) -> Result<(), MigrationError> {
    let column_exists: bool = tx
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('cfg_general_settings') WHERE name = ?1",
            [column],
            |row| row.get::<_, i64>(0),
        )
        .map(|n| n > 0)
        .map_err(sqlite_err)?;

    if !column_exists {
        tx.execute_batch(ddl).map_err(sqlite_err)?;
    }
    Ok(())
}

fn sqlite_err(source: rusqlite::Error) -> MigrationError {
    MigrationError::Sqlite {
        path: std::path::PathBuf::from("<unknown>"),
        source,
    }
}
