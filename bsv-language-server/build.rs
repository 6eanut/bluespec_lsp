use std::path::Path;

fn ensure_parser_files() {
    let bsv_c_path = Path::new("src/tree_sitter_bsv.c");
    let tree_sitter_dir = Path::new("src/tree_sitter");

    let ts_bsv_src = Path::new("../tree-sitter-bsv/src");
    let ts_bsv_parser = ts_bsv_src.join("parser.c");
    let ts_bsv_headers = ts_bsv_src.join("tree_sitter");

    // If already generated, check freshness against source parser.c
    if bsv_c_path.exists() && tree_sitter_dir.exists() {
        if let (Ok(src_meta), Ok(dst_meta)) = (
            std::fs::metadata(&ts_bsv_parser),
            std::fs::metadata(bsv_c_path),
        ) {
            if let (Ok(src_mod), Ok(dst_mod)) = (src_meta.modified(), dst_meta.modified()) {
                if dst_mod >= src_mod {
                    // Target is up-to-date or newer than source; nothing to do
                    return;
                }
                // Source parser.c is newer (e.g. grammar.js was modified and
                // tree-sitter was re-run); fall through to re-copy.
            }
        }
    }

    // Ensure tree-sitter-bsv/src/parser.c exists; generate if missing
    if !ts_bsv_parser.exists() {
        println!("cargo:warning=tree-sitter-bsv parser not found, generating with tree-sitter...");

        let version_check = std::process::Command::new("tree-sitter")
            .arg("--version")
            .output();

        if version_check.is_err() {
            eprintln!("Error: tree-sitter-cli not found.");
            eprintln!("Install it with: cargo install tree-sitter-cli");
            eprintln!("              or: npm install -g tree-sitter-cli");
            std::process::exit(1);
        }

        let status = std::process::Command::new("tree-sitter")
            .arg("generate")
            .current_dir("../tree-sitter-bsv")
            .status()
            .expect("Failed to run tree-sitter generate");

        if !status.success() {
            eprintln!("Error: tree-sitter generate failed in tree-sitter-bsv/");
            std::process::exit(1);
        }

        if !ts_bsv_parser.exists() {
            eprintln!("Error: tree-sitter generate did not produce parser.c");
            std::process::exit(1);
        }
    }

    // Copy parser.c → src/tree_sitter_bsv.c
    std::fs::copy(&ts_bsv_parser, bsv_c_path)
        .expect("Failed to copy parser.c to src/tree_sitter_bsv.c");

    // Copy tree_sitter/ headers → src/tree_sitter/
    if tree_sitter_dir.exists() {
        std::fs::remove_dir_all(tree_sitter_dir)
            .expect("Failed to remove existing src/tree_sitter/");
    }
    copy_dir(&ts_bsv_headers, tree_sitter_dir)
        .expect("Failed to copy tree_sitter headers");
}

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir(&entry.path(), &dst_path)?;
        } else {
            std::fs::copy(entry.path(), &dst_path)?;
        }
    }
    Ok(())
}

fn main() {
    ensure_parser_files();

    // Compile tree-sitter-bsv
    cc::Build::new()
        .include("src")
        .file("src/tree_sitter_bsv.c")
        .compile("tree-sitter-bsv");

    // Rebuild triggers — also monitor source files so that grammar.js
    // changes are detected and cargo re-runs this build script.
    println!("cargo:rerun-if-changed=../tree-sitter-bsv/grammar.js");
    println!("cargo:rerun-if-changed=../tree-sitter-bsv/src/parser.c");
    println!("cargo:rerun-if-changed=src/tree_sitter_bsv.c");
    println!("cargo:rerun-if-changed=src/tree_sitter/alloc.h");
    println!("cargo:rerun-if-changed=src/tree_sitter/array.h");
    println!("cargo:rerun-if-changed=src/tree_sitter/parser.h");
}
