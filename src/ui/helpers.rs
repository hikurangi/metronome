use crate::sound::beat::Beat;

#[derive(Clone, Copy)]
pub struct ActiveTick {
    pub idx: usize,
    pub beat: Beat,
    pub parity: bool,
}

pub fn flash_class(beat: Beat, parity: bool) -> &'static str {
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

pub fn subdivision_label(n: usize) -> String {
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

#[derive(Clone, Copy, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    System,
}

impl Theme {
    pub fn next(self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::System,
            Theme::System => Theme::Dark,
        }
    }
    pub fn app_class(self) -> &'static str {
        match self {
            Theme::Dark => "app dark",
            Theme::Light => "app light",
            Theme::System => "app",
        }
    }
    pub fn icon(self) -> &'static str {
        match self {
            Theme::Light => "☀",
            Theme::Dark => "☾",
            Theme::System => "◐",
        }
    }
}
