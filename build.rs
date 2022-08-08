fn main() {
    println!("cargo:rerun-if-changed=src/term/lib.c");
    cc::Build::new()
        .file("src/term/lib.c")
        .compile("term");
}
