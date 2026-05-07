mod components;
mod helpers;

use crate::context::AppContext;
use crate::session::config::{Mode, SessionStatus};
use crate::session::handle::{Phase, SessionHandle};

use crate::ui::components::config::block::BlockConfigPanel;
use crate::ui::components::config::ladder::LadderConfigPanel;
use crate::ui::components::metronome::Metronome;
use crate::ui::components::mode_slider::ModeSlider;
use crate::ui::components::ui_theme_toggle::UIThemeToggle;
use crate::ui::helpers::Theme;
use dioxus::prelude::*;

use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;

#[component]
pub fn App() -> Element {
    let ctx = use_context_provider(|| Signal::new(AppContext::new()));

    // NOTE: possibly provide the following as context at the dioxus instatiation level in main.rs
    let theme = use_context_provider(|| Signal::new(Theme::Light));
    let session = use_context::<Arc<SessionHandle>>();

    let mut session_status = use_context_provider(|| Signal::new(SessionStatus::Inactive));

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

    const STYLE: Asset = asset!("/assets/main.css");

    rsx! {

        document::Stylesheet { href: STYLE }

        div { class: theme.read().app_class(),

            UIThemeToggle {}

            match ctx.read().mode {
                Mode::Infinity => rsx! {
                    Metronome {}
                },
                Mode::Block => {
                    match *session_status.read() {
                        SessionStatus::Active => rsx! {
                            Metronome {}
                        },
                        SessionStatus::Inactive => rsx! {
                            BlockConfigPanel {}
                        },
                    }
                }
                Mode::Ladder => {
                    match *session_status.read() {
                        SessionStatus::Active => rsx! {
                            Metronome {}
                        },
                        SessionStatus::Inactive => rsx! {
                            LadderConfigPanel {}
                        },
                    }
                }
            }

            ModeSlider {}

        }
    }
}
