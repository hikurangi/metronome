use std::sync::Arc;

use dioxus::prelude::*;

use crate::{context::AppContext, engine::handle::EngineHandle};

// TODO: make generic - just pass its kind as props
#[component]
pub fn BeatDecrementButton() -> Element {
    let engine = use_context::<Arc<EngineHandle>>();
    let mut ctx = use_context::<Signal<AppContext>>();

    // NOTE: this component's styling depends on being placed inside div.beat-area - we should consider modularising it
    rsx! {
        button {
            class: "adj",
            onclick: move |_| {
                let mut c = ctx.write();
                if c.beats_per_bar > 1 {
                    let n = c.beats_per_bar - 1;
                    c.set_beats_per_bar(n);
                    *engine.beat_states_pending.write().unwrap() = Some(c.beat_states.clone());
                }
            },
            "−"
        }
    }
}

#[component]
pub fn BeatIncrementButton() -> Element {
    let engine = use_context::<Arc<EngineHandle>>();
    let mut ctx = use_context::<Signal<AppContext>>();

    // NOTE: as mentioned above, this component's styling depends on being placed inside its parent
    rsx! {
        button {
            class: "adj",
            onclick: move |_| {
                let mut c = ctx.write();
                let n = c.beats_per_bar + 1;
                c.set_beats_per_bar(n);
                *engine.beat_states_pending.write().unwrap() = Some(c.beat_states.clone());
            },
            "+"
        }
    }
}
