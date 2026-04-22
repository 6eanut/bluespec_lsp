extern crate cc;

fn main() {
    // 编译tree-sitter-bsv
    let mut build = cc::Build::new();

    build
        .include("src")
        .file("src/tree_sitter_bsv.c");

    // 添加平台特定的编译标志
    if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        // macOS ARM64 特定标志
        build.flag("-arch").flag("arm64");
    } else if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
        // macOS Intel 特定标志
        build.flag("-arch").flag("x86_64");
    }

    build.compile("tree-sitter-bsv");

    // 重新编译的触发条件
    println!("cargo:rerun-if-changed=src/tree_sitter_bsv.c");
    println!("cargo:rerun-if-changed=src/tree_sitter_bsv.h");
}
