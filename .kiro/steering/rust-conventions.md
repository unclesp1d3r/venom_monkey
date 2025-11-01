---
inclusion: fileMatch
fileMatchPattern: 
    - '*.rs'
---

# Rust Coding Conventions

## Error Handling

- Use `anyhow::Result<T>` for application-level errors
- Use `thiserror::Error` for custom error types
- Follow the pattern in [agent/src/error.rs](mdc:agent/src/error.rs) and [client/src/error.rs](mdc:client/src/error.rs)

## Module Organization

- Keep modules focused and cohesive
- Use `mod.rs` files for module declarations
- Follow the pattern: `mod name;` then `use name::*;` for re-exports

## Dependencies

- Prefer workspace dependencies over external crates
- Use `common = { path = "../common" }` for shared code
- Keep cryptographic dependencies consistent across components

## Configuration

- Use environment variables with `load-dotenv` crate
- Follow the config pattern in [agent/src/config.rs](mdc:agent/src/config.rs)
- Use `serde` for serialization/deserialization

## Async Code

- Use `tokio` for async runtime
- Use `warp` for HTTP server (server component)
- Use `reqwest` for HTTP client (client component)
- Use `ureq` for simple HTTP requests (agent component)

## Security

- Use `zeroize` for sensitive data cleanup
- Use `ed25519-dalek` and `x25519-dalek` for cryptographic operations
- Use `chacha20poly1305` for encryption
- Use `blake2` for hashing
