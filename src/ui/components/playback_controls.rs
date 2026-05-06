use std::sync::{Arc, atomic::Ordering};

use dioxus::prelude::*;

use crate::{
    context::AppContext,
    engine::handle::EngineHandle,
    session::{
        config::Mode,
        handle::{Cmd, SessionHandle},
    },
};

#[component]
pub fn PlaybackControls() -> Element {
    let mut ctx: Signal<AppContext> = use_context::<Signal<AppContext>>();
    let is_running = ctx.read().is_running;
    let mode = ctx.read().mode;

    let engine = use_context::<Arc<EngineHandle>>();
    let h_running = Arc::clone(&engine);

    let session = use_context::<Arc<SessionHandle>>();
    let s_running = Arc::clone(&session);
    let is_paused = session.paused.load(Ordering::Relaxed);

    rsx! {
        if mode == Mode::Infinity {
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
        } else {
            // Block / Ladder: Pause + Stop
            div { class: "play-controls",
                button {
                    class: "play pause",
                    onclick: move |_| {
                        let cmd = if is_paused { Cmd::Resume as u8 } else { Cmd::Pause as u8 };
                        s_running.cmd.store(cmd, Ordering::Relaxed);
                    },
                    if is_paused {
                        "▶  RESUME"
                    } else {
                        "⏸  PAUSE"
                    }
                }
                button {
                    class: "play stop",
                    onclick: move |_| {
                        session.cmd.store(Cmd::Stop as u8, Ordering::Relaxed);
                    },
                    "■  STOP"
                }
            }
        }

    }
}
