//! Integration tests for the Find References feature using real BSV fixture files.
//!
//! These tests verify that the reference extraction correctly identifies identifier
//! usage sites and distinguishes them from declaration sites.

use bsv_language_server::{BsvParser, Reference};

/// Path helper: returns the full path to a fixture file.
fn fixture_path(name: &str) -> std::path::PathBuf {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test_fixtures");
    path.push(name);
    path
}

/// Parse a fixture file and extract references.
fn collect_fixture_references(name: &str) -> Vec<Reference> {
    let source = std::fs::read_to_string(fixture_path(name))
        .unwrap_or_else(|e| panic!("Failed to read fixture '{}': {}", name, e));

    let parser = BsvParser::default();
    let tree = parser.parse(&source).expect("parse should succeed");

    parser.extract_references(&tree, &source)
}

#[test]
fn test_references_fixture_parse() {
    let refs = collect_fixture_references("references.bsv");

    // Verify key expected references exist
    assert!(
        refs.iter().any(|r| r.name == "Vector"),
        "Vector should be a reference (import)"
    );
    assert!(
        refs.iter().any(|r| r.name == "mkReg"),
        "mkReg should be a reference"
    );
    assert!(
        refs.iter()
            .any(|r| r.name == "mkHello" && r.range.start.line == 18),
        "mkHello on line 18 should be a reference (module instance)"
    );

    // Verify key non-references (declarations)
    assert!(
        !refs.iter().any(|r| r.name == "TestRefs"),
        "TestRefs is the package name, not a reference"
    );
    assert!(
        !refs.iter().any(|r| r.name == "hello"),
        "hello is a rule name, not a reference"
    );
}

#[test]
fn test_references_correct_fixture() {
    let refs = collect_fixture_references("correct.bsv");

    // correct.bsv has module instance reference: mkTest on line 19
    assert!(
        refs.iter()
            .any(|r| r.name == "mkTest" && r.range.start.line == 19),
        "mkTest on line 19 should be a module instance reference"
    );

    // Should NOT find exported names as references
    assert!(
        !refs
            .iter()
            .any(|r| r.name == "mkMain" && r.range.start.line == 7),
        "exportItem mkMain is not a reference"
    );
}

#[test]
fn test_broken_fixture_references() {
    // Even with syntax errors, references should still be extractable
    let refs = collect_fixture_references("broken.bsv");

    // broken.bsv has mkReg and mkTest references
    assert!(
        refs.iter().any(|r| r.name == "mkReg"),
        "mkReg should be a reference even in broken code"
    );
}
