fn main() {
    let src_dir = std::path::Path::new("src");

    // 确保 src 目录存在
    if !src_dir.exists() {
        std::fs::create_dir_all(src_dir).expect("Failed to create src directory");
    }

    let parser_path = src_dir.join("parser.c");

    // 如果 parser.c 不存在，尝试生成
    if !parser_path.exists() {
        println!("cargo:warning=parser.c not found, attempting to generate with tree-sitter");

        // 检查 tree-sitter-cli 是否可用
        let output = std::process::Command::new("tree-sitter")
            .arg("--version")
            .output();

        if output.is_err() {
            eprintln!("Error: tree-sitter-cli not found. Please install it with:");
            eprintln!("  cargo install tree-sitter-cli");
            eprintln!("  or npm install -g tree-sitter-cli");
            eprintln!("");
            eprintln!("After installing tree-sitter-cli, run:");
            eprintln!("  cd tree-sitter-bsv && tree-sitter generate");
            std::process::exit(1);
        }

        // 生成 parser.c
        let status = std::process::Command::new("tree-sitter")
            .arg("generate")
            .current_dir(".")
            .status()
            .expect("Failed to run tree-sitter generate");

        if !status.success() {
            eprintln!("Error: tree-sitter generate failed");
            std::process::exit(1);
        }

        if !parser_path.exists() {
            eprintln!("Error: tree-sitter generate did not create parser.c");
            eprintln!("Please check if grammar.js exists and is valid");
            std::process::exit(1);
        }
    }

    let mut c_config = cc::Build::new();
    c_config.std("c11").include(src_dir);

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    c_config.file(&parser_path);
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    // NOTE: if your language uses an external scanner, uncomment this block:
    /*
    let scanner_path = src_dir.join("scanner.c");
    c_config.file(&scanner_path);
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    */

    c_config.compile("tree-sitter-bsv");
}
