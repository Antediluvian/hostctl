# Just command file - Modern build tool

# Default task
default:
    build

# Development build
dev:
    cargo build

# Release build
build:
    cargo build --release

# Run tests
test:
    cargo test

# Run all checks
check:
    cargo check
    cargo clippy
    cargo fmt -- --check

# Code formatting
fmt:
    cargo fmt

# Clippy check
clippy:
    cargo clippy -- -D warnings

# Clean build artifacts
clean:
    cargo clean

# Install to system path
install:
    cargo install --path .

# Create new environment
create-env *env_name:
    cargo run -- create {{env_name}}

# Switch to specified environment
switch-env *env_name:
    cargo run -- switch {{env_name}}

# List all environments
list-envs:
    cargo run -- list

# Show current environment
current-env:
    cargo run -- current

# Cross-platform build
cross-build:
    # Linux
    cargo build --release --target x86_64-unknown-linux-gnu
    # macOS
    cargo build --release --target x86_64-apple-darwin
    # Windows
    cargo build --release --target x86_64-pc-windows-msvc