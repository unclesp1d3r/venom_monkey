---
inclusion: always
---

# VenomMonkey Technical Standards

## Technology Stack

### Core Language

- **Rust**: Primary language for all components
- **Edition**: 2021 or later
- **MSRV**: Minimum Supported Rust Version as defined in Cargo.toml

### Dependencies

- **Async Runtime**: Tokio for asynchronous operations
- **HTTP Client/Server**: Reqwest for client, Axum/Warp for server
- **Database**: PostgreSQL with SQLx or Diesel ORM
- **Serialization**: Serde with JSON
- **Cryptography**: Ring or RustCrypto for secure operations
- **CLI**: Clap for command-line interfaces

## Architecture Principles

### Code Organization

- **Workspace Structure**: Multi-crate workspace with shared common crate
- **Separation of Concerns**: Clear boundaries between agent, client, server, and common
- **Dependency Direction**: Common crate as foundation, no circular dependencies

### Performance Standards

- **Binary Size**: Optimize for minimal footprint using `opt-level = 'z'` for agents
- **Memory Usage**: Efficient memory management, avoid unnecessary allocations
- **Network Efficiency**: Minimal bandwidth usage for beacon communications
- **Cross-Compilation**: Support for multiple target architectures

### Security Standards

- **Cryptographic Operations**: Use established, audited cryptographic libraries
- **Input Validation**: Strict validation of all external inputs
- **Error Handling**: Comprehensive error handling without information leakage
- **Secure Defaults**: Default configurations prioritize security

## Development Standards

### Code Quality

- **Formatting**: Use `rustfmt` with project-specific configuration
- **Linting**: Use `clippy` with strict linting rules
- **Testing**: Unit tests for all public APIs, integration tests for workflows
- **Documentation**: Comprehensive rustdoc for public interfaces

### Build System

- **Local Development**: Standard `cargo` commands for development
- **Cross-Compilation**: Use `cargo-zigbuild` for multi-platform builds
- **Binary Optimization**: UPX compression for release binaries
- **CI/CD**: Automated testing and building for all supported platforms

### Platform Support

- **Primary Targets**:
  - Linux x86_64
  - Windows x86_64
  - macOS aarch64 (Apple Silicon)
- **Secondary Targets**: Additional architectures as needed
- **Compatibility**: Maintain compatibility across supported platforms

## API Standards

### REST API Design

- **HTTP Methods**: Proper use of GET, POST, PUT, DELETE
- **Status Codes**: Appropriate HTTP status codes for all responses
- **Content Type**: JSON for request/response bodies
- **Error Format**: Consistent error response structure

### Database Design

- **Migrations**: Version-controlled database schema changes
- **Indexing**: Appropriate indexes for query performance
- **Constraints**: Database-level constraints for data integrity
- **Normalization**: Proper database normalization practices

## Security Considerations

### Operational Security

- **Communication**: Encrypted communication between components
- **Authentication**: Secure authentication mechanisms
- **Authorization**: Role-based access control where applicable
- **Logging**: Security-conscious logging (no sensitive data)

### Deployment Security

- **Configuration**: Secure configuration management
- **Secrets**: Proper secrets management and rotation
- **Network**: Secure network communication protocols
- **Monitoring**: Security monitoring and alerting capabilities
