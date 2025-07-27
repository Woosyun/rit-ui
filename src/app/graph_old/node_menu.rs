use leptos::{
    prelude::*,
    html::Dialog,
    ev::MouseEvent,
    task::spawn_local,
};
use super::NodeType;
use crate::app::invoke;
use rit::prelude::Oid;
use serde::Serialize;

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
    let checkout_to_revision = move |ev: MouseEvent, oid: Oid| {
        ev.prevent_default();
        spawn_local(async move {
            let p = CheckoutToRevisionPayload::from(oid);
            let res = invoke("checkout_to_revision", serde_wasm_bindgen::to_value(&p).unwrap()).await;
            let _: () = serde_wasm_bindgen::from_value(res)
                .expect("cannot checkout");
        });
    };
    let buttons = move || match node_type.get() {
        NodeType::Workspace => {
            view! {
                <button on:click=move |ev| open_dialog(ev, commit_dialog_ref)>
                    "commit"
                </button>
            }.into_any()
        },
        NodeType::Revision(oid) => {
            view! {
                <button on:click=move |ev| open_dialog(ev, create_branch_dialog_ref)>
                    "create branch"
                </button>
                <button on:click=move |ev| checkout_to_revision(ev, oid.clone())>
                    "checkout to revision"
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

#[derive(Serialize)]
struct CheckoutToRevisionPayload {
    oid: Oid,
}
impl CheckoutToRevisionPayload {
    fn from(oid: Oid) -> Self {
        Self { oid }
    }
}

#[derive(Serialize)]
struct CheckoutToBranchPayload {
    oid: Oid,
}
impl CheckoutToBranchPayload {
    fn from(oid: Oid) -> Self {
        Self { oid }
    }
}
