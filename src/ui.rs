use crate::constants::{BPM_MAX, BPM_MIN};
use crate::context::AppContext;
use crate::engine::handle::EngineHandle;
use crate::sound::beat::Beat;
use dioxus::prelude::*;
use std::ops::Deref;
use std::sync::{Arc, atomic::Ordering};
use std::time::{Duration, Instant};

// ── Theme ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Dark,
    Light,
    System,
}

impl Theme {
    fn next(self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::System,
            Theme::System => Theme::Dark,
        }
    }
    fn app_class(self) -> &'static str {
        match self {
            Theme::Dark => "app dark",
            Theme::Light => "app light",
            Theme::System => "app",
        }
    }
    fn icon(self) -> &'static str {
        match self {
            Theme::Light => "☀",
            Theme::Dark => "☾",
            Theme::System => "◐",
        }
    }
}

// ── Flash helpers ─────────────────────────────────────────────────────────────

fn flash_class(beat: Beat, parity: bool) -> &'static str {
    match (beat, parity) {
        (Beat::Accent, true) => "flash-hi-a",
        (Beat::Accent, false) => "flash-hi-b",
        (Beat::Normal, true) => "flash-mid-a",
        (Beat::Normal, false) => "flash-mid-b",
        (Beat::SubAccent, true) | (Beat::SubNormal, true) => "flash-lo-a",
        (Beat::SubAccent, false) | (Beat::SubNormal, false) => "flash-lo-b",
        _ => "",
    }
}

#[derive(Clone, Copy)]
struct ActiveTick {
    idx: usize,
    beat: Beat,
    parity: bool,
}

// ── Subdivision label ─────────────────────────────────────────────────────────

fn subdivision_label(n: usize) -> String {
    match n {
        1 => "Downbeats only".into(),
        2 => "Eighths".into(),
        3 => "Triplets".into(),
        4 => "Sixteenths".into(),
        5 => "Quintuplets".into(),
        6 => "Sextuplets".into(),
        7 => "Septuplets".into(),
        8 => "32nds".into(),
        n => format!("×{n}"),
    }
}

// ── Component ─────────────────────────────────────────────────────────────────

#[component]
pub fn App() -> Element {
    let handle = use_context::<Arc<EngineHandle>>();
    let mut ctx = use_signal(AppContext::new);
    let mut theme = use_signal(|| Theme::Light);

    // separate signals for beats and subs
    let mut active_beat = use_signal(|| Option::<ActiveTick>::None);
    let mut active_sub = use_signal(|| Option::<ActiveTick>::None);
    let mut ring_beat = use_signal(|| Option::<(usize, bool)>::None); // persists for full beat duration

    // pre-clone handles
    let h_bpm_dec = Arc::clone(&handle);
    let h_bpm_inc = Arc::clone(&handle);
    let h_bpm_slider = Arc::clone(&handle);
    let h_running = Arc::clone(&handle);
    let h_beats_dec = Arc::clone(&handle);
    let h_beats_inc = Arc::clone(&handle);
    let h_subs_dec = Arc::clone(&handle);
    let h_subs_inc = Arc::clone(&handle);

    // ── Tick poller ───────────────────────────────────────────────────────────
    let h_poll = Arc::clone(&handle);
    use_effect(move || {
        let h = Arc::clone(&h_poll);
        spawn(async move {
            let mut last_beat_tick = 0u64;
            let mut last_sub_tick = 0u64;
            let mut beat_clear_at: Option<Instant> = None;
            let mut sub_clear_at: Option<Instant> = None;

            loop {
                tokio::time::sleep(Duration::from_millis(8)).await;
                let now = Instant::now();

                let beat_tick = h.beat_tick_count.load(Ordering::Relaxed);
                let sub_tick = h.sub_tick_count.load(Ordering::Relaxed);

                if beat_tick != last_beat_tick {
                    let idx = h.current_beat_idx.load(Ordering::Relaxed);
                    let beat = Beat::from(h.current_beat_type.load(Ordering::Relaxed));
                    let parity = beat_tick % 2 == 0;
                    active_beat.set(Some(ActiveTick { idx, beat, parity }));
                    ring_beat.set(Some((idx, parity)));
                    beat_clear_at = Some(now + Duration::from_millis(120));
                    last_beat_tick = beat_tick;
                }

                if sub_tick != last_sub_tick {
                    let idx = h.current_sub_idx.load(Ordering::Relaxed);
                    let beat = Beat::from(h.current_beat_type.load(Ordering::Relaxed));
                    let parity = sub_tick % 2 == 0;
                    active_sub.set(Some(ActiveTick { idx, beat, parity }));
                    sub_clear_at = Some(now + Duration::from_millis(80));
                    last_sub_tick = sub_tick;
                }

                if let Some(t) = beat_clear_at {
                    if now >= t {
                        active_beat.set(None);
                        beat_clear_at = None;
                    }
                }
                if let Some(t) = sub_clear_at {
                    if now >= t {
                        active_sub.set(None);
                        sub_clear_at = None;
                    }
                }
            }
        });
    });

    // ── Derive display values ─────────────────────────────────────────────────
    let bpm = ctx.read().bpm;
    let is_running = ctx.read().is_running;
    let subdivisions = ctx.read().subdivisions;
    let beat_states = ctx.read().beat_states.clone();
    let sub_states = ctx.read().sub_states.clone();
    let beat_ms = 60_000u64 / bpm;

    // BPM flash — fires on any tick
    let bpm_flash_cls = match *active_beat.read() {
        Some(ab) => format!("bpm-number {}", flash_class(ab.beat, ab.parity)),
        None => match *active_sub.read() {
            Some(ab) => format!("bpm-number {}", flash_class(ab.beat, ab.parity)),
            None => "bpm-number".into(),
        },
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }

        div { class: theme.read().app_class(),

            // ── Theme toggle ─────────────────────────────────────────────────
            button {
                class: "theme-btn",
                onclick: move |_| {
                    let t = theme.read().next();
                    theme.set(t);
                },
                "{theme.read().icon()}"
            }

            // ── BPM ──────────────────────────────────────────────────────────
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

            // ── Beat row ─────────────────────────────────────────────────────
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

                div { class: "beat-column",
                    div { class: "beat-grid",
                        {
                            beat_states
                                .iter()
                                .enumerate()
                                .map(|(i, &beat)| {
                                    let is_flash = active_beat
                                        .read().is_some_and(|ab| ab.idx == i);
                                    let flash_cls = active_beat
                                        .read()
                                        .filter(|_| is_flash)
                                        .map(|ab| flash_class(ab.beat, ab.parity))
                                        .unwrap_or("");
                                    let (is_ring, parity_cls) = ring_beat
                                        .read()
                                        .deref()
                                        .map_or(
                                            (false, ""),
                                            |(idx, parity)| {
                                                (idx == i, if parity { "even" } else { "odd" })
                                            },
                                        );
                                    let state_cls = match beat {
                                        Beat::Accent => "accent",
                                        Beat::Normal => "normal",
                                        _ => "silent",
                                    };
                                    let cls = format!("beat {state_cls} {flash_cls}");
                                    let h = Arc::clone(&handle);
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
                                                xmlns: "http://www.w3.org/2000/svg",
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
                                            let is_flash = active_sub
                                            .read().is_some_and(|ab| ab.idx == i);
                                            let flash_cls = active_sub
                                                .read()
                                                .filter(|_| is_flash)
                                                .map(|ab| flash_class(ab.beat, ab.parity))
                                                .unwrap_or("");
                                            let state_cls = match beat {
                                                Beat::SubAccent => "accent",
                                                Beat::SubNormal => "normal",
                                                _ => "silent",
                                            };
                                            let cls = format!("sub {state_cls} {flash_cls}");
                                            let h = Arc::clone(&handle);
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

            // ── Subdivision control ──────────────────────────────────────────
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

            // ── Start / Stop ─────────────────────────────────────────────────
            button {
                class: if is_running { "play stop" } else { "play start" },
                onclick: move |_| {
                    let next = {
                        let mut c = ctx.write();
                        c.is_running = !c.is_running;
                        c.is_running
                    };
                    h_running.running.store(next, Ordering::Relaxed);
                },
                if is_running {
                    "■  STOP"
                } else {
                    "▶  START"
                }
            }
        }
    }
}
