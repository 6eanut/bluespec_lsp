fn main() {
    // 监视 tree-sitter-bsv 的 grammar.js 变化，触发重新构建
    println!("cargo:rerun-if-changed=../tree-sitter-bsv/grammar.js");
    println!("cargo:rerun-if-changed=../tree-sitter-bsv/src/parser.c");
}
