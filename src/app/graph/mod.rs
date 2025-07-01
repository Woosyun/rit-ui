mod coordinates;
use coordinates::*;

mod svg;
use svg::*;

mod commit_dialog;
use commit_dialog::*;

mod node_menu;
use node_menu::*;

use rit::{
    prelude::Oid,
    commands::history::HistoryGraph,
};
use leptos::{
    prelude::*,
    ev::MouseEvent,
    html::Dialog,
};

#[component]
pub fn RenderHistoryGraph<F>(
    history_graph: LocalResource<HistoryGraph>,
    head: LocalResource<Option<Oid>>,
    callback_after_commit: F
) -> impl IntoView 
where
    F: Fn() + 'static,
{
    let node_menu_ref: NodeRef<Dialog> = NodeRef::new();
    let commit_dialog_ref: NodeRef<Dialog> = NodeRef::new();
    let create_branch_dialog_ref: NodeRef<Dialog> = NodeRef::new();

    let (node_type, set_node_type) = signal(NodeType::Revision);

    let on_contextmenu_node = move |ev: MouseEvent, node_type: NodeType| {
        ev.prevent_default();
        set_node_type.set(node_type);
        leptos::logging::log!("clicked: {}", ev.button());
        //2 => right click
        if ev.button() == 2 {
            node_menu_ref
                .get().unwrap()
                .show_modal().unwrap();
        }
    };
    
    let nodes = move |coordinates: &Coordinates| coordinates
        .iter()
        .map(|(_oid, (x, y))| {
            view! {
                <Circle x=*x y=*y on_contextmenu=on_contextmenu_node node_type=NodeType::Revision/>
            }
        })
        .collect_view();

    let edges = move |hg: HistoryGraph, coord: &Coordinates| hg
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

    let workspace = move |head: &Option<Oid>, coordinates: &Coordinates| {
        let (parent, child) = match head {
            Some(head) => {
                let parent = coordinates.get(head).unwrap();
                let child = (parent.0, parent.1 + Coordinates::gap());
                (*parent, child)
            },
            None => {
                (Coordinates::init(), Coordinates::init())
            }
        };

        view! {
            <Circle x=child.0 y=child.1 on_contextmenu=on_contextmenu_node node_type=NodeType::Workspace />
            <Line from=parent to=child />
        }
    };


    view! {
        <svg width="100%" height="100%" style="border: 1px solid #ccc;">
        <Transition fallback=move || view! { <h1>"loading..."</h1> }>
        {move || Suspend::new(async move {
            let history_graph = history_graph.await;
            let head = head.await;
            let coordinates = Coordinates::from(&history_graph);

            view! {
                {nodes(&coordinates)}
                {edges(history_graph, &coordinates)}
                {workspace(&head, &coordinates)}
            }
        })}
        </Transition>
        </svg>

        <NodeMenu 
            node_type=node_type
            dialog_ref=node_menu_ref 
            commit_dialog_ref=commit_dialog_ref
            create_branch_dialog_ref=create_branch_dialog_ref
        />
        <CommitDialog dialog_ref=commit_dialog_ref callback_after_commit=callback_after_commit />
        //<CreateBranchDialog dialog_ref=create_branch_dialog_ref />
    }
}

#[derive(Clone)]
pub enum NodeType {
    Workspace,
    Revision,
}
impl NodeType {
    pub fn color(&self) -> &'static str {
        match self {
            NodeType::Workspace => "orange",
            NodeType::Revision => "gray",
        }
    }
}
