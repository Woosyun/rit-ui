use leptos::prelude::*;
use std::collections::{HashSet, HashMap};

use super::History;

#[component]
pub fn RenderGraph(
    history: ReadSignal<History>
) -> impl IntoView {
    // Circle and line data — static for now
    let circles = vec![
        (100.0, 100.0, 20.0, "red"),
        (300.0, 100.0, 20.0, "blue"),
    ];

    let lines = vec![
        (100.0, 100.0, 300.0, 100.0, "gray"),
    ];

    view! {
        <svg width="100%" height="100%" style="border: 1px solid #ccc;">
            {lines.into_iter().map(|(x1, y1, x2, y2, color)| view! {
                <line
                    x1={x1}
                    y1={y1}
                    x2={x2}
                    y2={y2}
                    stroke={color}
                    stroke-width="2"
                />
            }).collect_view()}

            {circles.into_iter().map(|(cx, cy, r, color)| view! {
                <circle
                    cx={cx}
                    cy={cy}
                    r={r}
                    fill={color}
                />
            }).collect_view()}
        </svg>
    }
}