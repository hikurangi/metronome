pub const BPM_MIN: u64 = 1;
pub const BPM_MAX: u64 = 420;
pub const BPM_DEFAULT: u64 = 120;

#[derive(Clone)]
pub struct AppContext {
    pub bpm: u64,
    pub is_running: bool,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            bpm: BPM_DEFAULT,
            is_running: false,
        }
    }
}
