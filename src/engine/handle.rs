use crate::constants::BPM_DEFAULT;
use crate::sound::beat::Beat;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, AtomicUsize};
use std::sync::{Arc, RwLock};

pub struct EngineHandle {
    pub bpm: AtomicU64,
    pub running: AtomicBool,
    pub subdivisions: AtomicUsize,
    pub tick_count: AtomicU64,
    pub current_beat_idx: AtomicUsize,
    pub current_beat_type: AtomicU8,
    pub beat_states_pending: Arc<RwLock<Option<Vec<Beat>>>>,
    pub sub_states_pending: Arc<RwLock<Option<Vec<Beat>>>>,
}

impl EngineHandle {
    pub fn new() -> Self {
        Self {
            bpm: AtomicU64::new(BPM_DEFAULT),
            running: AtomicBool::new(false),
            subdivisions: AtomicUsize::new(1),
            tick_count: AtomicU64::new(0),
            current_beat_idx: AtomicUsize::new(0),
            current_beat_type: AtomicU8::new(0),
            beat_states_pending: Arc::new(RwLock::new(None)),
            sub_states_pending: Arc::new(RwLock::new(None)),
        }
    }
}
