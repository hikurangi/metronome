use std::time::{Duration, Instant};

use crate::sound::bank::Beat;

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
}

// `on_tick` will become an Output trait — audio, haptic, MIDI, WAV render, etc.
pub fn run(state: &mut EngineState, on_tick: impl Fn(&Beat)) {
    let interval = Duration::from_nanos(60_000_000_000 / state.bpm);
    let mut next_tick = Instant::now();

    loop {
        let now = Instant::now();
        if now < next_tick {
            spin_sleep::sleep(next_tick - now);
        }
        let beat = state.advance();
        on_tick(beat);
        next_tick += interval;
    }
}
