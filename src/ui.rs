use crate::{
    context::{AppContext, BPM_MAX, BPM_MIN},
    engine::handle::EngineHandle,
};
use dioxus::prelude::*;
use std::sync::{Arc, atomic::Ordering};

#[component]
pub fn App() -> Element {
    let mut ctx = use_signal(AppContext::new);
    let handle = use_context::<Arc<EngineHandle>>();
    let handle_bpm = Arc::clone(&handle);
    let handle_running = Arc::clone(&handle);

    rsx! {
        div {
            p { "♩ {ctx.read().bpm} BPM" }

            input {
                r#type: "range",
                min: BPM_MIN as f64,
                max: BPM_MAX as f64,
                value: ctx.read().bpm,
                oninput: move |e| {
                    if let Ok(val) = e.value().parse::<u64>() {
                        ctx.write().bpm = val;
                        handle_bpm.bpm.store(val, Ordering::Relaxed);
                    }
                }
            }

            button {
                onclick: move |_| {
                    let next = !ctx.read().is_running;
                    ctx.write().is_running = next;
                    handle_running.is_running.store(next, Ordering::Relaxed);
                },
                if ctx.read().is_running { "Stop" } else { "Start" }
            }
        }
    }
}
