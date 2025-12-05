# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Fixed

## [1.1.0] - 2025-12-05

### Added
- WebSocket endpoint for real-time log event streaming
  - `GET /ws/logs` - WebSocket connection for live log updates
  - Optional `schema_id` query parameter to filter events by schema
  - Broadcasts `created` events when logs are created
  - Broadcasts `deleted` events when logs are deleted
- Comprehensive WebSocket integration tests
  - Connection tests (successful connections, schema validation, error handling)
  - Event broadcasting tests (created/deleted events, schema filtering)
  - Multi-client tests (simultaneous connections, event distribution)
- Python WebSocket client example in `examples/websocket-events-client/`
- WebSocket documentation in README.md with usage examples
- Event-driven architecture using `tokio::sync::broadcast` channel

### Changed
- Updated dependencies to support WebSocket functionality
  - Added `futures-util` for WebSocket stream handling
  - Added `tokio-tungstenite` as dev dependency for testing

## [1.0.0] - 2025-11-26

### Added
- Initial release of log-server
- Schema management endpoints
  - `POST /schemas` - Create custom JSON Schema definitions
  - `GET /schemas` - List all schemas with filtering
  - `GET /schemas/{id}` - Get specific schema by UUID
  - `GET /schemas/{name}/{version}` - Get schema by name and version
  - `PUT /schemas/{id}` - Update existing schema
  - `DELETE /schemas/{id}` - Delete schema (with force option for cascading)
- Log management endpoints
  - `POST /logs` - Create log entry with schema validation
  - `GET /logs/schema/{name}` - Get logs by schema name
  - `GET /logs/schema/{name}/{version}` - Get logs by schema name and version
  - `GET /logs/{id}` - Get specific log entry
  - `DELETE /logs/{id}` - Delete log entry
- Health check endpoints
  - `GET /` - Service health status
  - `GET /health` - Service health status
- Request tracking via `X-Request-ID` header
  - Auto-generates UUID v4 if not provided by client
  - Echoes request ID in all responses
  - Includes request ID in server logs for correlation and debugging
- PostgreSQL database with JSONB storage
  - GIN indexes for efficient JSON querying
  - Schema versioning support
  - Foreign key constraints
  - Unique constraints on schema name/version pairs
- JSON Schema Draft 7 validation
- Docker Compose deployment
  - Separate development and production configurations
  - PostgreSQL 16-alpine database
  - Health checks for both services
  - Volume persistence for database
- Comprehensive error handling
  - HTTP 400 (Bad Request), 404 (Not Found), 409 (Conflict), 422 (Unprocessable Entity), 500 (Internal Server Error)
  - Detailed error messages
  - Validation error details
  - Request ID in all error responses
- API documentation
  - OpenAPI 3.0 specification (`docs/openapi.yaml`)
  - Software Requirements Document (`docs/SRD.md`)
  - Performance benchmarks (`BENCHMARK.md`)
- Testing infrastructure
  - Integration tests for all endpoints
  - Test fixtures and utilities
  - Docker-based test environment
  - Concurrent performance benchmarking script
- Shell scripts for API interaction
  - Schema creation, retrieval, update, deletion
  - Log creation and retrieval
  - Configuration management

### Performance
- Achieves 2,361 req/s throughput (136% above SRD requirement of 1,000 req/s)
- Average response time: 33.65ms
- P95 response time: 60.18ms
- P99 response time: 77.43ms
- 100% success rate under load testing
- See `BENCHMARK.md` for detailed metrics

### Technical Details
- Built with Rust 1.82+ (edition 2021)
- Axum web framework for async HTTP handling
- SQLx for async PostgreSQL operations with compile-time query verification
- Tower middleware for request processing
- Tracing for structured logging with request ID correlation
- Serde for JSON serialization/deserialization
- UUID v4 generation for schemas and request tracking
- jsonschema crate for JSON Schema Draft 7 validation

---

## Release Notes Format

Each release follows this structure:

- **Added** - New features
- **Changed** - Changes to existing functionality
- **Deprecated** - Soon-to-be removed features
- **Removed** - Removed features
- **Fixed** - Bug fixes
- **Security** - Security improvements
- **Performance** - Performance improvements

---

[Unreleased]: https://github.com/Milo46/log-server/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/Milo46/log-server/releases/tag/v1.0.0
