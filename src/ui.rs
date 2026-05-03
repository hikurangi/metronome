use crate::constants::{BPM_MAX, BPM_MIN};
use crate::sound::beat::Beat;
use crate::{context::AppContext, engine::handle::EngineHandle};
use dioxus::prelude::*;
use std::sync::{Arc, atomic::Ordering};

fn cycle_beat(beat: Beat) -> Beat {
    match beat {
        Beat::Silent => Beat::Normal,
        Beat::Normal => Beat::Accent,
        Beat::Accent => Beat::Silent,
    }
}

fn beat_class(beat: &Beat, large: bool) -> &'static str {
    let size = if large { "beat" } else { "sub" };
    match beat {
        Beat::Accent => match size {
            "beat" => "beat accent",
            _ => "sub accent",
        },
        Beat::Normal => match size {
            "beat" => "beat normal",
            _ => "sub normal",
        },
        Beat::Silent => match size {
            "beat" => "beat silent",
            _ => "sub silent",
        },
    }
}

#[component]
pub fn App() -> Element {
    let handle = use_context::<Arc<EngineHandle>>();
    let mut ctx = use_signal(AppContext::new);

    let h_bpm_dec = Arc::clone(&handle);
    let h_bpm_inc = Arc::clone(&handle);
    let h_running = Arc::clone(&handle);
    let h_beats_dec = Arc::clone(&handle);
    let h_beats_inc = Arc::clone(&handle);
    let h_subs_dec = Arc::clone(&handle);
    let h_subs_inc = Arc::clone(&handle);

    let ctx_bpm = ctx.read().bpm;
    let ctx_is_running = ctx.read().is_running;
    let ctx_beats_per_bar = ctx.read().beats_per_bar;
    let ctx_subdivisions = ctx.read().subdivisions;
    let ctx_beat_states = ctx.read().beat_states.clone();
    let ctx_sub_states = ctx.read().sub_states.clone();

    rsx! {
        style { {CSS} }
        div { class: "app",

            // ── BPM ──────────────────────────────────────────────────────────
            div { class: "bpm-row",
                button { class: "adj", onclick: move |_| {
                    let bpm = {
                        let mut c = ctx.write();
                        c.bpm = (c.bpm.saturating_sub(1)).max(BPM_MIN);
                        c.bpm
                    };
                    h_bpm_dec.bpm.store(bpm, Ordering::Relaxed);
                }, "−" }
                div { class: "bpm-number", "{ctx_bpm}" }
                button { class: "adj", onclick: move |_| {
                    let bpm = {
                        let mut c = ctx.write();
                        c.bpm = (c.bpm + 1).min(BPM_MAX);
                        c.bpm
                    };
                    h_bpm_inc.bpm.store(bpm, Ordering::Relaxed);
                }, "+" }
            }
            div { class: "bpm-label", "BPM" }

            // ── Beat grid ────────────────────────────────────────────────────
            div { class: "beat-row",
                {ctx_beat_states.iter().enumerate().map(|(i, beat)| {
                    let cls = beat_class(beat, true);
                    let h = Arc::clone(&handle);
                    rsx! {
                        button { key: "{i}", class: "{cls}",
                            onclick: move |_| {
                                let pattern = {
                                    let mut c = ctx.write();
                                    c.beat_states[i] = cycle_beat(c.beat_states[i].clone());
                                    c.generate_pattern()
                                };
                                *h.pattern.write().unwrap() = pattern;
                            }
                        }
                    }
                })}
            }

            // ── Subdivision row ──────────────────────────────────────────────
            if ctx_subdivisions > 1 {
                div { class: "sub-row",
                    {ctx_sub_states.iter().enumerate().map(|(i, beat)| {
                        let cls = beat_class(beat, false);
                        let h = Arc::clone(&handle);
                        rsx! {
                            button { key: "{i}", class: "{cls}",
                                onclick: move |_| {
                                    let pattern = {
                                        let mut c = ctx.write();
                                        c.sub_states[i] = cycle_beat(c.sub_states[i].clone());
                                        c.generate_pattern()
                                    };
                                    *h.pattern.write().unwrap() = pattern;
                                }
                            }
                        }
                    })}
                }
            }

            // ── Controls ─────────────────────────────────────────────────────
            div { class: "controls",
                div { class: "ctrl-group",
                    button { class: "adj sm", onclick: move |_| {
                        let pattern = {
                            let mut c = ctx.write();
                            if c.beats_per_bar > 1 {
                                let one_fewer = c.beats_per_bar - 1;
                                c.set_beats_per_bar(one_fewer);
                            }
                            c.generate_pattern()
                        };
                        *h_beats_dec.pattern.write().unwrap() = pattern;
                    }, "−" }
                    span { class: "ctrl-label", "{ctx_beats_per_bar} beats" }
                    button { class: "adj sm", onclick: move |_| {
                        let pattern = {
                            let mut c = ctx.write();
                            let one_more = c.beats_per_bar + 1;
                            c.set_beats_per_bar(one_more);
                            c.generate_pattern()
                        };
                        *h_beats_inc.pattern.write().unwrap() = pattern;
                    }, "+" }
                }
                div { class: "ctrl-group",
                    button { class: "adj sm", onclick: move |_| {
                        let pattern = {
                            let mut c = ctx.write();
                            if c.subdivisions > 1 {
                                let one_fewer = c.subdivisions - 1;
                                c.set_subdivisions(one_fewer);
                            }
                            c.generate_pattern()
                        };
                        *h_subs_dec.pattern.write().unwrap() = pattern;
                    }, "−" }
                    span { class: "ctrl-label", "÷ {ctx_subdivisions}" }
                    button { class: "adj sm", onclick: move |_| {
                        let pattern = {
                            let mut c = ctx.write();
                            let one_more = c.subdivisions + 1;
                            c.set_subdivisions(one_more);
                            c.generate_pattern()
                        };
                        *h_subs_inc.pattern.write().unwrap() = pattern;
                    }, "+" }
                }
            }

            // ── Start / Stop ─────────────────────────────────────────────────
            button {
                class: if ctx_is_running { "play stop" } else { "play start" },
                onclick: move |_| {
                    let next = {
                        let mut c = ctx.write();
                        c.is_running = !c.is_running;
                        c.is_running
                    };
                    h_running.running.store(next, Ordering::Relaxed);
                },
                if ctx_is_running { "■  STOP" } else { "▶  START" }
            }
        }
    }
}

const CSS: &str = r#"
@import url('https://fonts.googleapis.com/css2?family=Bebas+Neue&family=DM+Mono:ital,wght@0,300;0,400;1,300&display=swap');

*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

:root {
    --bg:     #080808;
    --fg:     #f0ede8;
    --dim:    #1c1c1c;
    --border: #2e2e2e;
    --mid:    #888;
}

body { background: var(--bg); }

.app {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    gap: 2.4rem;
    background: var(--bg);
    color: var(--fg);
    font-family: 'DM Mono', monospace;
    user-select: none;
    -webkit-user-select: none;
}

/* ── BPM ──────────────────────────────────────────────── */
.bpm-row {
    display: flex;
    align-items: center;
    gap: 2rem;
}

.bpm-number {
    font-family: 'Bebas Neue', sans-serif;
    font-size: 10rem;
    line-height: 0.9;
    letter-spacing: -0.01em;
    min-width: 4ch;
    text-align: center;
    color: var(--fg);
}

.bpm-label {
    font-size: 0.6rem;
    letter-spacing: 0.5em;
    color: var(--mid);
    text-transform: uppercase;
    margin-top: -2rem;
}

.adj {
    background: none;
    border: 1px solid var(--border);
    color: var(--fg);
    font-family: 'DM Mono', monospace;
    font-size: 1.4rem;
    width: 3rem;
    height: 3rem;
    cursor: pointer;
    transition: background 0.08s, border-color 0.08s;
    line-height: 1;
}

.adj:hover  { border-color: var(--fg); }
.adj:active { background: var(--fg); color: var(--bg); }
.adj.sm     { width: 2rem; height: 2rem; font-size: 1rem; }

/* ── Beat / Sub circles ───────────────────────────────── */
.beat-row, .sub-row {
    display: flex;
    gap: 0.9rem;
    align-items: center;
}

.sub-row { gap: 0.9rem; margin-top: -1.4rem; }

.beat, .sub {
    border-radius: 50%;
    cursor: pointer;
    transition: transform 0.08s, background 0.1s;
    border: none;
    outline: none;
}

.beat { width: 3.6rem; height: 3.6rem; }
.sub  { width: 1.4rem; height: 1.4rem; }

.beat.accent { background: var(--fg); }
.beat.silent { background: transparent; box-shadow: inset 0 0 0 2px var(--fg); }
.beat.normal { background: var(--dim); box-shadow: inset 0 0 0 1px var(--border); }

.sub.accent  { background: var(--fg); }
.sub.silent  { background: transparent; box-shadow: inset 0 0 0 1.5px var(--fg); }
.sub.normal  { background: var(--dim); box-shadow: inset 0 0 0 1px var(--border); }

.beat:active, .sub:active { transform: scale(0.88); }

/* ── Controls ─────────────────────────────────────────── */
.controls {
    display: flex;
    gap: 3rem;
}

.ctrl-group {
    display: flex;
    align-items: center;
    gap: 0.6rem;
}

.ctrl-label {
    font-size: 0.7rem;
    letter-spacing: 0.12em;
    color: var(--fg);
    min-width: 7ch;
    text-align: center;
}

/* ── Play / Stop ──────────────────────────────────────── */
.play {
    font-family: 'DM Mono', monospace;
    font-size: 0.75rem;
    letter-spacing: 0.35em;
    padding: 1rem 3.5rem;
    cursor: pointer;
    transition: opacity 0.1s, transform 0.08s;
    border: none;
}

.play.start { background: var(--fg); color: var(--bg); }
.play.stop  { background: transparent; box-shadow: inset 0 0 0 1px var(--fg); color: var(--fg); }

.play:hover  { opacity: 0.82; }
.play:active { transform: scale(0.96); }
"#;
