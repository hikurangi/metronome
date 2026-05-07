use std::{
    ops::Deref,
    sync::{Arc, atomic::Ordering},
    time::Duration,
};

use dioxus::prelude::*;

use crate::{
    context::AppContext,
    engine::handle::EngineHandle,
    session::{
        config::{Mode, SessionStatus},
        handle::{Phase, SessionHandle},
    },
    sound::beat::Beat,
    ui::{
        components::{
            beat_control_buttons::{BeatDecrementButton, BeatIncrementButton},
            bpm_slider::BPMSlider,
            playback_controls::PlaybackControls,
            timer::Timer,
        },
        helpers::{ActiveTick, flash_class, subdivision_label},
    },
};

#[component]
pub fn Metronome() -> Element {
    let engine = use_context::<Arc<EngineHandle>>();
    let session = use_context::<Arc<SessionHandle>>();
    let mut ctx = use_context::<Signal<AppContext>>();

    let bpm = ctx.read().bpm;

    let mut session_status = use_context_provider(|| Signal::new(SessionStatus::Inactive));

    let h_subs_dec = Arc::clone(&engine);
    let h_subs_inc = Arc::clone(&engine);

    let subdivisions = ctx.read().subdivisions;
    let beat_states = ctx.read().beat_states.clone();
    let sub_states = ctx.read().sub_states.clone();
    let beat_ms = 60_000u64 / bpm;

    let active_beat = use_signal(|| Option::<ActiveTick>::None);
    let active_sub = use_signal(|| Option::<ActiveTick>::None);
    let ring_beat = use_signal(|| Option::<(usize, bool)>::None);

    let bpm_flash_cls = match *active_beat.read() {
        Some(ab) => format!("bpm-number {}", flash_class(ab.beat, ab.parity)),
        None => match *active_sub.read() {
            Some(ab) => format!("bpm-number {}", flash_class(ab.beat, ab.parity)),
            None => "bpm-number".into(),
        },
    };

    let s = Arc::clone(&session);
    use_effect(move || {
        let s = Arc::clone(&s);
        spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(100)).await;
                if Phase::from(s.phase.load(Ordering::Relaxed)) == Phase::Finished {
                    session_status.set(SessionStatus::Inactive);
                }
            }
        });
    });

    rsx! {
        // ── BPM ───────────────────────────────────────────────────────────────
        div { class: "bpm-row",
            div { class: "{bpm_flash_cls}", "{bpm}" }
        }
        div { class: "bpm-label", "BPM" }

        BPMSlider {}

        // ── Beat grid ─────────────────────────────────────────────────────────
        div { class: "beat-area",

            BeatDecrementButton {}

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
                                    .is_some_and(|ab| ab.idx == i);
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

                div { class: "sub-row-wrapper",
                    if subdivisions > 1 {
                        div { class: "sub-row",
                            {
                                sub_states
                                    .iter()
                                    .enumerate()
                                    .map(|(i, &beat)| {
                                        let is_flash = active_sub.read().deref().is_some_and(|ab| ab.idx == i);
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

            BeatIncrementButton {}
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
        if !matches!(ctx.read().mode, Mode::Infinity) {
            Timer {}
        }

        PlaybackControls {}
    }
}
