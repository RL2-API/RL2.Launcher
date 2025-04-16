use leptos::{ task };
use leptos::{ prelude::* };
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
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

#[component]
pub fn App() -> impl IntoView {
    let (rl2_path, set_rl2_path) = signal(String::new());

    let update_rl2_path = move |ev| {
        let v = event_target_value(&ev);
        set_rl2_path.set(v);
    };

    view! {
        <main class="container">
            <div id="sidebar">
                <input
                    id="rl2_path"
                    placeholder="RL2 installation path"
                    on:input=update_rl2_path
                />
                <div>
                    <button id="modded">Modded</button>
                    <button id="vanilla">Vanilla</button>
                </div>
            </div>
        </main>
    }
}
