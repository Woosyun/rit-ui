#![allow(unused)]

use std::collections::{HashSet, VecDeque, HashMap};
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
    let coordinates = Coordinates::from(&history_graph);
    
    let t = coordinates.clone();
    let nodes = move || t
        .iter()
        .map(|(_, (x, y))| {
            view! {
                <Circle x=*x y=*y />
            }
        })
        .collect_view();
    let edges = move || history_graph
        .children()
        .iter()
        .map(|(parent, children)| {
            let parent_xy = coordinates.get(parent).unwrap();
            children
                .iter()
                .map(|child| {
                    let child_xy = coordinates.get(child).unwrap();
                    view! {
                        <Line from=*parent_xy to=*child_xy />
                    }
                })
                .collect_view()
        })
        .collect_view();

    view! {
        {move || match head {
            Some(_) => {
                view! {
                    {nodes()}
                    {edges()}
                }.into_any()
            },
            None => {
                view! {
                    <h1>"No History Found"</h1>
                }
            }.into_any()
        }}
    }
}
#[component]
pub fn Circle(x: usize, y: usize) -> impl IntoView {
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

#[derive(Clone)]
pub struct Coordinates(HashMap<Oid, (usize, usize)>);
impl Coordinates {
    pub fn from(hg: &HistoryGraph) -> Self {
        let mut nodes = Coordinates(HashMap::new());
        let mut next_x = 0;
        for root in hg.roots() {
            let next_x = std::cmp::max(
                next_x + Self::gap(),
                nodes.grant(root, (next_x, 0), hg)
            );
        }
        nodes
    }
    fn gap() -> usize {
        50
    }
    fn grant(&mut self, parent: &Oid, xy: (usize, usize), hg: &HistoryGraph) -> usize {
        self.insert(parent.clone(), xy);

        let mut next_x = xy.0 + Self::gap();
        if let Some(children) = hg.children().get(parent) {
            for child in children {
                next_x = std::cmp::max(
                    next_x + Self::gap(), 
                    self.grant(child, (next_x, xy.1+Self::gap()), hg)
                );
            }
        }

        next_x
    }
}

impl std::ops::Deref for Coordinates {
    type Target = HashMap<Oid, (usize, usize)>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Coordinates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
