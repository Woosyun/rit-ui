use leptos::task::spawn_local;
use leptos::{
    ev::SubmitEvent,
    prelude::*
};
//use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::{
    path::PathBuf,
    collections::HashSet,
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
        <Show
            when=move || !wd.get().is_none()
            fallback=move || view! {
                <WorkingDirectorySelectionPage set_wd=set_wd />
            }
        >
            <MainPage />
        </Show>
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
        <h1>"Select Working Directory"</h1>
        //todo: recent history
        <form on:submit=open_explorer>
            <button type="submit">
                "search folder"
            </button>
        </form>
    }
}

#[component] 
pub fn MainPage() -> impl IntoView {
    view! {
        <Sidebar />
        <HistoryPage />
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    let _rev = LocalResource::new(move || {
        invoke_without_argument("read_workspace")
    });

    view! {
        <h1>"Sidebar"</h1>
    }
}

#[component]
pub fn HistoryPage() -> impl IntoView {
    view! {
        <h1>"History Page"</h1>
    }
}
