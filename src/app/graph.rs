use leptos::prelude::*;
use std::collections::{HashSet, HashMap};

use super::History;

#[derive(Clone)]
struct Node {
    x: f64,
    y: f64,
}
impl Node {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y
        }
    }
}
#[derive(Clone)]
pub struct Graph {
    nodes: HashMap<String, Node>,
    edges: HashMap<String, HashSet<String>>,
}
impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}
fn render_graph(graph: &Graph) -> impl IntoView {
    let nodes = graph.nodes
        .iter()
        .map(|(_, node)| {
            view! {
                <circle 
                    cx={node.x}
                    cy={node.y}
                    r=20.0
                    fill="black"
                />
            }
        })
        .collect_view();
    let edges = graph.edges
        .iter()
        .map(|(from, to_set)| {
            to_set
                .iter()
                .map(|to| {
                    let from = graph.nodes.get(from).unwrap();
                    let to = graph.nodes.get(to).unwrap();

                    view! {
                        <line 
                            x1={from.x}
                            y1={from.y}
                            x2={to.x}
                            y2={to.y}
                            stroke="black"
                            stroke-width="2"
                        />
                    }
                })
                .collect_view()
        })
        .collect_view();
    view! {
        {nodes}
        {edges}
    }
}

#[component]
pub fn RenderGraph(
    history: History,
    head: String,
) -> impl IntoView {
    let make_graph = move || {
        let mut graph = Graph::new();
        //head
        let current_commit = history.0.get(&head).unwrap();
        let current_node = Node::new(100.0, 300.0);
        graph.nodes.insert(head.clone(), current_node.clone());
        //children~1
        for child in current_commit.children.iter() {
            graph.edges
                .entry(head.clone())
                .or_insert_with(|| HashSet::new())
                .insert(child.to_string());

            let n_x = current_node.x;
            let n_y = current_node.y - 150.0;
            graph.nodes.insert(child.to_string(), Node::new(n_x, n_y));
        }
        for parent in current_commit.parents.iter() {
            graph.edges
                .entry(head.clone())
                .or_insert_with(|| HashSet::new())
                .insert(parent.to_owned());

            let x = current_node.x;
            let y = current_node.y + 150.0;
            graph.nodes.insert(parent.to_owned(), Node::new(x, y));
        }

        graph
    };

    view! {
        <svg width="100%" height="100%" style="border: 1px solid #ccc;">
            {move || {
                let graph = make_graph();
                render_graph(&graph)
            }}
        </svg>
    }
}