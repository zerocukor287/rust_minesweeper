fn main() {
    println!("cargo:rerun-if-changed=app-name-manifest.rc");
    // Compile and link checksums.rc
    embed_resource::compile("src/minesweeper.rc", embed_resource::NONE).manifest_optional().unwrap();
}