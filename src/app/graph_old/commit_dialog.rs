use serde::Serialize;
use leptos::{
    prelude::*,
    task::spawn_local,
};

#[derive(Serialize)]
struct CommitPayLoad {
    msg: String,
}
impl CommitPayLoad {
    pub fn from(msg: String) -> Self {
        Self {
            msg
        }
    }
}
#[component]
pub fn CommitDialog(
    dialog_ref: NodeRef<leptos::html::Dialog>,
) -> impl IntoView {
    use leptos::ev::SubmitEvent;

    let (msg, set_msg) = signal("".to_string());
    let close = move |_| {
        dialog_ref
            .get().unwrap()
            .close();
    };
    let on_submit = move |_ev: SubmitEvent| {
        let msg = msg.get();
        let f = format!("commit message entered: {}", msg);
        leptos::logging::log!("{}", f);
        spawn_local(async move {
            let payload = CommitPayLoad::from(msg);
            let res = crate::app::invoke("commit", serde_wasm_bindgen::to_value(&payload).unwrap()).await;
            let _: () = serde_wasm_bindgen::from_value(res)
                .expect("cannot commit");
        });
    };
    view! {
        <dialog class="dialog-box" node_ref=dialog_ref>
            <form on:submit=on_submit>
            <input
                class="input-box"
                bind:value=(msg, set_msg)
            />
            <input type="submit" value="commit" />
            </form>
            <button on:click=close>"close"</button>
        </dialog>
    }
}
