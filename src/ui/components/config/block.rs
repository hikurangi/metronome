use crate::context::AppContext;
use crate::engine::handle::EngineHandle;
use crate::session::handle::{Cmd, SessionHandle};
use dioxus::prelude::*;
use std::sync::{Arc, atomic::Ordering};

#[component]
pub fn BlockConfigPanel() -> Element {
    let mut ctx = use_context::<Signal<AppContext>>();
    let engine = use_context::<Arc<EngineHandle>>();
    let session = use_context::<Arc<SessionHandle>>();

    let bpm = ctx.read().block_config.bpm;
    let duration = ctx.read().block_config.duration.as_secs();
    let mins = duration / 60;
    let secs = duration % 60;

    rsx! {
        div { class: "config-panel",
            h2 { class: "config-title", "Block" }

            div { class: "config-field",
                label { "Tempo" }
                div { class: "config-input-row",
                    input {
                        r#type: "number",
                        min: 1,
                        max: 420,
                        value: bpm,
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<u64>() {
                                ctx.write().block_config.bpm = v;
                            }
                        },
                    }
                    span { class: "config-unit", "BPM" }
                }
            }

            div { class: "config-field",
                label { "Duration" }
                div { class: "config-input-row",
                    input {
                        r#type: "number",
                        min: 0,
                        max: 99,
                        value: mins,
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<u64>() {
                                let s = ctx.read().block_config.duration.as_secs() % 60;
                                ctx.write().block_config.duration = std::time::Duration::from_secs(
                                    v * 60 + s,
                                );
                            }
                        },
                    }
                    span { class: "config-unit", "m" }
                    input {
                        r#type: "number",
                        min: 0,
                        max: 59,
                        value: secs,
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<u64>() {
                                let m = ctx.read().block_config.duration.as_secs() / 60;
                                ctx.write().block_config.duration = std::time::Duration::from_secs(
                                    m * 60 + v,
                                );
                            }
                        },
                    }
                    span { class: "config-unit", "s" }
                }
            }

            button {
                class: "play start config-start",
                onclick: move |_| {
                    let cfg = ctx.read().block_config.clone();
                    *session.block_config.write().unwrap() = Some(cfg.clone());
                    session.mode.store(1, Ordering::Relaxed);
                    engine.bpm.store(cfg.bpm, Ordering::Relaxed);
                    session.cmd.store(Cmd::Start as u8, Ordering::Relaxed);
                },
                "▶  START"
            }
        }
    }
}
