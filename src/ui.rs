use dioxus::prelude::*;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

// ui.rs
#[component]
pub fn App() -> Element {
    let running = use_context::<Arc<AtomicBool>>();
    let mut is_running = use_signal(|| false);

    rsx! {
        div {
            p { "♩ 120 BPM" }
            button {
                onclick: move |_| {
                    let next = !is_running();
                    is_running.set(next);
                    running.store(next, Ordering::Relaxed);
                },
                if is_running() { "Stop" } else { "Start" }
            }
        }
    }
}
