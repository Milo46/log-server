# Testing Directory

This directory contains all testing-related configuration and Docker files for the log-server project.

## Structure

```
testing/
├── Dockerfile.test-runner    # Docker image for running integration tests
├── config.env               # Environment variables and configuration
└── README.md               # This file
```

## Files

### `Dockerfile.test-runner`
- Builds a containerized test environment with Rust and system dependencies
- Compiles the test suite and provides a clean execution environment
- Avoids host system dependency issues (like SSL linking problems)

### `config.env`
- Centralized configuration for all test environment settings
- URLs, timeouts, Docker image names, and network configuration
- Source this file to use consistent settings across scripts

## Usage

The testing infrastructure is used by `scripts/test.sh`:

```bash
# Uses testing/Dockerfile.test-runner
./scripts/test.sh full

# Individual commands
./scripts/test.sh setup
./scripts/test.sh test
./scripts/test.sh cleanup
```

## Customization

Edit `config.env` to modify:
- Test timeouts
- Network configurations  
- Docker image names
- Environment URLs

This keeps all testing configuration in one place rather than hardcoded in scripts.
