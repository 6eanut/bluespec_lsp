# BSV Language Server - Testing Guide

This document describes how to run and extend the test suite for the BSV Language Server.

## Prerequisites

- Rust toolchain (1.70+)
- Cargo
- tree-sitter CLI (optional, for regenerating grammar)

## Quick Start

```bash
cd bsv-language-server

# Run all tests
cargo test

# Run with detailed output
cargo test -- --nocapture

# Run specific test
cargo test test_extract_module_with_broken_endmodule

# Run in release mode (faster)
cargo test --release
```

Or use the provided script:

```bash
./scripts/run_tests.sh
./scripts/run_tests.sh --verbose
./scripts/run_tests.sh --release
```

## Test Cases

The test suite covers the following scenarios:

### 1. `test_extract_module_and_function_symbols`
**Purpose**: Baseline test for syntactically correct code

**What it tests**:
- Module extraction from `moduleDef` nodes
- Function extraction from `functionDef` nodes
- Variable extraction from `varDecl` nodes

**Expected**: All symbols should be correctly extracted.

### 2. `test_extract_module_with_broken_endmodule`
**Purpose**: Test error recovery when `endmodule` is misspelled

**Input**:
```bsv
module mkTest();
    // test logic
endmodulex  // <-- typo
```

**Expected**: Module `mkTest` should still be extracted.

### 3. `test_extract_multiple_modules_with_errors`
**Purpose**: Test extraction when one module has errors but others are fine

**Input**:
```bsv
module mkA(); endmodule
module mkB(); endmodulex  // <-- typo
module mkC(); endmodule
```

**Expected**: All three modules (`mkA`, `mkB`, `mkC`) should be extracted.

### 4. `test_extract_function_with_broken_module`
**Purpose**: Test function extraction after a broken module

**Input**:
```bsv
module mkTest(); endmodulex
function Bit#(32) add(Bit#(32) a, Bit#(32) b);
    return a + b;
endfunction
```

**Expected**: Both `mkTest` (module) and `add` (function) should be extracted.

### 5. `test_missing_endmodule_entirely`
**Purpose**: Test recovery when `endmodule` is completely missing

**Input**:
```bsv
module mkTest();
```

**Expected**: Module `mkTest` should still be extracted.

### 6. `test_performance_large_file`
**Purpose**: Verify parsing performance

**Input**: 100 modules with correct syntax

**Expected**:
- All 100 modules extracted
- Execution time < 100ms

## Test Fixtures

Test fixture files are located in `test_fixtures/`:

- `correct.bsv` - Syntactically correct BSV code
- `broken.bsv` - BSV code with intentional syntax errors
- `constants.bsv` - #define constant definitions for testing constant expansion

These can be used for manual testing or integration tests.

## Constant Expansion Tests

The constant expansion module (`src/constant_expansion/`) has comprehensive tests for:

### Supported Type Functions

| Function | Description | Example |
|----------|-------------|--------|
| `TAdd#(a, b)` | Addition | `TAdd#(2, 3) = 5` |
| `TSub#(a, b)` | Subtraction | `TSub#(10, 3) = 7` |
| `TMul#(a, b)` | Multiplication | `TMul#(4, 5) = 20` |
| `TDiv#(a, b)` | Division | `TDiv#(20, 4) = 5` |
| `TLog#(n)` | Log base 2 | `TLog#(256) = 8` |
| `TExp#(n)` | 2^n | `TExp#(3) = 8` |
| `TMax#(a, b)` | Maximum | `TMax#(5, 10) = 10` |
| `TMin#(a, b)` | Minimum | `TMin#(5, 10) = 5` |

### Running Constant Expansion Tests

```bash
# Run all constant expansion tests
cargo test constant_expansion

# Run with verbose output
cargo test constant_expansion -- --nocapture
```

### Example: Hover Behavior

When hovering over a constant like `BAR` in:

```bsv
#define 2 FOO;
#define TAdd#(FOO, 1) BAR;
```

The hover will show:

```
**BAR** = `3`

```
BAR = TAdd#(FOO, 1)
|- FOO = 2
|- Result: 3
```
```

### Test Cases for Constant Expansion

The test suite includes tests for:

1. **Simple numeric constants** - Direct numeric values
2. **Type function constants** - TAdd, TSub, TMul, TDiv, TLog, TExp, TMax, TMin
3. **Nested expansion** - Constants defined in terms of other constants
4. **Complex nested expansion** - Multiple levels of nesting
5. **Circular reference detection** - Error handling for circular definitions
6. **Undefined constant handling** - Error handling for missing definitions
7. **Position-based lookup** - Finding constants at cursor position

## How Error Recovery Works

When the tree-sitter parser encounters syntax errors:

1. **ERROR nodes** are generated in the parse tree
2. The symbol extractor traverses ERROR nodes looking for:
   - `module` keyword followed by an identifier
   - `functionProto` nodes containing function names
   - `varDecl` nodes that might represent function definitions
3. Extracted symbols are deduplicated to avoid duplicates

## Adding New Tests

To add a new test:

1. Add a test function in `src/parser.rs` under `#[cfg(test)]`
2. Use the existing test pattern:

```rust
#[test]
fn test_your_new_scenario() {
    let source = "your BSV code here";
    let parser = BsvParser::default();
    let tree = parser.parse(source).expect("parse failed");
    let symbols = parser.extract_symbols(&tree, source);
    
    assert!(symbols.iter().any(|s| s.name == "expected_symbol"));
}
```

3. Run the test: `cargo test test_your_new_scenario`

## Debugging Failed Tests

If tests fail:

1. Print the extracted symbols:
```rust
for s in &symbols {
    println!("{:?}: {} at {:?}", s.kind, s.name, s.range);
}
```

2. Check the parse tree:
```rust
println!("{:#?}", tree.root_node().to_sexp());
```

3. Run with verbose output:
```bash
cargo test -- --nocapture
```

## CI Integration

For CI pipelines:

```yaml
- name: Run tests
  run: |
    cd bsv-language-server
    cargo test --release
```

## Related Documentation

- [Error Recovery Implementation](../docs/error-recovery.md)
- [Tree-sitter Documentation](https://tree-sitter.github.io/tree-sitter/)
- [BSV Language Reference](https://github.com/B-Lang-org/bsc)
