#[cfg(target_os = "linux")]
fn build_linux() {
    println!("cargo:rerun-if-changed=src/term/lib.c");
    cc::Build::new()
        .file("src/term/lib.c")
        .compile("term");
}

fn main() {
    #[cfg(target_os = "linux")]
    build_linux();
}
