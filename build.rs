fn main() {
    if cfg!(target_os = "windows") {
        winres::WindowsResource::new()
        .set_icon("favicon.ico")
        .set("InternalName", "MATRIX.EXE")
        .compile().unwrap();
    }
}
