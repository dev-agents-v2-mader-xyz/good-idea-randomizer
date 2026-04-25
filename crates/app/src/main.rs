// App agent fills in Tauri-specific setup here.
// The Yew App component from crates/ui is shared with the web frontend.

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Tauri wraps the Yew UI in a native WebView.
    // App agent replaces this stub with tauri::Builder setup.
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    yew::Renderer::<ui::app::App>::new().render();
}
