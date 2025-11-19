# Integration Testing Workflow

This document describes how to run integration tests for the log-server project.

## Overview

The integration testing workflow uses Docker Compose to deploy the application and database services, then runs tests against the running services. This approach ensures that tests run against a real environment that closely matches production.

## Testing Approaches

### 1. Full Integration Tests (Recommended for CI/CD)

Uses a dedicated test environment with isolated containers:

```bash
# Run the complete test workflow
./scripts/test-integration.sh

# Or step by step:
./scripts/test-integration.sh setup    # Setup test environment
./scripts/test-integration.sh test     # Run tests
./scripts/test-integration.sh cleanup  # Cleanup
```

**Services:**
- Test database: `postgres-test` (port 5434)
- Test application: `log-server-test` (port 8082)

### 2. Development Integration Tests (Quick iteration)

Uses the existing dev environment for faster iteration during development:

```bash
# First, make sure dev environment is running
docker-compose -f docker-compose.dev.yml up -d

# Then run tests against it
./scripts/test-dev.sh test

# Or just check if dev environment is ready
./scripts/test-dev.sh check
```

**Services:**
- Dev database: `postgres` (port 5433)  
- Dev application: `log-server-dev` (port 8081)

## Test Configuration

The integration tests can be configured via environment variables:

- `TEST_BASE_URL`: Base URL for the test service (default: `http://localhost:8082`)

For dev testing, the script automatically sets `TEST_BASE_URL=http://localhost:8081`.

## Test Coverage

The integration tests cover:

1. **Health Check**: Verify service is responding
2. **Root Endpoint**: Basic connectivity test
3. **Schema Management**: 
   - List schemas
   - Create schema
   - Get schema by ID
4. **Log Management**:
   - Create log entries with schema validation
   - Retrieve logs by schema

## Docker Compose Profiles

### Test Environment (`docker-compose.test.yml`)
- Isolated test database and application
- Uses test credentials and database name
- Runs on different ports to avoid conflicts
- Automatically cleaned up after tests

### Development Environment (`docker-compose.dev.yml`)
- Persistent development database
- Hot reloading for code changes
- Suitable for interactive development

### Production Environment (`docker-compose.yml`)
- Production-ready configuration
- Health checks and restart policies
- Optimized for deployment

## Prerequisites

1. Docker and Docker Compose installed
2. Rust and Cargo installed (for running tests)
3. curl (for health checks in scripts)

## Troubleshooting

### Tests fail with connection errors
- Check if Docker containers are running: `docker-compose -f docker-compose.test.yml ps`
- Check container logs: `./scripts/test-integration.sh logs`
- Ensure ports 5434 and 8082 are not in use

### Database connection issues
- Verify database is healthy: `docker-compose -f docker-compose.test.yml exec postgres-test pg_isready -U testuser -d logserver_test`
- Check if database initialization completed successfully

### SSL/TLS linking errors
- These typically occur during Rust compilation, not runtime
- Install development libraries if needed
- Consider using the dev environment approach which doesn't require rebuilding

## CI/CD Integration

For continuous integration, use the full integration test workflow:

```yaml
# Example GitHub Actions step
- name: Run Integration Tests
  run: |
    chmod +x scripts/test-integration.sh
    ./scripts/test-integration.sh
```

The script handles environment setup, test execution, and cleanup automatically.

## Manual Testing

You can also test endpoints manually once the environment is running:

```bash
# Setup test environment
./scripts/test-integration.sh setup

# Test health endpoint
curl http://localhost:8082/health

# Create a schema
curl -X POST http://localhost:8082/schemas \
  -H "Content-Type: application/json" \
  -d '{
    "name": "test-schema",
    "version": "1.0.0",
    "schema": {
      "type": "object",
      "properties": {
        "message": {"type": "string"}
      }
    }
  }'

# Cleanup when done
./scripts/test-integration.sh cleanup
```
