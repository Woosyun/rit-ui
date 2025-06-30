mod graph;
use graph::*;

mod commit_dialog;
use commit_dialog::*;

use leptos::task::spawn_local;
use leptos::{
    ev::{SubmitEvent, MouseEvent},
    prelude::*
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::path::PathBuf;
use rit::{
    commands::history::HistoryGraph,
    prelude::Oid,
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
        let head: Option<Oid> = serde_wasm_bindgen::from_value(res)
            .expect("cannot parse head");
        head
    });
    let ws = LocalResource::new(|| async move {
        let res = invoke_without_argument("read_workspace").await;
        let ws: Vec<Entry> = serde_wasm_bindgen::from_value(res)
            .expect("cannot parse workspace");
        ws
    });

    view! {
        <Transition fallback=move || view! { <h1>"loading..."</h1> }>
            {move || Suspend::new(async move {
                let h = head.await;
                view! {
                    <Show
                        when=move || h.is_some()
                        fallback=move || view! { <UninitializedPage head=head /> }
                    >
                        <Sidebar workspace=ws />
                        <HistoryPage workspace=ws head=head />
                    </Show>
                }
            })}
        </Transition>
    }
}
#[component] 
pub fn UninitializedPage(
    head: LocalResource<Option<Oid>>,
) -> impl IntoView {
    let on_click = move |ev: MouseEvent| {
        ev.prevent_default();

        spawn_local(async move {
            let _ = invoke_without_argument("initialize_repository").await;

            head.refetch();
        });
    };
    
    view! {
        <button on:click=on_click>
            "initalize repository"
        </button>
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

#[component]
pub fn HistoryPage(
    workspace: LocalResource<Vec<Entry>>,
    head: LocalResource<Option<Oid>>,
) -> impl IntoView {
    let hg = LocalResource::new(|| async move {
        let res = invoke_without_argument("get_history").await;
        let hg: HistoryGraph = serde_wasm_bindgen::from_value(res)
            .expect("cannot parse history graph from resposne");
        hg
    });

    let refetch_history = move || {
        hg.refetch();
        head.refetch();
        workspace.refetch();
    };
    let commit_dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();

    view! {
        <Transition fallback=move || view! {<h1>"waiting..."</h1>}>
        {move || Suspend::new(async move {
            let hg = hg.await;
            let head = head.await;
            view! {
                <RenderHistoryGraph history_graph=hg head=head commit_dialog_ref=commit_dialog_ref/>
            }
        })}
        <CommitDialog dialog_ref=commit_dialog_ref refetch_history=refetch_history />
        </Transition>
    }
}
