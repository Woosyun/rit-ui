use leptos::task::spawn_local;
use leptos::{
    ev::SubmitEvent,
    prelude::*
};
//use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::{
    path::PathBuf,
    //collections::HashSet,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_argument(cmd: &str) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (wd, set_wd) = signal(None::<PathBuf>);

    view! {
        <div id="app">
        <Show
            when=move || !wd.get().is_none()
            fallback=move || view! {
                <WorkingDirectorySelectionPage set_wd=set_wd />
            }
        >
            <MainPage />
        </Show>
        </div>
    }
}

#[component]
pub fn WorkingDirectorySelectionPage(
    set_wd: WriteSignal<Option<PathBuf>>,
) -> impl IntoView {
    let open_explorer = move |ev: SubmitEvent| {
        ev.prevent_default();

        spawn_local(async move {
            let path = invoke_without_argument("set_working_directory")
                .await;
            let path: Option<PathBuf> = serde_wasm_bindgen::from_value(path).unwrap();

            set_wd.set(path);
        });
    };

    view! {
        <div class="select-page">
            //todo: recent history
            <form on:submit=open_explorer>
                <button type="submit" class="select-button">
                    "search folder"
                </button>
            </form>
        </div>
    }
}

#[component] 
pub fn MainPage() -> impl IntoView {
    view! {
        <Sidebar />
        <HistoryPage />
    }
}

use crate::entry::{Entry, EntryStatus};

#[component] 
pub fn Sidebar() -> impl IntoView {
    let (ws, set_ws) = signal(Vec::<Entry>::new());
    Effect::new(move || {
        spawn_local(async move {
            let res = invoke_without_argument("read_workspace").await;
            let res = serde_wasm_bindgen::from_value::<Vec<Entry>>(res)
                .map_err(|e| e.to_string());
            if let Ok(ws) = res {
                set_ws.set(ws);
            }
        });
    });
    let render_entry = move |entry: Entry| {
        let class_name = match entry.status {
            EntryStatus::Added => "file-item added-file",
            EntryStatus::Modified => "file-item modified-file",
            EntryStatus::NotChanged => "file-item"
        };
        let path = entry.path
            .to_str().unwrap()
            .to_string();
        view! {
            <p class=class_name>
                {path}
            </p>
        }
    };

    view! {
        <div class="sidebar">
        {move || {
            let ws = ws.get();
            ws.into_iter()
                .map(render_entry)
                .collect_view()
        }}
        </div>
    }
}

#[component]
pub fn HistoryPage() -> impl IntoView {
    view! {
        <div class="main">
            <h1>"History Page"</h1>
        </div>
    }
}
