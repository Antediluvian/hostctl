#!/bin/bash

# Hostctl Build Script
# This script helps build the hostctl tool with latest Rust features

echo "🚀 Hostctl Build Script"
echo "======================"

# Check if Rust is installed and up to date
if ! command -v rustc &> /dev/null; then
    echo "❌ Error: Rust is not installed."
    echo "📥 Please install Rust from https://rustup.rs/"
    echo "💡 Then run: source $HOME/.cargo/env"
    exit 1
fi

# Check Rust version
echo "🔍 Checking Rust version..."
rustc --version

# Update to latest stable if needed
echo "🔄 Updating Rust toolchain..."
if ! rustup update stable; then
    echo "⚠️  Rust toolchain update failed, continuing with current version..."
fi

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo (Rust package manager) is not available."
    echo "⚠️  Please ensure Rust is properly installed."
    exit 1
fi

# Check for just command runner (optional but recommended)
if command -v just &> /dev/null; then
    echo "📋 Just command runner detected - using it for enhanced workflow"
    echo "💡 Available commands: just --list"
fi

echo "🔨 Building hostctl with latest dependencies..."

# Update dependencies first
echo "📦 Updating dependencies..."
if ! cargo update; then
    echo "⚠️  Dependency update failed, continuing with current versions..."
fi

# Run code quality checks
echo "🔍 Running code quality checks..."
cargo fmt -- --check || echo "⚠️  Formatting issues found"
cargo clippy -- -D warnings || echo "⚠️  Clippy warnings found"

# Build the project with latest optimizations
echo "🏗️  Building release version..."
if cargo build --release; then
    echo ""
    echo "✅ Build successful!"
    echo "📁 The binary is available at: target/release/hostctl"
    echo ""
    echo "🌍 Installation options:"
    echo "   cargo install --path .          # Install globally"
    echo "   just install                    # Using just command runner"
    echo ""
    echo "💡 Quick start examples:"
    echo "   hostctl list                    # List environments"
    echo "   hostctl create dev              # Create dev environment"
    echo "   hostctl switch dev              # Switch to dev environment"
    echo ""
    echo "🔧 Development tools:"
    echo "   just test                       # Run tests"
    echo "   just fmt                        # Format code"
    echo "   just clippy                     # Run clippy checks"
    echo "   just check                      # Run all checks"
else
    echo ""
    echo "❌ Build failed. Please check the error messages above."
    exit 1
fi
