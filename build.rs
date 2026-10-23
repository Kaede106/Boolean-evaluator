// build.rs
fn main() {
    let dir = std::path::Path::new("src");
    cc::Build::new()
        .file(dir.join("parser.c"))
        .include(dir)
        .compile("tree-sitter-boolean-algebra");

    println!("cargo:rerun-if-changed=src/parser.c");
}
