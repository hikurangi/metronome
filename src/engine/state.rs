// takes Arc<AppContext>, interval recomputed each tick
use crate::engine::handle::EngineHandle;
use crate::sound::beat::Beat;
use std::sync::{Arc, atomic::Ordering};
use std::time::{Duration, Instant};

pub struct EngineState {
    pub pattern: Vec<Beat>,
    beat_index: usize,
}

impl EngineState {
    pub fn new(pattern: Vec<Beat>) -> Self {
        Self {
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

pub fn run(
    state: &mut EngineState,
    ctx: Arc<EngineHandle>,
    on_tick: impl Fn(&Beat),
    on_stop: impl Fn(),
) {
    let mut next_tick = Instant::now();
    let mut was_running = false;

    loop {
        let is_running = ctx.is_running.load(Ordering::Relaxed);

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

        // recompute each tick — in-flight BPM changes take effect here
        let bpm = ctx.bpm.load(Ordering::Relaxed);
        let interval = Duration::from_nanos(60_000_000_000 / bpm);

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
