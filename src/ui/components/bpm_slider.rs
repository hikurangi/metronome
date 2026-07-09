use std::sync::{Arc, atomic::Ordering};

use dioxus::prelude::*;

use crate::{
    constants::{BPM_MAX, BPM_MIN},
    context::AppContext,
    engine::handle::EngineHandle,
};

#[component]
pub fn BPMSlider() -> Element {
    let mut ctx = use_context::<Signal<AppContext>>();
    let engine = use_context::<Arc<EngineHandle>>();

    let h_bpm_dec = Arc::clone(&engine);
    let h_bpm_inc = Arc::clone(&engine);
    let h_bpm_slider = Arc::clone(&engine);

    rsx! {
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
                value: ctx.read().bpm,
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
    }
}
