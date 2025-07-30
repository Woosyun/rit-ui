use leptos::prelude::*;
use rit::{
    repository::{Oid, Head},
    commands::history::HistoryGraph,
};
use super::*;

#[component]
pub fn Page(
    history_graph: LocalResource<HistoryGraph>,
    head: LocalResource<Head>,
) -> impl IntoView {
    let render_edges = |hv: &HistoryViewer| {
        hv.edges
            .iter()
            .map(|(parent, children)| {
                let from = hv.nodes.get(*parent)
                    .map(|node| (node.x, node.y))
                    .unwrap();
                children.iter()
                    .map(|child| {
                        let to = hv.nodes.get(*child)
                            .map(|node| (node.x, node.y))
                            .unwrap();
                        view! {
                            <RenderEdge from=from to=to />
                        }
                    })
                    .collect_view()
            })
            .collect_view()
    };
    let render_nodes = move |hv: &HistoryViewer| {
        hv.nodes
            .iter()
            .map(|(oid, node)| {
                view! {
                    <RenderNode oid=oid node=node />
                }
            })
            .collect_view()
    };
    let (ws_node, ws_edge) = move |head: Head| {
        match head {
            Head::None => (Node::new(), None),
            Head::Oid(_) => (Node::new(), None),
            Head::Branch(_) => (Node::new(), None),
        }
    };

    view! {
        <svg width="100%" height="100%" style="border: 1px solid #ccc;">
        <Transition fallback=move || view!{ <h1>"loading..."</h1> }>
        {move || Suspend::new(async move {
            let hg = history_graph.await;
            let head = head.await;

            let hv = HistoryViewer::from(&hg);
            view! {
                {render_edges(&hv)}
                {render_nodes(&hv)}
            }
        })}
        </Transition>
        </svg>
    }
}

#[component]
fn RenderNode<'a, 'b>(
    #[allow(unused)]
    oid: &'a Oid,
    #[allow(unused)]
    node: &'b Node,
) -> impl IntoView {
    view! {
        <circle 
            cx={node.x}
            cy={node.y}
            r=20.0
            fill="gray"
        />
    }
}
#[component]
fn RenderEdge(
    from: (u32, u32),
    to: (u32, u32),
) -> impl IntoView {
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
