#!/bin/bash

# MediaLanguage DSL Parser Test Examples
# This script demonstrates the test capabilities of the mdsl-rs parser

echo "=== MediaLanguage DSL Parser Test Examples ==="
echo

# 1. Run regression tests (known successful files)
echo "1. Running regression tests on known successful files..."
echo "Command: cargo run --bin test_runner"
echo
(cd .. && cargo run --bin test_runner)
echo

# 2. Test a specific file
echo "2. Testing a specific file..."
echo "Command: cargo run --bin test_runner -- --file ../MediaLanguage/sources.mdsl"
echo
(cd .. && cargo run --bin test_runner -- --file ../MediaLanguage/sources.mdsl)
echo

# 3. Run all unit tests
echo "3. Running unit tests..."
echo "Command: cargo test --test unit_tests"
echo
(cd .. && cargo test --test unit_tests)
echo

# 4. Run all integration tests
echo "4. Running integration tests..."
echo "Command: cargo test --test integration_tests"
echo
(cd .. && cargo test --test integration_tests)
echo

# 5. Test all files in directory
echo "5. Testing all files in MediaLanguage directory..."
echo "Command: cargo run --bin test_runner -- --all"
echo
(cd .. && cargo run --bin test_runner -- --all)
echo

echo "=== Test Examples Complete ==="
echo
echo "Available test commands (run from mdsl-rs/ directory):"
echo "  cargo run --bin test_runner                    # Run regression tests"
echo "  cargo run --bin test_runner -- --all          # Test all files"
echo "  cargo run --bin test_runner -- --file <path>  # Test specific file"
echo "  cargo test --test unit_tests                   # Run unit tests"
echo "  cargo test --test integration_tests           # Run integration tests"
echo "  cargo test --lib                              # Run library tests"
echo
echo "Test coverage:"
echo "  - 14 unit tests covering lexer, parser components"
echo "  - 10 integration tests covering real DSL files"
echo "  - 14/17 MediaLanguage files parsing successfully (82.4% success rate)"
echo "  - Comprehensive test suite for MediaLanguage DSL constructs"
