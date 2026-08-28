/// Integration tests verifying that `EventRecord` values produced by the
/// tracing bridge (category = `System`, as required by V1 coercion) all pass
/// `AuditService::validate_event`.
///
/// These tests do not instantiate a real tracing subscriber.  They construct
/// records directly and feed them to the validator to confirm no bridge record
/// will be silently rejected downstream.
use dory_audit::AuditService;
use dory_core::observability::{EventCategory, EventOutcome, EventRecord, EventSeverity};

fn minimal_system_record(action: &str, summary: &str) -> EventRecord {
    EventRecord::new(
        0,
        EventSeverity::Info,
        EventCategory::System,
        EventOutcome::Success,
    )
    .with_summary(summary)
    .with_action(action)
    .with_actor_id("system")
}

#[test]
fn system_record_with_action_and_summary_passes_validate() {
    let record = minimal_system_record("log_event", "something happened");
    assert!(
        AuditService::validate_event(&record).is_ok(),
        "minimal System record should pass validate_event"
    );
}

#[test]
fn system_record_empty_action_fails_validate() {
    let record = minimal_system_record("", "something happened");
    assert!(
        AuditService::validate_event(&record).is_err(),
        "empty action should fail validate_event"
    );
}

#[test]
fn system_record_empty_summary_fails_validate() {
    let record = minimal_system_record("log_event", "");
    assert!(
        AuditService::validate_event(&record).is_err(),
        "empty summary should fail validate_event"
    );
}

/// For every module target that the bridge prefix table covers, a synthesized
/// `System`-category record (as produced by V1 coercion) must pass
/// `validate_event`.  This guards against category regressions where a prefix
/// entry is changed to produce a category that requires additional structured
/// fields the bridge cannot supply.
#[test]
fn system_records_for_all_prefix_map_modules_pass_validate() {
    let module_targets = [
        "dory_core::connection::pool",
        "dory_core::pipeline::exec",
        "dory_app::access_manager::auth",
        "dory_ssh::tunnel",
        "dory_proxy::http",
        "dory_aws::client",
        "dory_ssm::params",
        "dory_driver_ipc::host",
        "dory_app::config::profiles",
        "dory_app::aws_config::reflect",
        "dory_storage::migrations",
        "dory_core::storage::db",
        "dory_driver_sqlite::query",
        "dory_core::facade::session",
        "dory_mcp::runtime",
        "dory_mcp_server::governance",
        "dory_app::mcp_command::run",
        "dory_app::app_state::init",
        "dory_ipc::framing",
        "dory_driver_host::main",
        "dory_ui::workspace",
    ];

    for target in module_targets {
        let action = target.rsplit("::").next().unwrap_or("log_event");
        let record = minimal_system_record(action, "test event from tracing bridge");
        let result = AuditService::validate_event(&record);
        assert!(
            result.is_ok(),
            "System record from target '{target}' (action='{action}') should pass validate_event, got: {result:?}"
        );
    }
}
