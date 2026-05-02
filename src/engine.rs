// `on_tick` will become an Output trait — audio, haptic, MIDI, WAV render, etc.

use std::time::{Duration, Instant};

pub fn run(bpm: u64, on_tick: impl Fn()) {
    let interval = Duration::from_nanos(60_000_000_000 / bpm);
    let mut next_tick = Instant::now();

    loop {
        let now = Instant::now();
        if now < next_tick {
            spin_sleep::sleep(next_tick - now);
        }
        on_tick();
        next_tick += interval; // fixed advance — never drifts
    }
}
