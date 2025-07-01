use leptos::{
    prelude::*,
    html::Dialog,
    ev::MouseEvent,
};
use super::NodeType;

#[component]
pub fn NodeMenu(
    node_type: ReadSignal<NodeType>,
    dialog_ref: NodeRef<Dialog>,
    commit_dialog_ref: NodeRef<Dialog>,
    create_branch_dialog_ref: NodeRef<Dialog>,
) -> impl IntoView {
    let open_dialog = move |ev: MouseEvent, node_ref: NodeRef<Dialog>| {
        ev.prevent_default();
        node_ref.get().unwrap()
            .show_modal().unwrap();
    };
    let buttons = move || match node_type.get() {
        NodeType::Workspace => {
            view! {
                <button on:click=move |ev| open_dialog(ev, commit_dialog_ref)>
                    "commit"
                </button>
            }.into_any()
        },
        NodeType::Revision => {
            view! {
                <button on:click=move |ev| open_dialog(ev, create_branch_dialog_ref)>
                    "create branch"
                </button>
            }.into_any()
        }
    };
    let close = move || dialog_ref
        .get().unwrap()
        .close();

    view! {
        <dialog
            node_ref=dialog_ref
        >
            {move || buttons()}
            <button on:click=move |ev| {
                ev.prevent_default();
                close();
            }>"close"</button>
        </dialog>
    }
}


