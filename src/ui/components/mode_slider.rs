use std::sync::Arc;
use std::sync::atomic::Ordering;

use crate::context::AppContext;
use crate::engine::handle::EngineHandle;
use crate::session::config::Mode;
use dioxus::prelude::*;

const MODES: [(Mode, &str); 3] = [
    (Mode::Infinity, "∞"),
    (Mode::Block, "▭"),
    (Mode::Ladder, "↑"),
];

#[component]
pub fn ModeSlider() -> Element {
    let engine = use_context::<Arc<EngineHandle>>();
    let engine_drag = Arc::clone(&engine);

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
                if delta.abs() >= 8.0 {
                    let mut c = ctx.write();
                    let new_mode = if delta > 0.0 { c.mode.next() } else { c.mode.prev() };
                    if c.is_running {
                        c.is_running = false;
                        engine_drag.running.store(false, Ordering::Relaxed);
                    }
                    c.mode = new_mode;
                }
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
                        let target = *m;
                        let engine_label = Arc::clone(&engine);
                        rsx! {
                            span {
                                key: "{label}",
                                class: if target == mode { "mode-label active" } else { "mode-label" },
                                onclick: move |_| {
                                    let mut c = ctx.write();
                                    let new_mode = c.mode.click(target);
                                    if c.is_running {
                                        c.is_running = false;
                                        engine_label.running.store(false, std::sync::atomic::Ordering::Relaxed);
                                    }
                                    c.mode = new_mode;
                                },
                                "{label}"
                            }
                        }
                    })
            }
        }
    }
}
