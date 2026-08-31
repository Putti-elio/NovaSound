fn main() {
    #[cfg(target_os = "linux")]
    // Must be set before WebKitGTK starts so it does not request unsupported GBM buffers.
    unsafe {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running NovaSound");
}
