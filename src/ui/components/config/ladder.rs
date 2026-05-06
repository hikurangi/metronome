use crate::context::AppContext;
use crate::engine::handle::EngineHandle;
use crate::session::handle::{Cmd, SessionHandle};
use dioxus::prelude::*;
use std::sync::{Arc, atomic::Ordering};

#[component]
pub fn LadderConfigPanel() -> Element {
    let mut ctx = use_context::<Signal<AppContext>>();
    let engine = use_context::<Arc<EngineHandle>>();
    let session = use_context::<Arc<SessionHandle>>();

    let cfg = ctx.read().ladder_config.clone();

    rsx! {
        div { class: "config-panel",
            h2 { class: "config-title", "Ladder" }

            div { class: "config-field",
                label { "Start tempo" }
                div { class: "config-input-row",
                    input {
                        r#type: "number",
                        min: 1,
                        max: 420,
                        value: cfg.start_bpm,
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<u64>() {
                                ctx.write().ladder_config.start_bpm = v;
                            }
                        },
                    }
                    span { class: "config-unit", "BPM" }
                }
            }

            div { class: "config-field",
                label { "Step" }
                div { class: "config-input-row",
                    input {
                        r#type: "number",
                        min: 0,
                        max: 99,
                        value: cfg.step_duration.as_secs() / 60,
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<u64>() {
                                let s = ctx.read().ladder_config.step_duration.as_secs() % 60;
                                ctx.write().ladder_config.step_duration = std::time::Duration::from_secs(
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
                        value: cfg.step_duration.as_secs() % 60,
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<u64>() {
                                let m = ctx.read().ladder_config.step_duration.as_secs() / 60;
                                ctx.write().ladder_config.step_duration = std::time::Duration::from_secs(
                                    m * 60 + v,
                                );
                            }
                        },
                    }
                    span { class: "config-unit", "s" }
                }
            }

            div { class: "config-field",
                label { "Rest" }
                div { class: "config-input-row",
                    input {
                        r#type: "number",
                        min: 0,
                        max: 99,
                        value: cfg.rest_duration.as_secs() / 60,
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<u64>() {
                                let s = ctx.read().ladder_config.rest_duration.as_secs() % 60;
                                ctx.write().ladder_config.rest_duration = std::time::Duration::from_secs(
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
                        value: cfg.rest_duration.as_secs() % 60,
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<u64>() {
                                let m = ctx.read().ladder_config.rest_duration.as_secs() / 60;
                                ctx.write().ladder_config.rest_duration = std::time::Duration::from_secs(
                                    m * 60 + v,
                                );
                            }
                        },
                    }
                    span { class: "config-unit", "s" }
                }
            }

            div { class: "config-field",
                label { "Increment" }
                div { class: "config-input-row",
                    input {
                        r#type: "number",
                        min: -100,
                        max: 100,
                        value: cfg.tempo_increment,
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<i64>() {
                                ctx.write().ladder_config.tempo_increment = v;
                            }
                        },
                    }
                    span { class: "config-unit", "BPM / step" }
                }
            }

            div { class: "config-field",
                label { "Cycles" }
                div { class: "config-input-row",
                    input {
                        r#type: "number",
                        min: 1,
                        max: 99,
                        value: cfg.cycle_count,
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<usize>() {
                                ctx.write().ladder_config.cycle_count = v;
                            }
                        },
                    }
                }
            }

            button {
                class: "play start config-start",
                onclick: move |_| {
                    let cfg = ctx.read().ladder_config.clone();
                    *session.ladder_config.write().unwrap() = Some(cfg.clone());
                    session.mode.store(2, Ordering::Relaxed);
                    engine.bpm.store(cfg.start_bpm, Ordering::Relaxed);
                    session.cmd.store(Cmd::Start as u8, Ordering::Relaxed);
                },
                "▶  START"
            }
        }
    }
}
