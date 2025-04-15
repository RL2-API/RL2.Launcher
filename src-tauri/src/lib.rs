// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn hide_window(window: tauri::Window) {
    let _ = window.minimize();
}

#[tauri::command]
fn close_window(window: tauri::Window) {
    let _ = window.close();
}

#[tauri::command]
fn maximize_window(window: tauri::Window) {
    if let Ok(maximized) = window.is_maximized() {
        let _ = match maximized {
            true => window.unmaximize(),
            false => window.maximize()
        };
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            hide_window,
            close_window,
            maximize_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
