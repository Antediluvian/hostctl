#!/bin/bash

# Hostctl test runner script

echo "🧪 Hostctl Test Runner"
echo "====================="

# Check Rust environment
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo not found. Please install Rust first."
    exit 1
fi

echo "🔍 Checking Rust version..."
cargo --version

echo ""
echo "📦 Running unit tests..."
if cargo test --lib; then
    echo "✅ Unit tests passed"
else
    echo "❌ Unit tests failed"
    exit 1
fi

echo ""
echo "🔧 Running integration tests..."
if cargo test --test integration_tests; then
    echo "✅ Integration tests passed"
else
    echo "❌ Integration tests failed"
    exit 1
fi

echo ""
echo "🎯 Running all tests..."
if cargo test; then
    echo ""
    echo "🎉 All tests passed!"
    echo ""
    echo "📊 Test Summary:"
    echo "   • Unit tests: ✅"
    echo "   • Integration tests: ✅"
    echo "   • Total: ✅ All tests passed"
else
    echo ""
    echo "💥 Some tests failed"
    exit 1
fi

echo ""
echo "🔍 Additional checks..."

# Code formatting check
echo "📝 Checking code formatting..."
if cargo fmt -- --check; then
    echo "✅ Code formatting OK"
else
    echo "⚠️  Formatting issues found"
fi

# Clippy check
echo "🔍 Running Clippy checks..."
if cargo clippy -- -D warnings; then
    echo "✅ Clippy checks passed"
else
    echo "⚠️  Clippy warnings found"
fi

echo ""
echo "🏁 Testing completed!"