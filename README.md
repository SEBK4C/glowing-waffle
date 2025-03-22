# Glowing Waffle

[![Rust CI/CD](https://github.com/sebk4c/glowing-waffle/actions/workflows/rust.yml/badge.svg)](https://github.com/sebk4c/glowing-waffle/actions/workflows/rust.yml)
[![Release](https://github.com/sebk4c/glowing-waffle/actions/workflows/release.yml/badge.svg)](https://github.com/sebk4c/glowing-waffle/actions/workflows/release.yml)
[![Benchmark](https://github.com/sebk4c/glowing-waffle/actions/workflows/benchmark.yml/badge.svg)](https://github.com/sebk4c/glowing-waffle/actions/workflows/benchmark.yml)
[![codecov](https://codecov.io/gh/sebk4c/glowing-waffle/branch/main/graph/badge.svg)](https://codecov.io/gh/sebk4c/glowing-waffle)
[![Crates.io](https://img.shields.io/crates/v/glowing-waffle.svg)](https://crates.io/crates/glowing-waffle)
[![Documentation](https://docs.rs/glowing-waffle/badge.svg)](https://docs.rs/glowing-waffle)

A terminal-based 3D ASCII art visualizer that renders a glowing waffle in semi-3D motion, implemented in Rust for optimal performance.

## Project Description

Glowing Waffle is a fun terminal visualization that uses ASCII characters to render a 3D waffle with a "glowing" effect by manipulating character density, colors, and animations. The project leverages Rust's performance and the `crossterm` library for terminal manipulation to create a visually appealing and performant experience.

## Features

- Real-time rendering of a 3D waffle in the terminal
- ASCII art visualization with depth perception
- "Glowing" effect through color manipulation
- Smooth animations and rotations
- Performance benchmarking functionality
- Configurable rendering options

## Prerequisites

- Rust (latest stable version)
- Cargo package manager
- A terminal that supports ANSI colors and Unicode characters

## Installation

```bash
# Install from crates.io
cargo install glowing-waffle

# Or clone the repository
git clone https://github.com/sebk4c/glowing-waffle.git
cd glowing-waffle

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Usage

```bash
# Run with default settings
glowing-waffle

# Run with benchmarking enabled
glowing-waffle --benchmark

# Run with custom settings
glowing-waffle --fps 60 --color rainbow --size large
```

## Project Structure

```
glowing-waffle/
├── src/
│   ├── main.rs          # Application entry point
│   ├── renderer.rs      # ASCII rendering engine
│   ├── waffle.rs        # Waffle model and transformations
│   ├── animation.rs     # Animation and movement logic
│   ├── benchmark.rs     # Performance benchmarking utilities
│   └── terminal.rs      # Terminal interaction utilities
├── tests/               # Unit and integration tests
├── examples/            # Example configurations and usages
├── benches/             # Benchmarking suite
├── Cargo.toml           # Project dependencies and metadata
└── README.md            # This file
```

## CI/CD Pipeline

This project uses GitHub Actions for continuous integration and deployment:

- **Rust CI/CD**: Runs checks, tests, linting, formatting, documentation generation, and benchmarks on each push and pull request.
- **Release**: Creates binaries for multiple platforms when a new version is tagged and pushes the package to crates.io.
- **Benchmark**: Runs performance benchmarks and tracks results over time.

### Required GitHub Secrets

The CI/CD pipeline requires the following secrets to be set in your repository settings:

- **PAT_TOKEN**: A Personal Access Token with `repo` scope for deploying to GitHub Pages (documentation and benchmarks)
- **CODECOV_TOKEN**: Token from [Codecov](https://codecov.io) for uploading coverage reports
- **CRATES_IO_TOKEN**: Token from [crates.io](https://crates.io) for publishing releases

To set up these secrets:
1. Go to your repository Settings → Secrets and variables → Actions
2. Click "New repository secret" and add each of the above

### Latest Build Status

The CI/CD pipeline ensures code quality and tracks performance:

- [View latest builds](https://github.com/sebk4c/glowing-waffle/actions)
- [View benchmark results](https://sebk4c.github.io/glowing-waffle/dev/bench/)
- [View code coverage report](https://codecov.io/gh/sebk4c/glowing-waffle)

### Release Process

Releases are automatically created when a version tag is pushed:

1. Create a new tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
2. Push the tag: `git push origin v0.1.0`
3. The CI/CD pipeline will:
   - Create a GitHub release
   - Build binaries for Linux, macOS, and Windows
   - Upload binaries as assets to the release
   - Publish the package to crates.io

## Development Roadmap

See [TODO.md](TODO.md) for the step-by-step development plan.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
