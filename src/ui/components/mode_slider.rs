// src/ui/components/mode_slider.rs

use crate::context::AppContext;
use crate::session::config::Mode;
use dioxus::prelude::*;

const MODES: [(Mode, &str); 3] = [
    (Mode::Infinity, "∞"),
    (Mode::Block, "▭"),
    (Mode::Ladder, "↑"),
];

#[component]
pub fn ModeSlider() -> Element {
    let mut ctx = use_context::<Signal<AppContext>>();
    let mut drag_start_x = use_signal(|| 0.0f64);

    let mode = ctx.read().mode;
    let active_idx = MODES.iter().position(|(m, _)| *m == mode).unwrap_or(0);

    rsx! {
        div {
            class: "mode-slider",
            // TODO: IMPORTANT: abstract away later to handle different events
            // ie touch vs mouse (different coordinate API etc)
            onmousedown: move |e| {
                drag_start_x.set(e.client_coordinates().x);
            },
            onmouseup: move |e| {
                let delta = e.client_coordinates().x - drag_start_x();
                let current = ctx.read().mode;
                let next = if delta.abs() < 8.0 || delta > 0.0 {
                    current.next() // drag right
                } else {
                    current.prev() // drag left
                };
                ctx.write().mode = next;
            },

            // sliding indicator — translates based on active index
            div {
                class: "mode-indicator",
                style: "transform: translateX({active_idx * 100}%)",
            }

            // labels
            {
                MODES
                    .iter()
                    .map(|(m, label)| {
                        let is_active = *m == mode;
                        rsx! {
                            span { key: "{label}", class: if is_active { "mode-label active" } else { "mode-label" }, "{label}" }
                        }
                    })
            }
        }
    }
}
