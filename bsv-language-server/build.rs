extern crate cc;

fn main() {
    // 编译tree-sitter-bsv
    cc::Build::new()
        .include("src")
        .file("src/tree_sitter_bsv.c")
        .compile("tree_sitter_bsv");  // 使用带下划线的名称，匹配C函数

    // 输出链接指令 - 使用正确的库名
    println!("cargo:rustc-link-lib=static=tree_sitter_bsv");

    // 重新编译的触发条件
    println!("cargo:rerun-if-changed=src/tree_sitter_bsv.c");
    println!("cargo:rerun-if-changed=src/tree_sitter/");
}
