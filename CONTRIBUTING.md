# Contributing to Glowing Waffle

Thank you for your interest in contributing to Glowing Waffle! This document provides guidelines and instructions for contributing to this project.

## Code of Conduct

By participating in this project, you agree to abide by the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/glowing-waffle.git`
3. Create a branch for your changes: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Format your code: `cargo fmt --all`
7. Run clippy: `cargo clippy --all-features -- -D warnings`
8. Commit your changes: `git commit -am 'Add some feature'`
9. Push to the branch: `git push origin feature/your-feature-name`
10. Submit a pull request to the main repository at https://github.com/sebk4c/glowing-waffle

## Development Environment

Make sure you have the following installed:

- Rust (latest stable version)
- Cargo
- A terminal that supports ANSI colors and Unicode

## Running Tests

```bash
# Run all tests
cargo test

# Run specific tests
cargo test --test terminal_tests

# Run ignored tests
cargo test -- --ignored
```

## Benchmarking

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench render_benchmark
```

## Pull Request Process

1. Update the README.md and documentation with details of changes if appropriate
2. Add tests for new functionality
3. Update the TODO.md file if completing a task
4. Ensure your code passes our CI checks
5. The pull request will be merged once it is approved by a maintainer

## Coding Style

- Follow Rust's official [style guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- Use `cargo fmt` to format your code
- Use `cargo clippy` to catch common mistakes
- Write meaningful commit messages following [conventional commits](https://www.conventionalcommits.org/)

## Documentation

- Update documentation when changing code
- Add comments for complex logic
- Keep the README.md and other documentation up to date

## Issue Reporting

- Use the issue templates when creating new issues
- Search for existing issues before creating a new one
- Be descriptive and include steps to reproduce

## Feature Requests

- Use the feature request template
- Clearly describe the problem your feature solves
- Consider alternatives

## CI/CD Pipeline

Our project uses GitHub Actions for continuous integration and delivery:

- Tests run automatically on every pull request
- Code formatting and linting are enforced
- Benchmarks track performance changes
- Documentation is generated automatically

## Releases

Releases are created automatically when version tags are pushed:

```bash
# Tag a new version
git tag -a v0.1.0 -m "Release v0.1.0"

# Push the tag
git push origin v0.1.0
```

This will trigger the release workflow which builds platform-specific binaries and publishes to crates.io. 