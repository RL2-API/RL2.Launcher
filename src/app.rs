use leptos::{ task };
use leptos::{ prelude::* };
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["console"])]
    fn log(cmd: String);
}

#[component]
pub fn App() -> impl IntoView {

    view! {
        <Titlebar />
        <main class="container">
            <Content />
        </main>
    }
}

#[component]
pub fn Titlebar() -> impl IntoView {
    let name = "Rogue Legacy 2 Launcher";

    let hide = move |_ev| {
        task::spawn_local(async move {
            invoke("hide_window", JsValue::default()).await;
        });
    };

    let close = move |_ev| {
        task::spawn_local(async move {
            invoke("close_window", JsValue::default()).await;
        });
    };
    
    let toggle_maximize = move |_ev| {
        task::spawn_local(async move {
            invoke("maximize_window", JsValue::default()).await;
        });
    };
    
    let drag = move |_ev| {
        task::spawn_local(async move {
            invoke("drag_window", JsValue::default()).await;
        });
    };

    view! {
        <nav id="titlebar">
            <img src="public/icon.png" width="30" />
            <button id="title" on:mousedown=drag>
                { move || name }
            </button>    
            <div>
                <button on:click=hide>_</button>
                <button on:click=toggle_maximize>[_]</button>
                <button on:click=close>"❌"</button>
            </div>
        </nav>
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct PathArgs {
    path: String
}

#[derive(serde::Serialize, serde::Deserialize)]
struct LaunchArgs {
    path: String,
    modded: bool
} 

#[component]
pub fn Content() -> impl IntoView {
    let (rl2_path, set_rl2_path) = signal(String::new());
    let (mod_list, set_mod_list) = signal(std::vec::Vec::<String>::new());

    let update_rl2_path = move |ev| {
        let v = event_target_value(&ev);
        set_mod_list.set(Vec::new());
        task::spawn_local(async move {
            if let Ok(json) = serde_wasm_bindgen::to_value(&PathArgs { path: v.to_string() }) { 
                if invoke("check_if_correct_path", json.clone()).await == JsValue::TRUE {
                    set_rl2_path.set(v.to_string());
                    if let Ok(mods) = serde_wasm_bindgen::from_value(invoke("get_mod_list", json).await) {
                        set_mod_list.set(mods);
                    }
                }
            }
        })
    };

    let launch_modded = move |_ev| {
        task::spawn_local(async move {
            if let Ok(json) = serde_wasm_bindgen::to_value(&LaunchArgs { path: rl2_path.get_untracked(), modded: true }) {
                invoke("launch_game", json).await;
            }
        })
    };
 
    let launch_vanilla = move |_ev| {
        task::spawn_local(async move {
            if let Ok(json) = serde_wasm_bindgen::to_value(&LaunchArgs { path: rl2_path.get_untracked(), modded: false }) {
                invoke("launch_game", json).await;
            }
        })
    };

    task::spawn_local(async move {
        if let Ok(Some(saved_path)) = serde_wasm_bindgen::from_value::<Option<String>>(invoke("get_saved_path", JsValue::default()).await) {
            if let Ok(json) = serde_wasm_bindgen::to_value(&PathArgs { path: saved_path.clone() }) { 
                set_rl2_path.set(saved_path);
                if let Ok(mods) = serde_wasm_bindgen::from_value(invoke("get_mod_list", json).await) {
                    set_mod_list.set(mods);
                }
            }
        }
    });

    view! {
        <div id="modlist">
            <div id="modlist_header">
                <p> Name </p>
                <p class="version"> Version </p>
                <p> Author </p>
                <p class="enabled"> Enabled </p>
            </div>
            {move || {
                let mut out = Vec::new();
                for element in mod_list.get() {
                    if let Ok(serde_json::Value::Object(mod_obj)) = serde_json::from_str(&element) {
                        if let (Some(name), Some(author), Some(version)) = (mod_obj["Name"].as_str(), mod_obj["Author"].as_str(), mod_obj["Version"].as_str()) {
                            let id = name.to_string() + "_enabled";
                            out.push(view! {
                                <article>
                                    <p> { name.to_string() } </p>
                                    <p class="version"> v{ version.to_string() } </p>
                                    <p> { author.to_string() } </p>
                                    <div class="enabled">
                                        <input type="checkbox" id={id}/>
                                    </div>
                                </article>
                            });
                        }
                    }
                }
                out
            }}
        </div>
        <div id="launchbar">
            <input
                id="rl2_path"
                placeholder="RL2 installation path"
                on:input=update_rl2_path
                value=rl2_path
            />
            <button id="modded" on:click=launch_modded>Modded</button>
            <button id="vanilla" on:click=launch_vanilla>Vanilla</button>
        </div>
    }
}
