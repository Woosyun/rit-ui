mod coordinates;
use coordinates::*;

mod svg;
use svg::*;

mod commit_dialog;
use commit_dialog::*;

mod node_menu;
use node_menu::*;

mod create_branch_dialog;
use create_branch_dialog::*;

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
pub fn RenderHistoryGraph(
    history_graph: LocalResource<HistoryGraph>,
    head: LocalResource<Option<Oid>>,
) -> impl IntoView {
    let node_menu_ref: NodeRef<Dialog> = NodeRef::new();
    let commit_dialog_ref: NodeRef<Dialog> = NodeRef::new();
    let create_branch_dialog_ref: NodeRef<Dialog> = NodeRef::new();

    let (node_type, set_node_type) = signal(NodeType::Workspace);

    let on_contextmenu_node = move |ev: MouseEvent, node_type: NodeType| {
        ev.prevent_default();
        set_node_type.set(node_type);
        //2 => right click
        if ev.button() == 2 {
            node_menu_ref
                .get().unwrap()
                .show_modal().unwrap();
        }
    };
    
    let nodes = move |coordinates: &Coordinates| coordinates
        .iter()
        .map(|(oid, (x, y))| {
            view! {
                <Circle x=*x y=*y on_contextmenu=on_contextmenu_node node_type=NodeType::Revision(oid.clone())/>
            }
        })
        .collect_view();

    let edges = move |hg: &HistoryGraph, coord: &Coordinates| hg
        .parents()
        .iter()
        .map(|(child, parents)| {
            let child_xy = coord.get(child).unwrap();
            parents
                .iter()
                .map(|parent| {
                    let parent_xy = coord.get(parent).unwrap();
                    view! {
                        <Line from=*child_xy to=*parent_xy />
                    }
                })
                .collect_view()
        })
        .collect_view();

    let workspace = move |hg: &HistoryGraph, head: &Option<Oid>, coordinates: &Coordinates| {
        let (child, parent) = match head {
            Some(head) => {
                let parent = coordinates.get(head).unwrap();
                let number_of_children = hg
                    .parents()
                    .get(head)
                    .map(|parents| parents.len())
                    .unwrap_or(0);
                let child = (parent.0 + Coordinates::gap()*number_of_children, parent.1 + Coordinates::gap());
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
                {edges(&history_graph, &coordinates)}
                {workspace(&history_graph, &head, &coordinates)}
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
        <CommitDialog dialog_ref=commit_dialog_ref />
        <CreateBranchDialog dialog_ref=create_branch_dialog_ref />
    }
}

#[derive(Clone)]
pub enum NodeType {
    Workspace,
    Revision(Oid),
}
impl NodeType {
    pub fn color(&self) -> &'static str {
        match self {
            NodeType::Workspace => "orange",
            NodeType::Revision(_) => "gray",
        }
    }
}
