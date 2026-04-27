use std::path::Path;

fn main() {
    let grammar_dir = Path::new("../tree-sitter-bsv");
    let src_dir = grammar_dir.join("src");
    let parser_c = src_dir.join("parser.c");

    // Auto-generate parser.c from grammar.js if missing
    if !parser_c.exists() {
        println!("cargo:warning=parser.c not found, generating with tree-sitter...");

        // Ensure src directory exists
        std::fs::create_dir_all(&src_dir).expect("Failed to create tree-sitter-bsv/src");

        let status = std::process::Command::new("tree-sitter")
            .arg("generate")
            .current_dir(grammar_dir)
            .status()
            .expect("Failed to run tree-sitter generate. Install it with: cargo install tree-sitter-cli");

        if !status.success() {
            panic!("tree-sitter generate failed");
        }

        if !parser_c.exists() {
            panic!("tree-sitter generate did not create parser.c");
        }
    }

    // Compile tree-sitter-bsv C code
    cc::Build::new()
        .include(&src_dir)
        .file(&parser_c)
        .compile("tree-sitter-bsv");

    // Re-compile triggers
    println!("cargo:rerun-if-changed={}", grammar_dir.join("grammar.js").display());
    println!("cargo:rerun-if-changed={}", parser_c.display());
}
