---
inclusion: always
---

# VenomMonkey Project Structure

## Workspace Layout

```text
venommonkey/
├── agent/                  # Cross-platform beacon agent
├── client/                 # CLI management client
├── server/                 # HTTP API server (current)
├── server2/               # Alternative server implementation
├── common/                # Shared types and utilities
├── docker/                # Multi-architecture Docker builds
├── target/                # Cargo build artifacts
└── .kiro/                 # Kiro IDE configuration
```

## Component Responsibilities

### Agent (`agent/`)

- **Purpose**: Lightweight beacon that runs on target systems
- **Key Files**:
  - `src/main.rs` - Entry point and daemon setup
  - `src/config.rs` - Configuration management
  - `src/run.rs` - Main beacon loop
  - `src/install/` - Platform-specific installation logic
- **Dependencies**: Minimal runtime dependencies for stealth
- **Build Target**: Optimized for size (`opt-level = 'z'`)

### Client (`client/`)

- **Purpose**: CLI interface for managing agents and jobs
- **Key Files**:
  - `src/main.rs` - CLI entry point
  - `src/cli/` - Command implementations
  - `src/api/` - Server API client
  - `src/config.rs` - Client configuration
- **Dependencies**: Rich CLI experience with clap
- **Build Target**: Standard optimization for usability

### Server (`server/`)

- **Purpose**: Central coordination server with REST API
- **Key Files**:
  - `src/main.rs` - Server startup and configuration
  - `src/api/` - HTTP route handlers
  - `src/service/` - Business logic layer
  - `src/repository/` - Data access layer
  - `src/entities.rs` - Data models
  - `db/migrations/` - Database schema evolution
- **Dependencies**: Web framework, database ORM, async runtime
- **Build Target**: Standard optimization for performance

### Common (`common/`)

- **Purpose**: Shared types, utilities, and API definitions
- **Key Files**:
  - `src/api.rs` - Shared API types and structures
  - `src/crypto.rs` - Cryptographic utilities
  - `src/lib.rs` - Public interface
- **Dependencies**: Core utilities used by all components
- **Build Target**: Library crate, no binary

## File Organization Patterns

### Source Code Structure

```text
src/
├── main.rs                # Binary entry point
├── lib.rs                 # Library root (for common)
├── config.rs              # Configuration management
├── error.rs               # Error types and handling
├── api/                   # API-related modules
│   ├── mod.rs
│   ├── routes/            # HTTP route handlers
│   └── error.rs           # API-specific errors
├── service/               # Business logic layer
│   ├── mod.rs
│   └── *.rs               # Service implementations
└── repository/            # Data access layer
    ├── mod.rs
    └── *.rs               # Repository implementations
```

### Configuration Files

- **Cargo.toml**: Workspace and crate configurations
- **Cross.toml**: Cross-compilation settings
- **Makefile**: Build automation and platform targets
- **Dockerfile**: Container build definitions
- **.env**: Environment variables (not committed)

## Build and Deployment Structure

### Build Targets

- **Native**: Local development and testing
- **Linux x86_64**: Primary deployment target
- **Windows x86_64**: Windows environment support
- **macOS aarch64**: Apple Silicon support

### Artifact Organization

```text
target/
├── debug/                 # Development builds
├── release/               # Optimized builds
└── <target-triple>/       # Cross-compiled artifacts
    ├── debug/
    └── release/
```

### Docker Structure

```text
docker/
├── Dockerfile.aarch64     # ARM64 container build
├── Dockerfile.armv7       # ARMv7 container build
└── Dockerfile.windows     # Windows container build
```

## Development Workflow

### Local Development

1. **Setup**: `cargo build` for initial compilation
2. **Testing**: `cargo test` for all workspace members
3. **Formatting**: `cargo fmt` with project configuration
4. **Linting**: `cargo clippy` for code quality

### Cross-Platform Building

1. **Install Cross**: `cargo install cross`
2. **Build Targets**: `make` for all platform builds
3. **Compression**: UPX for binary size optimization
4. **Testing**: Platform-specific testing where possible

### Database Management

1. **Migrations**: Version-controlled SQL files in `server/db/migrations/`
2. **Schema**: Generated schema files for ORM integration
3. **Seeding**: Development data setup scripts
4. **Backup**: Production backup and restore procedures
