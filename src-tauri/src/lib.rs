// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
use std::io::Write;
mod consts;

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
            launch_game,
            get_mod_list,
            get_enabled_mods_list,
            update_modloader
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
                let _ = std::fs::write(local.to_string() + "/com.rl2-launcher.app/path.saved", path);
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
            if let Ok(path) = std::fs::read_to_string(local.to_string() + "/com.rl2-launcher.app/path.saved") {
                return Some(path);
            }
       }
    }
    None
}

#[tauri::command]
fn get_enabled_mods_list(path: String) -> Vec<String> {
    if let Ok(contents) = std::fs::read_to_string(path + "/Rogue Legacy 2_Data/Mods/enabled.json") {
        if let Ok(enabled_json) = serde_json::from_str::<EnabledModsJson>(contents.as_str()) {
            return enabled_json.enabled;
        }
    }
    std::vec::Vec::<String>::new()
}

#[derive(serde::Deserialize, serde::Serialize)]
struct EnabledModsJson {
    #[serde(rename = "Enabled")]
    enabled: std::vec::Vec::<String>,
    #[allow(dead_code)]
    #[serde(rename = "Disabled")]
    disabled: std::vec::Vec::<String>
}

#[tauri::command]
fn launch_game(path: String, modded: bool, enabled: std::collections::HashSet::<String>, disabled: std::collections::HashSet::<String>) {
    match modded {
        true => {
            let _ = std::fs::write(path.clone() + "/Rogue Legacy 2_Data/RuntimeInitializeOnLoads.json", consts::MODDED_RIOL_JSON.to_string());
            let _ = std::fs::write(path.clone() + "/Rogue Legacy 2_Data/ScriptingAssemblies.json", consts::MODDED_SA_JSON.to_string());
            if let Ok(json) = serde_json::to_string(&EnabledModsJson { 
                enabled: enabled.into_iter().collect::<std::vec::Vec<String>>(), 
                disabled: disabled.into_iter().collect::<std::vec::Vec<String>>(), 
            }) {
                let _ = std::fs::write(path.clone() + "/Rogue Legacy 2_Data/Mods/enabled.json", json);
            }
        },
        false => {
            let _ = std::fs::write(path.clone() + "/Rogue Legacy 2_Data/RuntimeInitializeOnLoads.json", consts::VANILLA_RIOL_JSON.to_string());
            let _ = std::fs::write(path.clone() + "/Rogue Legacy 2_Data/ScriptingAssemblies.json", consts::VANILLA_SA_JSON.to_string());
        }
    }

    match std::fs::exists(path.clone() + "/.egstore") {
       Err(_e) => return (),
       Ok(exists) => {
            match exists {
                true => launch_epic(),
                false => launch_steam(path)
            }
        }
    }
}

fn launch_epic() {
    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args([
                "/C",
                "start", 
                "",
                "\"com.epicgames.launcher://apps/4966d5da285c4f2c876937844b0e23ee%3Af5d84259a95a4b11ade74a7e4e0bde66%3Abd35425c9548494082d002f36601ff45?action=launch&silent=true\""
            ])
            .spawn();
    }
}

fn launch_steam(path: String) {
    let _ = std::process::Command::new(
        format!("{}/Rogue Legacy 2.exe", path)        
    ).spawn();
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

#[tauri::command]
async fn update_modloader() {
    if cfg!(target_os = "windows") {
        if let Ok(mut process) = std::process::Command::new("cmd")
            .current_dir("/")
            .stdin(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn() 
        {
            if let Some(stdin) = process.stdin
                .as_mut()
            {

                let _ = stdin.write_all(b"curl -OL https://github.com/RL2-API/RL2.ModLoader/releases/latest/download/RL2.ModLoader.tar.gz\n");
                let _ = stdin.write_all(b"mkdir rl2-ml\n");
                let _ = stdin.write_all(b"tar -xzvf RL2.ModLoader.tar.gz -C rl2-ml\n");
                let _ = stdin.write_all(b"start rl2-ml/RL2.ModLoader.Installer.exe\n");
                let _ = stdin.write_all(b"del RL2.ModLoader.tar.gz\n");
                let _ = stdin.write_all(b"del /q rl2-ml\n");
                
                let _ = process.wait_with_output();
            }
        }
        else { 
            return; 
        }
    }
}
