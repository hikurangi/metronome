use crate::{engine::handle::EngineHandle, sound::beat::Beat};
use std::{
    sync::{Arc, RwLock, atomic::Ordering},
    time::{Duration, Instant},
};

pub struct EngineState {
    pattern: Arc<RwLock<Vec<Beat>>>,
    beat_index: usize,
}

impl EngineState {
    pub fn new(pattern: Arc<RwLock<Vec<Beat>>>) -> Self {
        Self {
            pattern,
            beat_index: 0,
        }
    }

    // returns owned Beat to avoid holding the RwLock across the tick
    fn advance(&mut self) -> Beat {
        let pattern = self.pattern.read().unwrap();
        if pattern.is_empty() {
            return Beat::Silent;
        }
        let beat = pattern[self.beat_index % pattern.len()].clone();
        self.beat_index = (self.beat_index + 1) % pattern.len();
        beat
    }

    pub fn reset(&mut self) {
        self.beat_index = 0;
    }
}

pub fn run(
    state: &mut EngineState,
    handle: Arc<EngineHandle>,
    on_tick: impl Fn(Beat),
    on_stop: impl Fn(),
) {
    let mut next_tick = Instant::now();
    let mut was_running = false;

    loop {
        let is_running = handle.running.load(Ordering::Relaxed);

        if !is_running {
            if was_running {
                state.reset();
                on_stop();
            }
            was_running = false;
            std::thread::sleep(Duration::from_millis(10));
            next_tick = Instant::now();
            continue;
        }

        let bpm = handle.bpm.load(Ordering::Relaxed);
        let interval = Duration::from_nanos(60_000_000_000 / bpm);

        was_running = true;
        let now = Instant::now();
        if now < next_tick {
            spin_sleep::sleep(next_tick - now);
        }
        on_tick(state.advance());
        next_tick += interval;
    }
}
