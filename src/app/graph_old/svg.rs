use leptos::{
    prelude::*,
    ev::MouseEvent,
};
use super::NodeType;

#[component]
pub fn Circle<F>(
    x: usize, 
    y: usize,
    on_contextmenu: F,
    node_type: NodeType,
) -> impl IntoView 
where
    F: Fn(MouseEvent, NodeType) + 'static,
{
    let color = node_type.clone().color();
    view! {
        <circle 
            cx={x}
            cy={y}
            r=20.0
            fill=move || color
            on:contextmenu=move |ev| on_contextmenu(ev, node_type.clone())
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
