# Log Server

A lightweight HTTP server built with **Rust** and **Axum** framework, featuring Docker containerization and PostgreSQL database integration. Currently implements basic REST endpoints with plans for schema-driven logging capabilities.

## 🚀 Current Features

- **Rust/Axum HTTP Server**: Fast, async web server with JSON responses
- **Docker Containerization**: Production and development Docker environments
- **PostgreSQL Integration**: Database setup with Docker Compose
- **Development Tools**: Hot-reloading, debugging, and live development setup
- **TODO Tracking**: Custom scripts for tracking code annotations
- **Multi-stage Builds**: Optimized production Docker images

## 📋 Quick Start

### 1. Using Docker Compose (Recommended)

```bash
# Clone the repository
git clone <your-repo-url>
cd log-server

# Copy and configure environment
cp .env.example .env
# Edit .env with your database password

# Start production services
docker-compose up -d

# Test the server
curl http://localhost:8080/
# Returns: {"message":"Hello from /"}

curl http://localhost:8080/user/123
# Returns: {"message":"Hello user with id 123"}
```

### 2. Development Setup

```bash
# Start development environment with hot-reloading
docker-compose -f docker-compose.dev.yml up -d

# View logs with live updates
docker-compose -f docker-compose.dev.yml logs -f log-server-dev

# Development server runs on http://localhost:8081
curl http://localhost:8081/
```

### 3. Local Development (without Docker)

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run the server locally
cargo run

# Server runs on http://localhost:8080
```

## 🏗️ Architecture

```text
┌─────────────────┐    HTTP/JSON    ┌──────────────────┐
│   Clients       │ ──────────────► │   Log Server     │
│   (curl, apps)  │                 │   (Rust/Axum)    │
└─────────────────┘                 └──────────────────┘
                                             │
                                             │ (planned)
                                             ▼
                                    ┌──────────────────┐
                                    │   PostgreSQL     │
                                    │   Database       │
                                    └──────────────────┘
```

### Current Components

- **HTTP Server**: Rust-based async server using Axum framework
- **Containerization**: Docker and Docker Compose setup
- **Database**: PostgreSQL 16 Alpine with initialization scripts
- **Development Tools**: Hot-reloading with cargo-watch

### Planned Components

- **Schema Validation**: JSON Schema validation for log entries
- **Database Integration**: Active PostgreSQL connection and models
- **RESTful API**: Complete CRUD operations for logs and schemas

## 📚 Current API Endpoints

| Method | Endpoint | Description | Status |
|--------|----------|-------------|--------|
| GET    | `/` | Root endpoint returning welcome message | ✅ Implemented |
| GET    | `/user/{id}` | Get user info by ID | ✅ Implemented |

### Planned API Endpoints

| Method | Endpoint | Description | Status |
|--------|----------|-------------|--------|
| POST   | `/schemas` | Register a new log schema | 🚧 Planned |
| GET    | `/schemas` | Retrieve registered schemas | 🚧 Planned | 
| POST   | `/logs/{schema_id}` | Create log entry | 🚧 Planned |
| GET    | `/logs` | Retrieve log entries | 🚧 Planned |
| GET    | `/health` | Health check endpoint | 🚧 Planned |

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
- **[API Documentation](docs/openapi.yaml)** - OpenAPI specification (planned)
- **[Documentation Guide](docs/README.md)** - Documentation guidelines
- **[Scripts Guide](scripts/README.md)** - Utility scripts documentation

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
│   └── main.rs              # Main application entry point
├── docker/
│   └── db/
│       └── init.sql         # Database initialization script
├── scripts/
│   ├── todo-scanner.sh      # Basic TODO scanner
│   ├── todo-advanced.sh     # Advanced TODO scanner with formats
│   └── README.md           # Scripts documentation
├── docs/
│   ├── README.md           # Documentation guidelines
│   ├── SRD.md              # Software Requirements Document
│   └── openapi.yaml        # API specification (planned)
├── Dockerfile              # Production container build
├── Dockerfile.dev          # Development container build
├── docker-compose.yml      # Production Docker Compose
├── docker-compose.dev.yml  # Development Docker Compose
└── .env.example           # Environment template
```

## 🎯 Current Status & Roadmap

### ✅ Implemented

- [x] **Basic HTTP Server** - Axum-based async web server
- [x] **Docker Containerization** - Production and development containers
- [x] **PostgreSQL Integration** - Database setup and configuration
- [x] **Development Workflow** - Hot-reloading development environment
- [x] **TODO Tracking** - Custom annotation scanning scripts
- [x] **Documentation** - Comprehensive setup and usage guides

### 🚧 In Progress

- [ ] **Database Models** - Rust structs and database schema
- [ ] **Health Endpoint** - Server and database health checks
- [ ] **Error Handling** - Proper error responses and logging

### 📋 Planned Features

- [ ] **Schema Management API** - CRUD operations for log schemas
- [ ] **Log Ingestion API** - Validated log entry creation
- [ ] **Query API** - Log filtering and search capabilities
- [ ] **JSON Schema Validation** - Schema-driven log validation
- [ ] **Web UI** - Browser-based management interface
- [ ] **Authentication** - API key management
- [ ] **Monitoring** - Metrics and observability

## 🧪 Testing

```bash
# Run tests
cargo test

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out html

# Test Docker builds
docker build -t log-server .
docker build -f Dockerfile.dev -t log-server:dev .

# Test endpoints
curl http://localhost:8080/
curl http://localhost:8080/user/123
```

## 🔧 Technology Stack

- **Backend**: Rust with Axum web framework
- **Database**: PostgreSQL 16 Alpine with JSONB support
- **Containerization**: Docker with multi-stage builds
- **Development**: cargo-watch for hot reloading
- **Future**: JSON Schema validation, OpenAPI documentation

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

## � Support

For questions, issues, or contributions:

- **Issues**: [GitHub Issues](link-to-your-repo/issues)
- **Documentation**: Check the `docs/` directory
- **Scripts**: See `scripts/README.md` for utility documentation

---

Built with ❤️ using **Rust**, **Axum**, and **Docker**
