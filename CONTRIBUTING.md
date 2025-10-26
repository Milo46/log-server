# Contributing to Log Server

Thank you for your interest in contributing to the Log Server project! This document outlines our development processes, branching strategy, and guidelines for contributors.

## 🌳 Git Branching Strategy

We follow a **Git Flow** branching model with the following structure:

### Main Branches

- **`main`** - Production-ready code
  - Always deployable
  - Protected branch requiring pull request reviews
  - Automatically tagged for releases
  - Direct pushes are not allowed

- **`develop`** - Integration branch for features
  - Latest development changes
  - Base branch for feature development
  - Merged into `main` for releases

### Supporting Branches

- **Feature branches** - `feature/issue-number-short-description`
  - Branched from: `develop`
  - Merged back to: `develop`
  - Naming: `feature/123-add-rate-limiting`
  - Deleted after merge

- **Release branches** - `release/version-number`
  - Branched from: `develop`
  - Merged to: `main` and `develop`
  - Naming: `release/1.2.0`
  - For final release preparations

- **Hotfix branches** - `hotfix/issue-number-description`
  - Branched from: `main`
  - Merged to: `main` and `develop`
  - Naming: `hotfix/456-fix-memory-leak`
  - For critical production fixes

- **Documentation branches** - `docs/description`
  - Branched from: `develop`
  - Merged back to: `develop`
  - Naming: `docs/update-api-spec`
  - For documentation-only changes

## 🔄 Development Workflow

### 1. Setting Up Your Development Environment

```bash
# Clone the repository
git clone https://github.com/your-org/log-server.git
cd log-server

# Add upstream remote (for forks)
git remote add upstream https://github.com/your-org/log-server.git

# Switch to develop branch
git checkout develop
git pull origin develop
```

### 2. Creating a Feature Branch

```bash
# Create and switch to feature branch
git checkout -b feature/123-add-authentication develop

# Push the branch to your fork
git push -u origin feature/123-add-authentication
```

### 3. Development Process

1. **Make your changes** in small, logical commits
2. **Write tests** for new functionality
3. **Update documentation** as needed
4. **Follow coding standards** (see below)
5. **Keep commits focused** and atomic

```bash
# Make commits with descriptive messages
git add .
git commit -m "Add JWT authentication middleware

- Implement JWT token validation
- Add authentication error handling
- Update API documentation for auth endpoints
- Add tests for auth middleware"

# Push changes regularly
git push origin feature/123-add-authentication
```

### 4. Staying Up to Date

```bash
# Regularly sync with develop
git checkout develop
git pull upstream develop
git checkout feature/123-add-authentication
git rebase develop

# Or merge if rebasing isn't preferred
git merge develop
```

### 5. Creating a Pull Request

1. **Push your completed feature branch**
2. **Open a Pull Request** to `develop`
3. **Fill out the PR template** completely
4. **Request reviews** from maintainers
5. **Address feedback** promptly

## 📝 Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

### Format
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types
- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Code formatting (no logic changes)
- **refactor**: Code restructuring (no behavior changes)
- **test**: Adding or updating tests
- **chore**: Maintenance tasks

### Examples
```bash
feat(api): add schema validation endpoint

Add POST /schemas endpoint for registering JSON schemas.
Includes validation against JSON Schema Draft 7 spec.

Closes #123

fix(database): resolve connection pool exhaustion

- Increase max connection pool size
- Add connection timeout configuration
- Improve error handling for failed connections

Fixes #456

docs: update OpenAPI specification for new endpoints

- Add /schemas endpoint documentation
- Update examples to use string schema IDs
- Fix parameter descriptions
```

## 🔍 Code Review Process

### Before Requesting Review

- [ ] Code builds successfully
- [ ] All tests pass
- [ ] Documentation updated
- [ ] No merge conflicts with target branch
- [ ] Self-review completed

### Review Criteria

**Functionality**
- [ ] Code works as intended
- [ ] Edge cases handled properly
- [ ] Error handling implemented

**Code Quality**
- [ ] Follows Rust/project conventions
- [ ] Well-structured and readable
- [ ] Appropriate comments added
- [ ] No unnecessary complexity

**Testing**
- [ ] Unit tests cover new functionality
- [ ] Integration tests updated if needed
- [ ] Test cases cover error scenarios

**Documentation**
- [ ] API changes reflected in OpenAPI spec
- [ ] README updated if needed
- [ ] Code comments explain complex logic

## 🚀 Release Process

### Semantic Versioning

We follow [Semantic Versioning](https://semver.org/) (MAJOR.MINOR.PATCH):

- **MAJOR** - Breaking API changes
- **MINOR** - New features (backward compatible)
- **PATCH** - Bug fixes (backward compatible)

### Release Steps

1. **Create release branch**
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b release/1.2.0
   ```

2. **Prepare release**
   - Update version numbers
   - Update CHANGELOG.md
   - Final testing and bug fixes

3. **Complete release**
   ```bash
   # Merge to main
   git checkout main
   git merge --no-ff release/1.2.0
   git tag -a v1.2.0 -m "Release version 1.2.0"
   
   # Merge back to develop
   git checkout develop
   git merge --no-ff release/1.2.0
   
   # Clean up
   git branch -d release/1.2.0
   git push origin main develop --tags
   ```

## 🐛 Hotfix Process

For critical production issues:

```bash
# Create hotfix from main
git checkout main
git pull origin main
git checkout -b hotfix/456-fix-critical-bug

# Make the fix
# ... fix the issue ...

# Merge to main
git checkout main
git merge --no-ff hotfix/456-fix-critical-bug
git tag -a v1.1.1 -m "Hotfix version 1.1.1"

# Merge to develop
git checkout develop
git merge --no-ff hotfix/456-fix-critical-bug

# Clean up
git branch -d hotfix/456-fix-critical-bug
git push origin main develop --tags
```

## 📋 Pull Request Template

When creating a PR, include:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Code refactoring

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Documentation
- [ ] OpenAPI spec updated
- [ ] README updated (if needed)
- [ ] Code comments added

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] No merge conflicts
- [ ] Related issues linked

## Related Issues
Closes #123
Fixes #456
```

## 🔧 Development Environment Setup

### Prerequisites
- Rust 1.70+
- PostgreSQL 15+
- Docker & Docker Compose
- Git 2.20+

### Local Setup
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Start database
docker-compose up -d db

# Run tests
cargo test

# Start development server
cargo run
```

## 💡 Getting Help

- **Issues**: Use GitHub Issues for bugs and feature requests
- **Discussions**: Use GitHub Discussions for questions and ideas
- **Documentation**: Check the [docs/](docs/) directory
- **API Reference**: See [docs/openapi.yaml](docs/openapi.yaml)

## 📜 Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). Please be respectful and inclusive in all interactions.

## 🎯 Good First Issues

Look for issues labeled with:
- `good first issue` - Perfect for newcomers
- `help wanted` - Community contributions welcome
- `documentation` - Documentation improvements needed

## 📊 Project Status

- **Main Branch**: [![Build Status](https://github.com/your-org/log-server/workflows/CI/badge.svg?branch=main)](https://github.com/your-org/log-server/actions)
- **Develop Branch**: [![Build Status](https://github.com/your-org/log-server/workflows/CI/badge.svg?branch=develop)](https://github.com/your-org/log-server/actions)

Thank you for contributing to Log Server! 🎉
