use crate::constants::{BPM_MAX, BPM_MIN};
// src/ui.rs
use crate::context::AppContext;
use crate::engine::handle::EngineHandle;
use crate::sound::beat::Beat;
use dioxus::prelude::*;
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

// Parity-based flash class forces CSS animation restart even on same beat
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
struct ActiveBeat {
    beat_idx: usize,
    beat: Beat,
    parity: bool,
    fired_at: Instant,
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
    let mut theme = use_signal(|| Theme::Dark);
    let mut active = use_signal(|| Option::<ActiveBeat>::None);
    let mut current_beat_idx = use_signal(|| Option::<(usize, bool)>::None); // (idx, parity)

    // pre-clone handles
    let h_bpm_dec = Arc::clone(&handle);
    let h_bpm_inc = Arc::clone(&handle);
    let h_bpm_slider = Arc::clone(&handle);
    let h_running = Arc::clone(&handle);
    let h_beats_dec = Arc::clone(&handle);
    let h_beats_inc = Arc::clone(&handle);
    let h_subs_dec = Arc::clone(&handle);
    let h_subs_inc = Arc::clone(&handle);

    // ── Tick poller coroutine ─────────────────────────────────────────────────
    let h_poll = Arc::clone(&handle);
    use_effect(move || {
        let h_poll = Arc::clone(&h_poll);
        spawn(async move {
            let mut last_tick = 0u64;
            let mut clear_at: Option<Instant> = None;
            loop {
                tokio::time::sleep(Duration::from_millis(8)).await;
                let now = Instant::now();
                let tick = h_poll.tick_count.load(Ordering::Relaxed);
                if tick != last_tick {
                    let idx = h_poll.current_beat_idx.load(Ordering::Relaxed);
                    let beat = Beat::from(h_poll.current_beat_type.load(Ordering::Relaxed));
                    let parity = tick.is_multiple_of(2);
                    active.set(Some(ActiveBeat {
                        beat_idx: idx,
                        beat,
                        parity,
                        fired_at: now,
                    }));
                    clear_at = Some(now + Duration::from_millis(120));

                    // ring — persists until next tick
                    current_beat_idx.set(Some((idx, parity)));

                    last_tick = tick;
                }
                if let Some(t) = clear_at
                    && now >= t
                {
                    active.set(None);
                    clear_at = None;
                }
            }
        });
    });

    // ── Derive display values ─────────────────────────────────────────────────
    let bpm = ctx.read().bpm;
    let is_running = ctx.read().is_running;
    // let beats_per_bar = ctx.read().beats_per_bar;
    let subdivisions = ctx.read().subdivisions;
    let beat_states = ctx.read().beat_states.clone();
    let sub_states = ctx.read().sub_states.clone();

    let (active_idx, bpm_flash_cls) = match *active.read() {
        Some(ab) => {
            let fc = flash_class(ab.beat, ab.parity);
            (Some(ab.beat_idx), format!("bpm-number {fc}"))
        }
        None => (None, "bpm-number".into()),
    };

    let beat_ms = 60_000u64 / bpm;
    // let sub_ms = if subdivisions > 1 {
    //     beat_ms / subdivisions as u64
    // } else {
    //     beat_ms
    // };

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
                                    let is_active = active_idx
                                        .map(|idx| { idx % subdivisions == 0 && idx / subdivisions == i })
                                        .unwrap_or(false);
                                    let state_cls = match beat {
                                        Beat::Accent => "accent",
                                        Beat::Normal => "normal",
                                        _ => "silent",
                                    };
                                    let flash_cls = active
                                        .read()
                                        .filter(|_| is_active)
                                        .map(|ab| flash_class(ab.beat, ab.parity))
                                        .unwrap_or("");

                                    let (is_ring_active, parity_cls) = current_beat_idx
                                        .read() // for beats
                                        .as_ref()
                                        .map(|(idx, parity)| {
                                            let is_this = *idx == i * subdivisions;
                                            (is_this, if *parity { "even" } else { "odd" })
                                        })
                                        .unwrap_or((false, ""));

                                    let cls = format!("beat {state_cls} {flash_cls}");
                                    let h = Arc::clone(&handle);
                                    rsx! {
                                        div { class: "ring-wrap",
                                            button {
                                                key: "{i}",
                                                class: "{cls}",
                                                onclick: move |_| {
                                                    let mut c = ctx.write();
                                                    c.beat_states[i] = c.beat_states[i].cycle_primary();
                                                    *h.beat_states_pending.write().unwrap() = Some(c.beat_states.clone());
                                                },
                                            }
                                            if is_ring_active {
                                                svg {
                                                    key: "beat-ring-{i}-{parity_cls}",
                                                    class: if is_ring_active { "sweep {parity_cls}" } else { "sweep hidden" },
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
                                            let is_active = active_idx
                                                .map(|idx| idx % subdivisions == i + 1)
                                                .unwrap_or(false);
                                            let state_cls = match beat {
                                                Beat::SubAccent => "accent",
                                                Beat::SubNormal => "normal",
                                                _ => "silent",
                                            };
                                            let flash_cls = active
                                                .read()
                                                .filter(|_| is_active)
                                                .map(|ab| flash_class(ab.beat, ab.parity))
                                                .unwrap_or("");

                                            let (is_ring_active, parity_cls) = current_beat_idx
                                                .read() // for beats
                                                .as_ref()
                                                .map(|(idx, parity)| {
                                                    let is_this = *idx == i * subdivisions;
                                                    (is_this, if *parity { "even" } else { "odd" })
                                                })
                                                .unwrap_or((false, ""));
                                            let cls = format!("sub {state_cls} {flash_cls}");
                                            let h = Arc::clone(&handle);
                                            rsx! {
                                                div { class: "ring-wrap",
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
                                                    // render whenever this beat is the current position
                                                    svg {
                                                        key: "beat-ring-{i}-{parity_cls}",
                                                        class: if is_ring_active { "sweep {parity_cls}" } else { "sweep hidden" },
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
