use crate::session::handle::{Phase, SessionHandle};
use dioxus::prelude::*;
use std::sync::{Arc, atomic::Ordering};

fn fmt_duration(ms: u64) -> String {
    let total_secs = ms / 1000;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    if mins > 0 {
        format!("{mins}:{secs:02}")
    } else {
        format!("{secs}")
    }
}

#[derive(Clone, Copy, PartialEq)]
enum TimerDisplay {
    Countdown,
    Countup,
}

impl TimerDisplay {
    fn toggle(self) -> Self {
        match self {
            TimerDisplay::Countdown => TimerDisplay::Countup,
            TimerDisplay::Countup => TimerDisplay::Countdown,
        }
    }
}

#[component]
pub fn Timer() -> Element {
    let session = use_context::<Arc<SessionHandle>>();

    // polled state
    let mut phase = use_signal(|| Phase::Idle);
    let mut session_elapsed = use_signal(|| 0u64);
    let mut session_total = use_signal(|| 0u64);
    let mut step_elapsed = use_signal(|| 0u64);
    let mut step_total = use_signal(|| 0u64);
    let mut current_step = use_signal(|| 0usize);
    let mut total_steps = use_signal(|| 0usize);
    let mut is_paused = use_signal(|| false);

    // UI-only state
    let mut display = use_signal(|| TimerDisplay::Countdown);

    // ── Poll session handle ~60fps ─────────────────────────────────────────
    let h = Arc::clone(&session);
    use_effect(move || {
        let h = Arc::clone(&h);
        spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(16)).await;
                phase.set(Phase::from(h.phase.load(Ordering::Relaxed)));
                session_elapsed.set(h.session_elapsed.load(Ordering::Relaxed));
                session_total.set(h.session_total.load(Ordering::Relaxed));
                step_elapsed.set(h.step_elapsed.load(Ordering::Relaxed));
                step_total.set(h.step_total.load(Ordering::Relaxed));
                current_step.set(h.current_step.load(Ordering::Relaxed));
                total_steps.set(h.total_steps.load(Ordering::Relaxed));
                is_paused.set(h.paused.load(Ordering::Relaxed));
            }
        });
    });

    let ph = *phase.read();
    let elapsed = *session_elapsed.read();
    let total = *session_total.read();
    let s_elapsed = *step_elapsed.read();
    let s_total = *step_total.read();
    let step = *current_step.read();
    let steps = *total_steps.read();
    let paused = *is_paused.read();
    let disp = *display.read();
    let is_ladder = steps > 0;

    // nothing to show when idle
    if ph == Phase::Idle {
        return rsx! {};
    }

    // ── Main time value ───────────────────────────────────────────────────
    let main_ms = match disp {
        TimerDisplay::Countdown => total.saturating_sub(elapsed),
        TimerDisplay::Countup => elapsed,
    };
    let main_time = fmt_duration(main_ms);

    // ── Phase label ───────────────────────────────────────────────────────
    let phase_label = match ph {
        Phase::Resting => Some("rest"),
        Phase::CountIn => Some("count in"),
        Phase::Finished => Some("done"),
        Phase::Playing if paused => Some("paused"),
        _ => None,
    };

    rsx! {
        div { class: "timer",

            // ── Main timer ────────────────────────────────────────────────
            div {
                class: "timer-main",
                title: "click to toggle countdown / countup",
                onclick: move |_| {
                    display.set(disp.toggle());
                },
                "{main_time}"
            }

            // ── Phase label ───────────────────────────────────────────────
            if let Some(label) = phase_label {
                div { class: "timer-phase", "{label}" }
            }

            // ── Ladder step info ──────────────────────────────────────────
            if is_ladder {
                div { class: "timer-step-row",
                    span { class: "timer-step-count", "{step + 1} / {steps}" }
                    if ph == Phase::Playing {
                        span { class: "timer-step-remaining",
                            "{fmt_duration(s_total.saturating_sub(s_elapsed))}"
                        }
                    }
                }
            }
        }
    }
}
