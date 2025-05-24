mod graph;

use leptos::task::spawn_local;
use leptos::{
    ev::{SubmitEvent, MouseEvent},
    prelude::*
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::{
    path::PathBuf,
    collections::{HashSet, HashMap},
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
    let (init, set_init) = signal(false);
    Effect::new(move || {
        spawn_local(async move {
            let res = invoke_without_argument("is_repository_initialized").await;
            if let Ok(init) = serde_wasm_bindgen::from_value::<bool>(res) {
                set_init.set(init);
            }
        });
    });

    view! {
        <Show
            when=move || init.get()
            fallback=move || view! {
                <UninitializedPage set_init=set_init/>
            }
        >
            <Sidebar />
            <HistoryPage />
        </Show>
    }
}
#[component] 
pub fn UninitializedPage(set_init: WriteSignal<bool>) -> impl IntoView {
    let on_click = move |ev: MouseEvent| {
        ev.prevent_default();

        spawn_local(async move {
            let _ = invoke_without_argument("initialize_repository").await;

            set_init.set(true);
        });
    };
    
    view! {
        <button on:click=on_click>
            "initalize repository"
        </button>
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

#[derive(Serialize, Deserialize, Clone)]
pub struct History(pub HashMap<String, Commit>);
impl History {
    fn new() -> Self {
        History(HashMap::new())
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Commit {
    pub parents: HashSet<String>,
    pub children: HashSet<String>,
}

#[component]
pub fn HistoryPage() -> impl IntoView {
    let (history, set_history) = signal(History::new());
    Effect::new(move || {
        spawn_local(async move {
            let res = invoke_without_argument("read_entire_history").await;
            let res: History = serde_wasm_bindgen::from_value(res)
                .expect("cannot parse response");
            set_history.set(res);
        });
    });
    let (head, set_head) = signal(String::new());
    Effect::new(move || {
        spawn_local(async move {
            let res = invoke_without_argument("get_head").await;
            let res: String = serde_wasm_bindgen::from_value(res)
                .expect("failed getting head");
            set_head.set(res);
        });
    });

    view! {
        <div class="main">
        {move || {
            let head = head.get();
            let history = history.get();

            view! {
                <graph::RenderGraph history=history head=head />
            }
        }}
        </div>
    }
}
