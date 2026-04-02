#!/bin/bash
# run_tests.sh - BSV Language Server Test Runner
#
# Usage:
#   ./run_tests.sh              # Run all tests
#   ./run_tests.sh --verbose    # Run with detailed output
#   ./run_tests.sh --release    # Run in release mode (faster)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

VERBOSE=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE="--nocapture"
            shift
            ;;
        --release|-r)
            MODE="--release"
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--verbose] [--release]"
            exit 1
            ;;
    esac
done

echo "=========================================="
echo "BSV Language Server - Test Suite"
echo "=========================================="
echo ""

# Check if tree-sitter-bsv is built
if [ ! -f "../tree-sitter-bsv/src/parser.c" ]; then
    echo "Building tree-sitter-bsv..."
    cd ../tree-sitter-bsv
    if command -v tree-sitter &> /dev/null; then
        tree-sitter generate
    else
        echo "Warning: tree-sitter CLI not found, using pre-built parser.c"
    fi
    cd "$PROJECT_DIR"
fi

echo "Building project..."
cargo build $MODE

echo ""
echo "Running tests..."
echo "------------------------------------------"

# Run specific tests for error-tolerant symbol extraction
cargo test $MODE $VERBOSE -- --test-threads=1

echo ""
echo "=========================================="
echo "Test Summary"
echo "=========================================="

# Print test descriptions
echo ""
echo "Test Cases:"
echo "  1. test_extract_module_and_function_symbols"
echo "     - Tests symbol extraction from syntactically correct BSV code"
echo ""
echo "  2. test_extract_module_with_broken_endmodule"
echo "     - Tests recovery when 'endmodule' is misspelled (e.g., 'endmodulex')"
echo ""
echo "  3. test_extract_multiple_modules_with_errors"
echo "     - Tests extraction of multiple modules when one has errors"
echo ""
echo "  4. test_extract_function_with_broken_module"
echo "     - Tests function extraction when preceding module has errors"
echo ""
echo "  5. test_missing_endmodule_entirely"
echo "     - Tests recovery when 'endmodule' is completely missing"
echo ""
echo "  6. test_performance_large_file"
echo "     - Tests parsing performance with 100 modules"
echo ""
echo "=========================================="
