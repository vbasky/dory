---
name: dory-rpc-auth-provider
description: >
  Create external Dory auth-provider services that speak the auth-provider RPC protocol.
  Trigger: When adding an RPC auth provider, external credential service, SSO plugin, OAuth plugin, or auth-provider service configured through Settings → RPC Services.
license: MIT
---

## When to Use

- The auth provider should live outside the Dory binary.
- The user mentions auth-provider RPC, external credential service, SSO plugin, or Settings → RPC Services.
- The provider should be registered with `RpcServiceKind::AuthProvider`.

## Source of Truth

- `docs/DRIVER_RPC_PROTOCOL.md`
- `crates/dory_ipc/src/auth_provider_protocol.rs`
- `crates/dory_ipc/src/auth_provider_client.rs`
- `crates/dory_app/src/rpc_services.rs`
- `examples/custom_auth_provider/`

## Critical Patterns

- Start from `examples/custom_auth_provider/` unless the user already has service code.
- Auth-provider services never register as database drivers.
- Managed auth-provider services require an explicit `command`; Dory does not assume `dory-driver-host`.
- Dory probes compatible services on startup, so restart Dory after changing RPC service settings.
- If Dory launches the provider, it injects the auth-provider handshake token through the environment.

## Protocol Checklist

1. Bind the local socket using Dory IPC conventions.
2. Handle `Hello` and negotiate the highest compatible protocol version.
3. Return `provider_id`, `display_name`, `form_definition`, and version-specific capabilities from `Hello`.
4. Implement `ValidateSession`.
5. Implement `Login`; emit zero or one `LoginUrlProgress` before terminal `LoginResult`.
6. Implement `ResolveCredentials`.
7. Return secret credential values as secret fields in the response type.
8. Return `UnsupportedMethod` for unsupported requests.
9. Use structured protocol errors such as `InvalidRequest`, `VersionMismatch`, `Provider`, or `Internal`.

## Launch Facts

- Use `RpcServiceKind::AuthProvider` in the persisted service descriptor.
- Empty `command` and empty `args`: Dory expects an already-running provider service.
- Explicit `command`: Dory starts and owns the provider process.
- Empty `command` with non-empty `args` is invalid for auth providers.

## Commands

```bash
cargo build
RUST_LOG=info cargo run -- --socket <socket-id>
cargo fmt --all -- --check
```
