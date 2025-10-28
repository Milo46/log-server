# Documentation

This directory contains the technical documentation for the Log Server project.

## 📁 Documentation Files

* **[`SRD.md`](./SRD.md)** - Software Requirements Document (project specifications)
* **[`openapi.yaml`](./openapi.yaml)** - OpenAPI 3.0 API specification (planned)

## 🚀 Current Status

### ✅ Available Documentation

- **Software Requirements Document** - Complete project specifications and requirements
- **README files** - Setup, usage, and development guides
- **Scripts documentation** - Utility scripts and TODO tracking
- **Contributing guidelines** - Development workflow and standards

### 🚧 In Development

- **OpenAPI Specification** - Will be created as API endpoints are implemented
- **Database Schema Documentation** - PostgreSQL schema and models
- **Deployment Guides** - Production deployment and scaling

## 🔍 Future API Documentation

When the API is implemented, documentation will include:

### Interactive Documentation

1. **Swagger Editor**: Upload [`openapi.yaml`](./openapi.yaml) to [editor.swagger.io](https://editor.swagger.io/)
2. **VS Code**: Use "OpenAPI (Swagger) Editor" extension
3. **Local Swagger UI**: `npx swagger-ui-serve openapi.yaml`
4. **Redoc**: `npx redoc-cli serve openapi.yaml`

### Planned API Features

- **Schema Management** - CRUD operations for log schemas
- **Log Ingestion** - Validated log entry creation
- **Query API** - Log filtering and search capabilities
- **Health Checks** - Server and database health monitoring

## 🔄 Documentation Workflow

When making changes:

1. **Update documentation first** - Design-first approach
2. **Keep examples current** - Ensure code examples work
3. **Update related docs** - Maintain consistency across files
4. **Version appropriately** - Update version numbers when needed

## 📖 Documentation Standards

### Code Examples

All code examples should be:

- **Tested and working** - Examples should actually run
- **Current** - Match the actual implementation
- **Clear** - Include necessary context and explanations

### Writing Style

- **Clear and concise** - Easy to understand
- **Consistent terminology** - Use same terms throughout
- **Well-structured** - Logical organization with headers
* Interactive examples and testing support

## 🛠️ Documentation Tools

* [OpenAPI Specification](https://spec.openapis.org/oas/v3.0.3) - API documentation standard
* [Swagger Tools](https://swagger.io/tools/) - OpenAPI toolchain
* [Redoc](https://redoc.ly/) - Beautiful API documentation
* [JSON Schema](https://json-schema.org/) - Schema validation specification
