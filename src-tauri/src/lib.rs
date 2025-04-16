// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            hide_window,
            close_window,
            maximize_window,
            drag_window,
            check_if_correct_path,
            get_saved_path,
            get_mod_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

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

#[tauri::command]
fn drag_window(window: tauri::Window) {
    let _ = window.start_dragging();
}

#[tauri::command]
fn check_if_correct_path(window: tauri::Window, path: String) -> bool {
    if let Ok(exists @ true) = std::fs::exists(path.clone() + "/Rogue Legacy 2.exe") {
        if let Ok(local_maybe) = window.path().local_data_dir() {
            if let Some(local) = local_maybe.to_str() {
                let _ = std::fs::write(local.to_string() + "/path.saved", path);
            }
        }

        return exists;
    }

    false
}

#[tauri::command]
fn get_saved_path(window: tauri::Window) -> Option<String> {
    if let Ok(local_maybe) = window.path().local_data_dir() {
       if let Some(local) = local_maybe.to_str() {
            if let Ok(path) = std::fs::read_to_string(local.to_string() + "/path.saved") {
                return Some(path);
            }
       }
    }
    
    None
}

#[tauri::command]
async fn get_mod_list(path: String) -> std::vec::Vec::<String> {
    walkdir::WalkDir::new(path + "/Rogue Legacy 2_Data/Mods")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .filter(|f| f.path().display().to_string().ends_with(".mod.json"))
        .map(|f| std::fs::read_to_string(f.path()))
        .filter_map(Result::ok)
        .collect()
} 
