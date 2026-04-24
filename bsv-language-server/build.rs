extern crate cc;

fn main() {
    // 确保tree_sitter_bsv.c文件存在
    let source_c = "src/tree_sitter_bsv.c";
    if !std::path::Path::new(source_c).exists() {
        // 尝试从tree-sitter-bsv目录复制
        let ts_parser = "../tree-sitter-bsv/src/parser.c";
        if std::path::Path::new(ts_parser).exists() {
            std::fs::copy(ts_parser, source_c).expect("Failed to copy parser.c");
            println!("cargo:warning=Copied parser.c from tree-sitter-bsv");
        } else {
            panic!("tree_sitter_bsv.c not found and cannot be generated. Please run 'tree-sitter generate' in tree-sitter-bsv directory first.");
        }
    }

    // 编译tree-sitter-bsv
    cc::Build::new()
        .include("src")
        .file(source_c)
        .compile("tree-sitter-bsv");

    // 重新编译的触发条件
    println!("cargo:rerun-if-changed={}", source_c);
    // tree_sitter_bsv.h可能不存在，但如果有的话就监视它
    let source_h = "src/tree_sitter_bsv.h";
    if std::path::Path::new(source_h).exists() {
        println!("cargo:rerun-if-changed={}", source_h);
    }
}
