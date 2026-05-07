// src/ui/components/config/ladder.rs

use crate::context::AppContext;
use crate::engine::handle::EngineHandle;
use crate::session::config::SessionStatus;
use crate::session::handle::{Cmd, SessionHandle};
use dioxus::prelude::*;
use std::sync::{Arc, atomic::Ordering};
use std::time::Duration;

fn duration_mins(d: Duration) -> u64 {
    d.as_secs() / 60
}
fn duration_secs(d: Duration) -> u64 {
    d.as_secs() % 60
}
fn from_mins_secs(m: u64, s: u64) -> Duration {
    Duration::from_secs(m * 60 + s)
}

#[component]
pub fn LadderConfigPanel() -> Element {
    let mut ctx = use_context::<Signal<AppContext>>();
    let engine = use_context::<Arc<EngineHandle>>();
    let session = use_context::<Arc<SessionHandle>>();
    let mut session_status = use_context::<Signal<SessionStatus>>();

    let cfg = ctx.read().ladder_config.clone();

    rsx! {
        div { class: "config-panel",
            h2 { class: "config-title", "Ladder" }

            div { class: "config-fields",

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
                                    ctx.write().ladder_config.start_bpm = v.clamp(1, 420);
                                }
                            },
                        }
                        span { class: "config-unit", "BPM" }
                    }
                }

                div { class: "config-field",
                    label { "Step duration" }
                    div { class: "config-input-row",
                        input {
                            r#type: "number",
                            min: 0,
                            max: 99,
                            value: duration_mins(cfg.step_duration),
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<u64>() {
                                    let s = duration_secs(ctx.read().ladder_config.step_duration);
                                    ctx.write().ladder_config.step_duration = from_mins_secs(v, s);
                                }
                            },
                        }
                        span { class: "config-separator", ":" }
                        input {
                            r#type: "number",
                            min: 0,
                            max: 59,
                            value: duration_secs(cfg.step_duration),
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<u64>() {
                                    let m = duration_mins(ctx.read().ladder_config.step_duration);
                                    ctx.write().ladder_config.step_duration = from_mins_secs(m, v);
                                }
                            },
                        }
                        span { class: "config-unit", "min : sec" }
                    }
                }

                div { class: "config-field",
                    label { "Rest between steps" }
                    div { class: "config-input-row",
                        input {
                            r#type: "number",
                            min: 0,
                            max: 99,
                            value: duration_mins(cfg.rest_duration),
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<u64>() {
                                    let s = duration_secs(ctx.read().ladder_config.rest_duration);
                                    ctx.write().ladder_config.rest_duration = from_mins_secs(v, s);
                                }
                            },
                        }
                        span { class: "config-separator", ":" }
                        input {
                            r#type: "number",
                            min: 0,
                            max: 59,
                            value: duration_secs(cfg.rest_duration),
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<u64>() {
                                    let m = duration_mins(ctx.read().ladder_config.rest_duration);
                                    ctx.write().ladder_config.rest_duration = from_mins_secs(m, v);
                                }
                            },
                        }
                        span { class: "config-unit", "min : sec" }
                    }
                }

                div { class: "config-field",
                    label { "Tempo increment" }
                    div { class: "config-input-row",
                        input {
                            r#type: "number",
                            class: "wide",
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
                                    ctx.write().ladder_config.cycle_count = v.max(1);
                                }
                            },
                        }
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
                    session_status.set(SessionStatus::Active)
                },
                "▶  START"
            }
        }
    }
}
