use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, AtomicUsize};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    Idle = 0,
    Playing = 1,
    Resting = 2,
    CountIn = 3,
    Finished = 4,
}

impl From<u8> for Phase {
    fn from(v: u8) -> Self {
        match v {
            1 => Phase::Playing,
            2 => Phase::Resting,
            3 => Phase::CountIn,
            4 => Phase::Finished,
            _ => Phase::Idle,
        }
    }
}

#[repr(u8)]
pub enum Cmd {
    None = 0,
    Start = 1,
    Pause = 2,
    Resume = 3,
    Stop = 4,
}

pub struct SessionHandle {
    // state → UI
    pub phase: AtomicU8,
    pub session_elapsed: AtomicU64, // ms
    pub session_total: AtomicU64,   // ms
    pub step_elapsed: AtomicU64,    // ms (Ladder)
    pub step_total: AtomicU64,      // ms (Ladder)
    pub current_step: AtomicUsize,
    pub total_steps: AtomicUsize,
    pub paused: AtomicBool,
    // commands → controller
    pub cmd: AtomicU8,
}

impl SessionHandle {
    pub fn new() -> Self {
        Self {
            phase: AtomicU8::new(Phase::Idle as u8),
            session_elapsed: AtomicU64::new(0),
            session_total: AtomicU64::new(0),
            step_elapsed: AtomicU64::new(0),
            step_total: AtomicU64::new(0),
            current_step: AtomicUsize::new(0),
            total_steps: AtomicUsize::new(0),
            paused: AtomicBool::new(false),
            cmd: AtomicU8::new(Cmd::None as u8),
        }
    }
}
