/// Tests for StorageRuntime accessor error propagation.
///
/// Verifies that audit(), saved_filters(), and viz_connection() return Err
/// rather than panicking when the underlying open_dory_db call fails.
use dory_storage::bootstrap::StorageRuntime;

/// Produces a StorageRuntime whose dory_db_path points at a directory
/// rather than a file, causing subsequent open_dory_db calls to fail.
fn runtime_with_inaccessible_db() -> StorageRuntime {
    let unique_dir = std::env::temp_dir().join(format!(
        "dory_bootstrap_test_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&unique_dir).expect("should create temp dir");

    // Start with a real runtime so migrations run successfully.
    let real =
        StorageRuntime::for_path(unique_dir.join("dory.db")).expect("runtime should initialize");

    // Replace the db file with a directory so subsequent open calls fail.
    std::fs::remove_file(unique_dir.join("dory.db")).expect("should remove db file");
    std::fs::create_dir_all(unique_dir.join("dory.db"))
        .expect("should create directory at db path");

    real
}

#[test]
fn audit_returns_err_when_db_inaccessible() {
    let rt = runtime_with_inaccessible_db();
    assert!(
        rt.audit().is_err(),
        "audit() must return Err when the database path is inaccessible"
    );
}

#[test]
fn saved_filters_returns_err_when_db_inaccessible() {
    let rt = runtime_with_inaccessible_db();
    assert!(
        rt.saved_filters().is_err(),
        "saved_filters() must return Err when the database path is inaccessible"
    );
}

#[test]
fn viz_connection_returns_err_when_db_inaccessible() {
    let rt = runtime_with_inaccessible_db();
    assert!(
        rt.viz_connection().is_err(),
        "viz_connection() must return Err when the database path is inaccessible"
    );
}
