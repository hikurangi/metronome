use crate::context::BPM_DEFAULT;
use std::sync::atomic::{AtomicBool, AtomicU64};

pub struct EngineHandle {
    pub bpm: AtomicU64,
    pub is_running: AtomicBool,
}

impl EngineHandle {
    pub fn new() -> Self {
        Self {
            bpm: AtomicU64::new(BPM_DEFAULT),
            is_running: AtomicBool::new(false),
        }
    }
}
