# Contributing to Commi

Thank you for your interest in contributing to Commi! This document provides guidelines and information for contributors.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git 2.0 or later
- A Google Gemini AI API key for testing

### Development Setup

1. **Fork and clone the repository**

   ```bash
   git clone https://github.com/Mahmoud-Emad/commi.git
   cd commi
   ```

2. **Install dependencies**

   ```bash
   cargo build
   ```

3. **Set up environment**

   ```bash
   export COMMI_API_KEY="your-test-api-key"
   ```

4. **Run tests**

   ```bash
   cargo test
   ```

## Development Workflow

### Branch Strategy

- `main` - Stable release branch
- `develop` - Development branch for new features
- `feature/feature-name` - Feature branches
- `fix/bug-description` - Bug fix branches

### Making Changes

1. **Create a feature branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Follow Rust coding conventions
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes**

   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

4. **Commit your changes**

   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

5. **Push and create a pull request**

   ```bash
   git push origin feature/your-feature-name
   ```

### Commit Message Format

We follow the [Conventional Commits](https://conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:

```
feat(cli): add new status command
fix(config): resolve API key validation issue
docs: update installation instructions
```

## Code Guidelines

### Rust Style

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Use `cargo fmt` to format code
- Address all `cargo clippy` warnings
- Write comprehensive tests for new features

### Testing

- Add unit tests for new functions
- Add integration tests for CLI features
- Ensure all tests pass before submitting PR
- Aim for high test coverage

### Documentation

- Document all public APIs
- Update README.md for user-facing changes
- Add examples for new features
- Update man page for CLI changes

## Pull Request Process

1. **Ensure CI passes**
   - All tests must pass
   - No clippy warnings
   - Code must be formatted

2. **Update documentation**
   - Update relevant documentation
   - Add changelog entry if needed

3. **Request review**
   - Assign reviewers
   - Respond to feedback promptly
   - Make requested changes

4. **Merge requirements**
   - At least one approval from maintainer
   - All CI checks passing
   - Up-to-date with target branch

## Issue Guidelines

### Bug Reports

Include:

- Commi version (`commi --version`)
- Operating system and version
- Steps to reproduce
- Expected vs actual behavior
- Error messages or logs

### Feature Requests

Include:

- Clear description of the feature
- Use case and motivation
- Proposed implementation (if any)
- Alternatives considered

## Release Process

Releases are handled by maintainers:

1. Version bump in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create release tag
4. Automated CI builds and publishes

## Getting Help

- **Documentation**: Check [docs/](docs/) directory
- **Discussions**: Use [GitHub Discussions](https://github.com/Mahmoud-Emad/commi/discussions)
- **Issues**: Search existing issues before creating new ones

## Recognition

Contributors are recognized in:

- `CHANGELOG.md` for significant contributions
- GitHub contributors page
- Release notes for major features

Thank you for contributing to Commi!

## License

By contributing to Commi, you agree that your contributions will be licensed under the MIT License.
