mod coordinates;
use coordinates::*;

mod commit_dialog;
use commit_dialog::*;

use rit::{
    prelude::Oid,
    commands::history::HistoryGraph,
};
use leptos::prelude::*;

#[component]
pub fn RenderHistoryGraph(
    history_graph: HistoryGraph,
    head: Option<Oid>,
) -> impl IntoView {
    leptos::logging::log!("history graph: {:?}", &history_graph);
    let coordinates = Coordinates::from(&history_graph);

    leptos::logging::log!("{:?}", coordinates);
    
    let t = coordinates.clone();
    let nodes = move || t
        .iter()
        .map(|(_oid, (x, y))| {
            view! {
                <Circle x=*x y=*y/>
            }
        })
        .collect_view();

    let coord = coordinates.clone();
    let edges = move || history_graph
        .children()
        .iter()
        .map(|(parent, children)| {
            let parent_xy = coord.get(parent).unwrap();
            children
                .iter()
                .map(|child| {
                    let child_xy = coord.get(child).unwrap();
                    view! {
                        <Line from=*parent_xy to=*child_xy />
                    }
                })
                .collect_view()
        })
        .collect_view();

    let h = head.clone();
    leptos::logging::log!("{:?}", &h);
    let dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let workspace = move || match h {
        Some(head) => {
            //error: commit이 있는 상태인데
            //coordinates is empty && head is some.
            let parent = coordinates.get(&head).unwrap();
            let y = parent.1 + Coordinates::gap();
            view! {
                <WorkspaceCircle x=parent.0 y=y commit_dialog_ref=dialog_ref/>
                <Line from=*parent to=(parent.0, y) />
            }.into_any()
        },
        None => {
            let (x, y) = Coordinates::init();
            view! {
                <WorkspaceCircle x=x y=y commit_dialog_ref=dialog_ref/>
            }.into_any()
        }
    };


    view! {
        <svg width="100%" height="100%" style="border: 1px solid #ccc;">
            {nodes()}
            {edges()}
            {workspace()}
        </svg>
        <CommitDialog dialog_ref=dialog_ref/>
    }
}

#[component]
pub fn Circle(
    x: usize, 
    y: usize,
) -> impl IntoView {
    view! {
        <circle 
            cx={x}
            cy={y}
            r=20.0
            fill="black"
        />
    }
}

#[component]
pub fn WorkspaceCircle(
    x: usize,
    y: usize,
    commit_dialog_ref: NodeRef<leptos::html::Dialog>,
) -> impl IntoView {
    let on_click = move |_| {
        commit_dialog_ref
            .get().unwrap()
            .show_modal().unwrap();
    };
    view! {
        <circle 
            cx={x}
            cy={y}
            r=20.0
            fill="black"
            on:click=on_click
        />
    }
}


#[component]
pub fn Line(from: (usize, usize), to: (usize, usize)) -> impl IntoView {
    view! {
        <line 
            x1={from.0}
            y1={from.1}
            x2={to.0}
            y2={to.1}
            stroke="black"
            stroke-width="2"
        />
    }
}
