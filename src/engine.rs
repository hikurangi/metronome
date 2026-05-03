use crate::sound::beat::Beat;
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

pub struct EngineState {
    pub bpm: u64,
    pub pattern: Vec<Beat>,
    beat_index: usize,
}

impl EngineState {
    pub fn new(bpm: u64, pattern: Vec<Beat>) -> Self {
        Self {
            bpm,
            pattern,
            beat_index: 0,
        }
    }

    fn advance(&mut self) -> &Beat {
        let beat = &self.pattern[self.beat_index];
        self.beat_index = (self.beat_index + 1) % self.pattern.len();
        beat
    }

    pub fn reset(&mut self) {
        self.beat_index = 0;
    }
}

// `on_tick` will become an Output trait — audio, haptic, MIDI, WAV render, etc.
pub fn run(
    state: &mut EngineState,
    running: Arc<AtomicBool>,
    on_tick: impl Fn(&Beat),
    on_stop: impl Fn(),
) {
    let interval = Duration::from_nanos(60_000_000_000 / state.bpm);
    let mut next_tick = Instant::now();
    let mut was_running = false;

    loop {
        let is_running = running.load(Ordering::Relaxed);

        if !is_running {
            if was_running {
                state.reset(); // back to top of pattern
                on_stop();
            }
            was_running = false;
            std::thread::sleep(Duration::from_millis(10));
            next_tick = Instant::now();
            continue;
        }

        was_running = true;
        let now = Instant::now();
        if now < next_tick {
            spin_sleep::sleep(next_tick - now);
        }
        let beat = state.advance();
        on_tick(beat);
        next_tick += interval;
    }
}
