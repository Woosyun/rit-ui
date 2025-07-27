use leptos::{
    prelude::*,
    html::Dialog,
    ev::SubmitEvent,
    task::spawn_local,
};
use super::super::invoke;
use serde::Serialize;

#[component]
pub fn CreateBranchDialog(
    dialog_ref: NodeRef<Dialog>,
) -> impl IntoView {
    let (branch, set_branch) = signal("".to_string());

    let on_submit = move |_ev: SubmitEvent| {
        let branch = branch.get();

        spawn_local(async move {
            let p = CreateBranchPayload::from(branch);
            let res = invoke("create_branch", serde_wasm_bindgen::to_value(&p).unwrap()).await;
            let _: () = serde_wasm_bindgen::from_value(res)
                .expect("cannot create branch");
        });
    };

    view! {
        <dialog class="dialog-box" node_ref=dialog_ref>
            <form on:submit=on_submit>
                <input bind:value=(branch, set_branch) />
                <input type="submit" value="create branch"/>
            </form>
        </dialog>
    }
}

#[derive(Serialize)]
struct CreateBranchPayload {
    new_branch: String,
}
impl CreateBranchPayload {
    fn from(new_branch: String) -> Self {
        Self {
            new_branch
        }
    }
}
