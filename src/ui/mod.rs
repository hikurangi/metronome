mod components;
mod helpers;

use crate::constants::{BPM_MAX, BPM_MIN};
use crate::context::AppContext;
use crate::engine::handle::EngineHandle;
use crate::session::config::Mode;
use crate::sound::beat::Beat;
use crate::ui::components::playback_controls::PlaybackControls;
use crate::ui::components::timer::Timer;
use crate::ui::helpers::{ActiveTick, flash_class, subdivision_label};

use dioxus::prelude::*;
use std::ops::Deref;
use std::sync::{Arc, atomic::Ordering};

#[component]
pub fn App() -> Element {
    let engine = use_context::<Arc<EngineHandle>>();
    let mut ctx: Signal<AppContext> = use_context::<Signal<AppContext>>();
    let mode = ctx.read().mode;

    let active_beat = use_context::<Signal<Option<ActiveTick>>>();
    let active_sub = use_context::<Signal<Option<ActiveTick>>>();
    let ring_beat = use_context::<Signal<Option<(usize, bool)>>>();

    let h_bpm_dec = Arc::clone(&engine);
    let h_bpm_inc = Arc::clone(&engine);
    let h_bpm_slider = Arc::clone(&engine);
    let h_beats_dec = Arc::clone(&engine);
    let h_beats_inc = Arc::clone(&engine);
    let h_subs_dec = Arc::clone(&engine);
    let h_subs_inc = Arc::clone(&engine);

    let bpm = ctx.read().bpm;
    let subdivisions = ctx.read().subdivisions;
    let beat_states = ctx.read().beat_states.clone();
    let sub_states = ctx.read().sub_states.clone();
    let beat_ms = 60_000u64 / bpm;

    let bpm_flash_cls = match *active_beat.read() {
        Some(ab) => format!("bpm-number {}", flash_class(ab.beat, ab.parity)),
        None => match *active_sub.read() {
            Some(ab) => format!("bpm-number {}", flash_class(ab.beat, ab.parity)),
            None => "bpm-number".into(),
        },
    };

    rsx! {
        // ── BPM ───────────────────────────────────────────────────────────────
        div { class: "bpm-row",
            div { class: "{bpm_flash_cls}", "{bpm}" }
        }
        div { class: "bpm-label", "BPM" }

        div { class: "bpm-slider-wrap",
            button {
                class: "adj",
                onclick: move |_| {
                    let v = {
                        let mut c = ctx.write();
                        c.bpm = (c.bpm.saturating_sub(1)).max(BPM_MIN);
                        c.bpm
                    };
                    h_bpm_dec.bpm.store(v, Ordering::Relaxed);
                },
                "−"
            }
            input {
                r#type: "range",
                min: BPM_MIN as f64,
                max: BPM_MAX as f64,
                value: bpm,
                oninput: move |e| {
                    if let Ok(val) = e.value().parse::<u64>() {
                        ctx.write().bpm = val;
                        h_bpm_slider.bpm.store(val, Ordering::Relaxed);
                    }
                },
            }
            button {
                class: "adj",
                onclick: move |_| {
                    let v = {
                        let mut c = ctx.write();
                        c.bpm = (c.bpm + 1).min(BPM_MAX);
                        c.bpm
                    };
                    h_bpm_inc.bpm.store(v, Ordering::Relaxed);
                },
                "+"
            }
        }

        // ── Beat grid ─────────────────────────────────────────────────────────
        div { class: "beat-area",
            button {
                class: "adj",
                onclick: move |_| {
                    let mut c = ctx.write();
                    if c.beats_per_bar > 1 {
                        let n = c.beats_per_bar - 1;
                        c.set_beats_per_bar(n);
                        *h_beats_dec.beat_states_pending.write().unwrap() = Some(
                            c.beat_states.clone(),
                        );
                    }
                },
                "−"
            }

            // TODO: factor out
            div { class: "beat-column",
                div { class: "beat-grid",
                    {
                        beat_states
                            .iter()
                            .enumerate()
                            .map(|(i, &beat)| {
                                let is_flash = active_beat
                                    .read()
                                    .deref()
                                    .map_or(false, |ab| ab.idx == i);
                                let flash_cls = active_beat
                                    .read()
                                    .filter(|_| is_flash)
                                    .map_or("", |ab| flash_class(ab.beat, ab.parity));
                                let (is_ring, parity_cls) = ring_beat
                                    .read()
                                    .map_or(
                                        (false, ""),
                                        |(idx, parity)| (idx == i, if parity { "even" } else { "odd" }),
                                    );
                                let state_cls = match beat {
                                    Beat::Accent => "accent",
                                    Beat::Normal => "normal",
                                    _ => "silent",
                                };
                                let cls = format!("beat {state_cls} {flash_cls}");
                                let h = Arc::clone(&engine);
                                rsx! {
                                    div { key: "beat-wrap-{i}", class: "ring-wrap",
                                        button {
                                            key: "beat-{i}",
                                            class: "{cls}",
                                            onclick: move |_| {
                                                let mut c = ctx.write();
                                                c.beat_states[i] = c.beat_states[i].cycle_primary();
                                                *h.beat_states_pending.write().unwrap() = Some(c.beat_states.clone());
                                            },
                                        }
                                        svg {
                                            key: "beat-ring-{i}-{parity_cls}",
                                            class: if is_ring { "sweep {parity_cls}" } else { "sweep hidden" },
                                            style: "animation-duration: {beat_ms}ms",
                                            view_box: "0 0 66 66",
                                            circle {
                                                class: "sweep-fill",
                                                cx: "33",
                                                cy: "33",
                                                r: "30",
                                            }
                                        }
                                    }
                                }
                            })
                    }
                }

                // TODO: factor out
                div { class: "sub-row-wrapper",
                    if subdivisions > 1 {
                        div { class: "sub-row",
                            {
                                sub_states
                                    .iter()
                                    .enumerate()
                                    .map(|(i, &beat)| {
                                        let is_flash = active_sub.read().deref().map_or(false, |ab| ab.idx == i);
                                        let flash_cls = active_sub
                                            .read()
                                            .filter(|_| is_flash)
                                            .map_or("", |ab| flash_class(ab.beat, ab.parity));
                                        let state_cls = match beat {
                                            Beat::SubAccent => "accent",
                                            Beat::SubNormal => "normal",
                                            _ => "silent",
                                        };
                                        let cls = format!("sub {state_cls} {flash_cls}");
                                        let h = Arc::clone(&engine);
                                        rsx! {
                                            div { key: "sub-wrap-{i}", class: "ring-wrap",
                                                button {
                                                    key: "sub-{i}",
                                                    class: "{cls}",
                                                    onclick: move |_| {
                                                        let mut c = ctx.write();
                                                        let current = c.sub_states[i];
                                                        c.sub_states[i] = current.cycle_sub();
                                                        *h.sub_states_pending.write().unwrap() = Some(c.sub_states.clone());
                                                    },
                                                }
                                            }
                                        }
                                    })
                            }
                        }
                    }
                }
            }

            button {
                class: "adj",
                onclick: move |_| {
                    let mut c = ctx.write();
                    let n = c.beats_per_bar + 1;
                    c.set_beats_per_bar(n);
                    *h_beats_inc.beat_states_pending.write().unwrap() = Some(c.beat_states.clone());
                },
                "+"
            }
        }

        // ── Subdivision control ───────────────────────────────────────────────
        div { class: "ctrl-group",
            button {
                class: "adj sm",
                onclick: move |_| {
                    let mut c = ctx.write();
                    if c.subdivisions > 1 {
                        let n = c.subdivisions - 1;
                        c.set_subdivisions(n);
                    }
                    h_subs_dec.subdivisions.store(c.subdivisions, Ordering::Relaxed);
                    *h_subs_dec.sub_states_pending.write().unwrap() = Some(c.sub_states.clone());
                },
                "−"
            }
            span { class: "ctrl-label", "{subdivision_label(subdivisions)}" }
            button {
                class: "adj sm",
                onclick: move |_| {
                    let mut c = ctx.write();
                    let n = c.subdivisions + 1;
                    c.set_subdivisions(n);
                    h_subs_inc.subdivisions.store(c.subdivisions, Ordering::Relaxed);
                    *h_subs_inc.sub_states_pending.write().unwrap() = Some(c.sub_states.clone());
                },
                "+"
            }
        }

        if mode != Mode::Infinity {
            Timer {}
        }

        PlaybackControls {}
    }
}
