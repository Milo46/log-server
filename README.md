# Log Server

A lightweight, schema-driven HTTP log sink service built with **Rust** and **Axum** framework. Allows users to define custom JSON schemas and receive validated log entries with PostgreSQL persistence.

## 🚀 Features

- **Schema-Driven Logging**: Define custom JSON schemas for log validation
- **JSON Schema Validation**: Automatic validation against JSON Schema Draft 7
- **PostgreSQL with JSONB**: Efficient storage and querying of JSON log data
- **RESTful API**: Complete CRUD operations for schemas and logs
- **Docker Containerization**: Production and development Docker environments
- **Health Monitoring**: Built-in health check endpoints
- **Development Tools**: Hot-reloading, debugging, and live development setup
- **CORS Support**: Cross-origin resource sharing enabled
- **Structured Logging**: Comprehensive tracing with tracing-subscriber

## 📋 Quick Start

### Prerequisites

- **Docker & Docker Compose** - For containerized deployment (recommended)
- **Rust 1.82+** - For local development (optional)
- **PostgreSQL 16+** - If running without Docker (optional)

### 1. Using Docker Compose (Recommended)

```bash
# Clone the repository
git clone https://github.com/Milo46/log-server.git
cd log-server

# Configure environment (optional - has defaults)
cp .env.example .env
# Edit .env if you want to change POSTGRES_PASSWORD

# Start production services
docker-compose up -d

# Wait for services to be healthy (check logs)
docker-compose logs -f

# Test the server
curl http://localhost:8080/health
# Expected response:
# {
#   "status": "healthy",
#   "service": "log-server",
#   "timestamp": "2025-11-13T00:43:40.616582133+00:00"
# }

# Create your first schema
curl -X POST http://localhost:8080/schemas \
  -H "Content-Type: application/json" \
  -d '{
    "name": "my-app-logs",
    "version": "1.0.0",
    "description": "My application logs",
    "schema_definition": {
      "type": "object",
      "required": ["level", "message"],
      "properties": {
        "level": { "type": "string" },
        "message": { "type": "string" }
      }
    }
  }'

# Note: The response will include a UUID for the schema (e.g., "id": "a3bb189e-8bf9-3888-9912-ace4e6543002")

# Send a log entry (use the UUID from the schema creation response)
curl -X POST http://localhost:8080/logs \
  -H "Content-Type: application/json" \
  -d '{
    "schema_id": "a3bb189e-8bf9-3888-9912-ace4e6543002",
    "log_data": {
      "level": "INFO",
      "message": "Hello, Log Server!"
    }
  }'
```

### 2. Development Setup

```bash
# Start development environment with hot-reloading
docker-compose -f docker-compose.dev.yml up -d

# View logs with live updates
docker-compose -f docker-compose.dev.yml logs -f log-server-dev

# Development server runs on http://localhost:8081
curl http://localhost:8081/health
```

### 3. Local Development (without Docker)

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install and start PostgreSQL (example for Ubuntu/Debian)
sudo apt-get install postgresql postgresql-contrib
sudo systemctl start postgresql

# Create database and user
sudo -u postgres psql -c "CREATE DATABASE logserver;"
sudo -u postgres psql -c "CREATE USER loguser WITH PASSWORD 'logpass123';"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE logserver TO loguser;"

# Run initialization script
psql -U loguser -d logserver -f docker/db/init.sql

# Set environment variable
export DATABASE_URL="postgresql://loguser:logpass123@localhost:5432/logserver"

# Run the server
cargo run

# Server runs on http://localhost:8080
curl http://localhost:8080/health
```

## 🏗️ Architecture

```text
┌─────────────────┐    HTTP/JSON    ┌──────────────────┐
│   Clients       │ ──────────────► │   Log Server     │
│   (curl, apps)  │ ◄────────────── │   (Rust/Axum)    │
└─────────────────┘   Validated     └──────────────────┘
                       Logs                    │
                                               │ sqlx
                                               ▼
                                    ┌──────────────────┐
                                    │   PostgreSQL     │
                                    │   + JSONB        │
                                    └──────────────────┘
```

### Components

- **HTTP Server**: Rust-based async server using Axum framework
- **Schema Service**: JSON Schema validation and management
- **Log Service**: Log entry creation and retrieval with validation
- **Database Layer**: PostgreSQL with JSONB support via sqlx
- **Containerization**: Docker and Docker Compose for deployment
- **Tracing**: Structured logging with tracing/tracing-subscriber

## 📚 API Endpoints

### Schema Management

| Method | Endpoint | Description | Status |
|--------|----------|-------------|--------|
| GET    | `/` | Health check (root endpoint) | ✅ |
| GET    | `/health` | Health check with service info | ✅ |
| GET    | `/schemas` | Get all registered schemas | ✅ |
| POST   | `/schemas` | Create a new log schema | ✅ |
| GET    | `/schemas/{id}` | Get schema by UUID | ✅ |
| PUT    | `/schemas/{id}` | Update an existing schema | ✅ |
| DELETE | `/schemas/{id}` | Delete a schema | ✅ |

### Log Management

| Method | Endpoint | Description | Status |
|--------|----------|-------------|--------|
| POST   | `/logs` | Create a validated log entry (requires UUID schema_id) | ✅ |
| GET    | `/logs/schema/{schema_name}` | Get logs for schema (latest version) | ✅ |
| GET    | `/logs/schema/{schema_name}/{version}` | Get logs for specific schema version | ✅ |
| GET    | `/logs/{id}` | Get a specific log entry by ID | ✅ |
| DELETE | `/logs/{id}` | Delete a log entry | ✅ |

### Example Usage

#### 1. Create a Schema

```bash
curl -X POST http://localhost:8080/schemas \
  -H "Content-Type: application/json" \
  -d '{
    "name": "web-server-logs",
    "version": "1.0.0",
    "description": "Schema for web server access logs",
    "schema_definition": {
      "type": "object",
      "required": ["level", "message", "request_id"],
      "properties": {
        "timestamp": { "type": "string", "format": "date-time" },
        "level": { "type": "string", "enum": ["DEBUG", "INFO", "WARN", "ERROR"] },
        "message": { "type": "string", "minLength": 1 },
        "request_id": { "type": "string", "pattern": "^[a-zA-Z0-9-]+$" },
        "user_id": { "type": "string" },
        "response_time_ms": { "type": "number", "minimum": 0 }
      }
    }
  }'

# Response will include a generated UUID as the schema ID:
# {
#   "id": "550e8400-e29b-41d4-a716-446655440000",
#   "name": "web-server-logs",
#   "version": "1.0.0",
#   ...
# }
```

#### 2. Create a Log Entry

```bash
# Use the UUID returned from creating the schema
curl -X POST http://localhost:8080/logs \
  -H "Content-Type: application/json" \
  -d '{
    "schema_id": "550e8400-e29b-41d4-a716-446655440000",
    "log_data": {
      "timestamp": "2025-11-13T10:00:00Z",
      "level": "INFO",
      "message": "User login successful",
      "request_id": "req-12345",
      "user_id": "user-67890",
      "response_time_ms": 150
    }
  }'
```

#### 3. Retrieve Logs

```bash
# Get all logs for a schema (by name, latest version)
curl http://localhost:8080/logs/schema/web-server-logs

# Get logs for specific schema version
curl http://localhost:8080/logs/schema/web-server-logs/1

# Get all schemas
curl http://localhost:8080/schemas

# Get specific schema by UUID
curl http://localhost:8080/schemas/550e8400-e29b-41d4-a716-446655440000
```

## 🚀 Deployment

### Using Docker Compose

#### Production Deployment

```bash
# Clone the repository
git clone <your-repo-url>
cd log-server

# Configure environment
cp .env.example .env
# Edit .env and set POSTGRES_PASSWORD

# Start production services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f
```

#### Development Deployment

```bash
# Start development environment with hot-reloading
docker-compose -f docker-compose.dev.yml up -d

# View development logs
docker-compose -f docker-compose.dev.yml logs -f log-server-dev

# Stop development environment
docker-compose -f docker-compose.dev.yml down
```

#### Available Services

| Service | Port | Environment | Description |
|---------|------|-------------|-------------|
| **log-server** | 8080 | Production | Main application server |
| **postgres** | 5432 | Production | PostgreSQL database |
| **log-server-dev** | 8081 | Development | Dev server with hot-reload |
| **postgres** | 5433 | Development | Dev PostgreSQL database |

#### Docker Commands

```bash
# Rebuild images
docker-compose build --no-cache

# View service status
docker-compose ps

# Follow logs
docker-compose logs -f [service-name]

# Execute shell in container
docker-compose exec log-server bash
docker-compose exec postgres psql -U loguser -d logserver

# Stop and remove containers
docker-compose down

# Stop and remove volumes (⚠️ data loss)
docker-compose down -v
```

#### Database Management

**Reinitialize the database** (removes all data and reruns `init.sql`):

```bash
# Stop containers and remove volumes
docker-compose down --volumes

# Start fresh (init.sql will run automatically)
docker-compose up -d
```

**Connect to PostgreSQL**:

```bash
# Using docker exec
docker exec -it log-server-db psql -U loguser -d logserver

# From host (if psql client installed)
psql "postgresql://loguser:logpass123@localhost:5432/logserver"
```

**Common database operations**:

```bash
# List all schemas
docker exec -it log-server-db psql -U loguser -d logserver -c "\dt"

# Check record counts
docker exec -it log-server-db psql -U loguser -d logserver -c \
  "SELECT 'schemas' AS table, count(*) FROM schemas 
   UNION ALL 
   SELECT 'logs', count(*) FROM logs;"

# Delete all logs (keep schemas)
docker exec -it log-server-db psql -U loguser -d logserver -c \
  "TRUNCATE logs;"

# Delete a specific schema and its logs
docker exec -it log-server-db psql -U loguser -d logserver -c \
  "DELETE FROM logs WHERE schema_id = '550e8400-e29b-41d4-a716-446655440000';
   DELETE FROM schemas WHERE id = '550e8400-e29b-41d4-a716-446655440000';"
```

### Environment Variables

Create a `.env` file from the template:

```bash
cp .env.example .env
```

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `POSTGRES_PASSWORD` | PostgreSQL database password | `logpass123` | ✅ |
| `DATABASE_URL` | Full PostgreSQL connection string | Auto-generated | ❌ |
| `RUST_LOG` | Application log level | `info` | ❌ |
| `RUST_BACKTRACE` | Show backtraces on errors | `0` | ❌ |

## 📖 Documentation

- **[Software Requirements Document](docs/SRD.md)** - Project specifications and requirements
- **[OpenAPI Specification](docs/openapi.yaml)** - Complete API documentation (OpenAPI 3.0)
- **[Documentation Guide](docs/README.md)** - Documentation guidelines
- **[Scripts Guide](scripts/README.md)** - Utility scripts documentation
- **[Database Schema](docker/db/init.sql)** - PostgreSQL table definitions and indexes

### TODO Tracking

The project includes custom TODO tracking scripts:

```bash
# Find all TODOs in the repository
./scripts/todo-scanner.sh

# Generate JSON report
./scripts/todo-advanced.sh --format json

# Generate CSV report
./scripts/todo-advanced.sh --format csv --output reports/todos.csv
```

## 🛠️ Development

### Prerequisites

- **Rust 1.82+** - Latest stable Rust toolchain
- **Docker & Docker Compose** - For containerized development
- **Git** - Version control

### Local Development Setup

#### Option 1: Docker Development (Recommended)

```bash
# Start development environment with hot-reloading
docker-compose -f docker-compose.dev.yml up -d

# View development logs
docker-compose -f docker-compose.dev.yml logs -f log-server-dev

# Access development server
curl http://localhost:8081/

# Stop development environment
docker-compose -f docker-compose.dev.yml down
```

#### Option 2: Native Development

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and run
git clone <your-repo-url>
cd log-server
cargo run

# Server runs on http://localhost:8080
```

### Development Features

- **🔥 Hot Reloading** - Code changes automatically rebuild and restart
- **🐛 Debug Mode** - Enhanced logging and error traces
- **📊 Database Integration** - PostgreSQL dev database on port 5433
- **🔧 Development Tools** - cargo-watch, debugging tools included

### Project Structure

```text
log-server/
├── src/
│   ├── main.rs                # Main application entry point
│   ├── lib.rs                 # Library exports
│   ├── handlers/
│   │   ├── mod.rs             # Handler module exports
│   │   ├── schema_handlers.rs # Schema CRUD endpoints
│   │   └── log_handlers.rs    # Log CRUD endpoints
│   ├── services/
│   │   ├── mod.rs             # Service module exports
│   │   ├── schema_service.rs  # Schema business logic & validation
│   │   └── log_service.rs     # Log business logic & validation
│   ├── repositories/
│   │   ├── mod.rs             # Repository module exports
│   │   ├── schema_repository.rs # Schema database operations
│   │   └── log_repository.rs  # Log database operations
│   └── models/
│       ├── mod.rs             # Model module exports
│       ├── schema_model.rs    # Schema data model
│       └── log_model.rs       # Log data model
├── docker/
│   └── db/
│       └── init.sql           # Database initialization script
├── scripts/
│   ├── todo-scanner.sh        # Basic TODO scanner
│   ├── todo-advanced.sh       # Advanced TODO scanner with formats
│   └── README.md             # Scripts documentation
├── docs/
│   ├── README.md             # Documentation guidelines
│   ├── SRD.md                # Software Requirements Document
│   └── openapi.yaml          # OpenAPI 3.0 specification
├── Cargo.toml                # Rust dependencies
├── Dockerfile                # Production container build
├── Dockerfile.dev            # Development container build
├── docker-compose.yml        # Production Docker Compose
├── docker-compose.dev.yml    # Development Docker Compose
└── .env.example             # Environment template
```

## 🎯 Project Status

### ✅ Implemented

- [x] **HTTP Server** - Axum-based async web server with JSON support
- [x] **Database Integration** - PostgreSQL with sqlx for async queries
- [x] **Schema Management** - Full CRUD API for JSON schemas
- [x] **Schema Validation** - JSON Schema Draft 7 validation
- [x] **Log Management** - Full CRUD API for log entries
- [x] **Log Validation** - Automatic validation against schemas
- [x] **Health Endpoints** - Server health check endpoints
- [x] **JSONB Storage** - Efficient JSON storage and indexing
- [x] **Docker Deployment** - Production and development containers
- [x] **Structured Logging** - Comprehensive tracing with tracing-subscriber
- [x] **CORS Support** - Cross-origin resource sharing
- [x] **Error Handling** - Proper HTTP status codes and error responses

### 🚧 In Progress

- [ ] **Query Filtering** - Advanced log filtering by time range, fields
- [ ] **Pagination** - Efficient pagination for large result sets
- [ ] **Authentication** - API key-based authentication
- [ ] **Rate Limiting** - Request throttling and abuse prevention

### 📋 Planned Features

- [ ] **Web UI** - Browser-based schema and log management
- [ ] **Bulk Operations** - Batch log ingestion
- [ ] **Log Aggregation** - Statistics and analytics endpoints
- [ ] **Export Functionality** - Export logs in various formats (CSV, JSON)
- [ ] **Webhooks** - Real-time notifications for log events
- [ ] **Multi-tenancy** - Support for multiple isolated workspaces
- [ ] **Monitoring Dashboard** - Metrics and observability UI
- [ ] **Performance Optimization** - Caching, connection pooling tuning

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run with coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out html

# Test Docker builds
docker build -t log-server .
docker build -f Dockerfile.dev -t log-server:dev .
```

### Manual API Testing

```bash
# Health check
curl http://localhost:8080/health

# Create a schema
curl -X POST http://localhost:8080/schemas \
  -H "Content-Type: application/json" \
  -d @temperature.schema.json

# List all schemas
curl http://localhost:8080/schemas

# Create a log entry
curl -X POST http://localhost:8080/logs \
  -H "Content-Type: application/json" \
  -d '{
    "schema_id": "550e8400-e29b-41d4-a716-446655440000",
    "log_data": {
      "level": "INFO",
      "message": "Test log",
      "request_id": "test-123"
    }
  }'

# Get logs for a schema
curl http://localhost:8080/logs/schema/web-server-logs
```

## 🔧 Technology Stack

- **Language**: Rust 1.82+ (2021 edition)
- **Web Framework**: Axum 0.8 (async, type-safe routing)
- **Database**: PostgreSQL 16 Alpine with JSONB support
- **Database Driver**: sqlx 0.8 (async, compile-time checked queries)
- **Validation**: jsonschema 0.33 (JSON Schema Draft 7)
- **Serialization**: serde + serde_json
- **Logging**: tracing + tracing-subscriber
- **Containerization**: Docker with multi-stage builds
- **Development**: cargo-watch for hot reloading
- **Middleware**: tower + tower-http (tracing, CORS)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Add/update tests as needed
5. Run `cargo fmt` and `cargo clippy`
6. Update documentation if needed
7. Commit your changes: `git commit -m 'Add amazing feature'`
8. Push to the branch: `git push origin feature/amazing-feature`
9. Open a Pull Request

### Development Guidelines

- **Code Style**: Follow Rust conventions, use `cargo fmt`
- **Linting**: Ensure `cargo clippy` passes without warnings
- **Testing**: Add tests for new functionality
- **Documentation**: Update docs for API/behavior changes
- **TODOs**: Use the format `@{type}(author): description` for tracking

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔍 Troubleshooting

### Database Connection Issues

**Problem**: "Failed to connect to database"

```bash
# Check if PostgreSQL container is running
docker-compose ps

# Check PostgreSQL logs
docker-compose logs postgres

# Verify connection from host
docker exec -it log-server-db psql -U loguser -d logserver -c "SELECT 1;"

# Restart services
docker-compose restart
```

### Schema Validation Errors

**Problem**: Log entry rejected with validation error

- Ensure your log data matches the schema's `required` fields
- Check that field types match (string, number, boolean, etc.)
- Verify enum values are exact matches
- Use `jsonschema` online validators to test your schema

### Build Errors

**Problem**: Rust compilation errors

```bash
# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update

# Check Rust version (requires 1.82+)
rustc --version
```

### Docker Issues

**Problem**: Containers won't start or show errors

```bash
# Remove all containers and volumes (⚠️ deletes data)
docker-compose down -v

# Rebuild from scratch
docker-compose build --no-cache
docker-compose up -d

# Check disk space
df -h
```

### Performance Issues

**Problem**: Slow query performance

```bash
# Check database indexes
docker exec -it log-server-db psql -U loguser -d logserver -c "\di"

# Vacuum and analyze
docker exec -it log-server-db psql -U loguser -d logserver -c "VACUUM ANALYZE;"

# Check connection pool settings in your DATABASE_URL
```

## 💡 Support

For questions, issues, or contributions:

- **Issues**: [GitHub Issues](https://github.com/Milo46/log-server/issues)
- **Documentation**: Check the `docs/` directory
- **Scripts**: See `scripts/README.md` for utility documentation

---

Built with ❤️ using **Rust**, **Axum**, **PostgreSQL**, and **Docker**
