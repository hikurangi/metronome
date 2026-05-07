use dioxus::prelude::*;

use crate::ui::helpers::Theme;

#[component]
pub fn UIThemeToggle() -> Element {
    let mut theme = use_context::<Signal<Theme>>();

    rsx! {
        button {
            class: "theme-btn",
            onclick: move |_| {
                let t = theme.read().next();
                theme.set(t);
            },
            "{theme.read().icon()}"
        }
    }
}
