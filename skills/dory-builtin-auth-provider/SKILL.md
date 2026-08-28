---
name: dory-builtin-auth-provider
description: >
  Add native Dory auth providers and credential resolvers using the core auth-provider contracts.
  Trigger: When adding a built-in auth provider, credential provider, SSO provider, OAuth provider, or secret/parameter value provider.
license: MIT
---

## When to Use

- The auth provider should be compiled into Dory.
- The provider should appear in Settings → Auth Profiles without configuring an RPC service.
- The provider resolves credentials or registers secret/parameter value providers.

## Source of Truth

- `crates/dory_core/src/auth/mod.rs`
- `crates/dory_core/src/auth/types.rs`
- `crates/dory_app/src/auth_provider_registry.rs`
- `crates/dory_app/src/app_state.rs`
- `crates/dory_aws/src/auth.rs` for built-in examples
- `crates/dory_ui/src/ui/windows/settings/auth_profiles_section.rs` for generic form behavior

## Critical Patterns

- Prefer implementing `DynAuthProvider` when the provider needs a form definition, login URL callback, dynamic options, value-provider registration, or import/write-back hooks.
- `AuthFormDef` reuses `DriverFormDef`; do not create provider-specific UI rendering.
- Return secrets through `ResolvedCredentials.secret_fields`, never regular fields or logs.
- `AuthSession.data` and `ResolvedCredentials.provider_data` are opaque runtime data, not persisted config.
- Capability flags must describe real behavior, especially login support and verification URL progress.

## Checklist

1. Define a stable `provider_id()` and display name.
2. Return an auth form definition for Settings → Auth Profiles.
3. Implement `validate_session()`.
4. Implement `login()` and call the URL callback when interactive login exposes a verification URL.
5. Implement `resolve_credentials()` with clear separation between plain fields and secret fields.
6. Override `register_value_providers()` only for real secret/parameter backends.
7. Override import/write-back hooks only when the provider owns external profiles.
8. Register the provider in `AuthProviderRegistry` in `AppState` behind the correct feature gate.
9. Add tests for form parsing, session states, credential resolution, and secret redaction expectations.

## Boundaries

- Do not log tokens, passwords, session tokens, access keys, or secret fields.
- Do not add auth-provider-specific UI branches unless the existing generic form seam is insufficient and the user approves a new generic seam.
- Do not persist opaque session/provider data.

## Commands

```bash
cargo fmt --all -- --check
cargo test -p <provider-crate-or-dory_aws>
cargo check --workspace
```
