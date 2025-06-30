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
pub fn CommitDialog<F>(
    dialog_ref: NodeRef<leptos::html::Dialog>,
    refetch_history: F,
) -> impl IntoView 
where
    F: Fn() + 'static,
{
    use leptos::ev::SubmitEvent;

    let (msg, set_msg) = signal("".to_string());
    let close = move |_| {
        dialog_ref
            .get().unwrap()
            .close();
    };
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let msg = msg.get();
        let f = format!("commit message entered: {}", msg);
        leptos::logging::log!("{}", f);
        spawn_local(async move {
            let payload = CommitPayLoad::from(msg);
            let res = crate::app::invoke("commit", serde_wasm_bindgen::to_value(&payload).unwrap()).await;
            let _: () = serde_wasm_bindgen::from_value(res)
                .expect("cannot commit");
        });
        dialog_ref
            .get().unwrap()
            .close();
        refetch_history();
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
