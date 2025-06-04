#[cfg(target_os = "windows")]
fn main() {
    println!("cargo:rerun-if-changed=src/minesweeper.rc");
    // Compile and link windows resources
    embed_resource::compile("src/minesweeper.rc", embed_resource::NONE).manifest_optional().unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {
}