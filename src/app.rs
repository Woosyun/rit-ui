mod history;

use leptos::task::spawn_local;
use leptos::{
    ev::{SubmitEvent},
    prelude::*
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::path::PathBuf;
use rit::{
    commands::history::HistoryGraph,
    prelude::{Oid, Head},
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
    let working_directory = LocalResource::new(|| async move {
        let res = invoke_without_argument("get_working_directory").await;
        let wd: Option<PathBuf> = serde_wasm_bindgen::from_value(res)
            .expect("cannot parse from get_working_directory");
        wd
    });

    view! {
        <div id="app">
        <Transition fallback=move || view! {<h1> "loading..." </h1>}>
        {move || Suspend::new(async move {
            let wd = working_directory.await;
            view! {
                <Show
                    when=move || wd.is_some()
                    fallback=move || view! {
                        <WorkingDirectorySelectionPage working_directory=working_directory />
                    }
                >
                    <MainPage />
                </Show>
            }
        })}
        </Transition>
        </div>
    }
}

#[component]
pub fn WorkingDirectorySelectionPage(
    working_directory: LocalResource<Option<PathBuf>>,
) -> impl IntoView {
    let open_explorer = move |ev: SubmitEvent| {
        ev.prevent_default();

        spawn_local(async move {
            let path = invoke_without_argument("set_working_directory")
                .await;
            let _: Option<PathBuf> = serde_wasm_bindgen::from_value(path).unwrap();

            working_directory.refetch();
        });
    };

    view! {
        <div class="select-page">
            <form on:submit=open_explorer>
                <button type="submit" class="select-button">
                    "search folder"
                </button>
            </form>
        </div>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub status: EntryStatus,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntryStatus {
    Added,
    Modified,
    NotChanged,
}

#[component] 
pub fn MainPage() -> impl IntoView {
    let head = LocalResource::new(|| async move {
        let res = invoke_without_argument("get_head").await;
        let head: Head = serde_wasm_bindgen::from_value(res)
            .expect("cannot parse head");
        head
    });
    let ws = LocalResource::new(|| async move {
        let res = invoke_without_argument("read_workspace").await;
        let ws: Vec<Entry> = serde_wasm_bindgen::from_value(res)
            .expect("cannot parse workspace");
        ws
    });
    let hg = LocalResource::new(|| async move {
        let res = invoke_without_argument("get_history").await;
        let hg: HistoryGraph = serde_wasm_bindgen::from_value(res)
            .expect("cannot parse history graph from resposne");
        hg
    });
    let is_repository_initialized = LocalResource::new(|| async move {
        let res = invoke_without_argument("is_repository_initialized").await;
        let init: bool = serde_wasm_bindgen::from_value(res)
            .expect("cannot parse is_repository_initialized");
        init
    });

    view! {
        <Transition fallback=move || view! { <h1>"loading..."</h1> }>
        {move || Suspend::new(async move {
            let init = is_repository_initialized.await;
            //let hg = hg.await;

            view! {
                <Show
                    when=move || init
                    fallback=move || view! { <UninitializedPage /> }
                >
                    <Sidebar workspace=ws />
                    <history::Page history_graph=hg head=head/>
                </Show>
            }
        })}
        </Transition>
    }
}

#[component] 
pub fn UninitializedPage() -> impl IntoView {
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        spawn_local(async move {
            let res = invoke_without_argument("initialize_repository").await;
            let _: () = serde_wasm_bindgen::from_value(res)
                .expect("cannot initialize repository");
        });
    };
    
    view! {
        <form on:submit=on_submit>
            <input type="submit" value="initialize repository" />
        </form>
    }
}

#[component] 
pub fn Sidebar(
    workspace: LocalResource<Vec<Entry>>,
) -> impl IntoView {
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
        <Transition fallback=move || view! { <h1>"loading..."</h1> }>
            {move || Suspend::new(async move {
                let ws = workspace.await;

                ws.into_iter()
                    .map(render_entry)
                    .collect_view()
            })}
        </Transition>
        </div>
    }
}
