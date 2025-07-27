use leptos::prelude::*;
use rit::{
    repository::Oid,
    commands::history::HistoryGraph,
};

#[component]
pub fn Page(
    #[allow(unused)]
    history_graph: LocalResource<HistoryGraph>,
    #[allow(unused)]
    head: LocalResource<Option<Oid>>,
) -> impl IntoView {
    view! {
        <h1>"hello world"</h1>
    }
}
