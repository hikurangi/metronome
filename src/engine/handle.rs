use crate::constants::BPM_DEFAULT;
use crate::sound::beat::Beat;
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicBool, AtomicU64},
};

pub struct EngineHandle {
    pub bpm: AtomicU64,
    pub running: AtomicBool,
    pub pattern: Arc<RwLock<Vec<Beat>>>,
}

impl EngineHandle {
    pub fn new(pattern: Arc<RwLock<Vec<Beat>>>) -> Self {
        Self {
            bpm: AtomicU64::new(BPM_DEFAULT),
            running: AtomicBool::new(false),
            pattern,
        }
    }
}
