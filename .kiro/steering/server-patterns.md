---
inclusion: fileMatch
fileMatchPattern: 
    - 'server/**/*.rs'
---

# Server Component Patterns

## Architecture

The server is an HTTP API server that manages agents and coordinates job execution.

## Web Framework

- Uses `warp` for HTTP server
- Async/await with `tokio` runtime
- RESTful API design with proper HTTP status codes

## Database Layer

- Uses `sqlx` with PostgreSQL
- Database migrations in [db/migrations/](mdc:server/db/migrations/)
- Repository pattern for data access
- Service layer for business logic

## API Structure

- Routes organized in [api/routes/](mdc:server/src/api/routes/)
- Each resource has its own route module
- Shared error handling in [api/error.rs](mdc:server/src/api/error.rs)
- Application state management with `Arc<AppState>`

## Database Patterns

- **Entities**: Data models in [entities.rs](mdc:server/src/entities.rs)
- **Repository**: Data access layer in [repository/](mdc:server/src/repository/)
- **Service**: Business logic in [service/](mdc:server/src/service/)

## Configuration

- Environment-based configuration
- Database connection pooling
- Graceful shutdown handling

## Security

- Agent authentication and authorization
- Secure job execution coordination
- Cryptographic verification of agent identity
