use crate::{engine::handle::EngineHandle, sound::beat::Beat};
use std::{
    sync::{Arc, atomic::Ordering},
    time::{Duration, Instant},
};

pub struct EngineState {
    pub beat_states: Vec<Beat>,
    pub sub_states: Vec<Beat>,
    pub beat_index: usize,
}

impl EngineState {
    pub fn new(beat_states: Vec<Beat>, sub_states: Vec<Beat>) -> Self {
        Self {
            beat_states,
            sub_states,
            beat_index: 0,
        }
    }

    fn current_beat(&self) -> Beat {
        if self.beat_states.is_empty() {
            return Beat::Silent;
        }
        self.beat_states[self.beat_index % self.beat_states.len()]
    }

    fn current_sub(&self, sub_index: usize) -> Beat {
        if self.sub_states.is_empty() {
            return Beat::Silent;
        }
        self.sub_states[sub_index % self.sub_states.len()]
    }

    fn advance_beat(&mut self) {
        if !self.beat_states.is_empty() {
            self.beat_index = (self.beat_index + 1) % self.beat_states.len();
        }
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
    let mut was_running = false;
    let mut last_downbeat = Instant::now();
    let mut next_downbeat = Instant::now();
    let mut next_sub = Instant::now();
    let mut sub_index = 0usize; // position within current beat's subs
    let mut last_bpm = handle.bpm.load(Ordering::Relaxed);

    loop {
        let is_running = handle.running.load(Ordering::Relaxed);

        if !is_running {
            if was_running {
                state.reset();
                on_stop();
            }
            was_running = false;
            sub_index = 0;
            last_downbeat = Instant::now();
            next_downbeat = Instant::now();
            next_sub = Instant::now();
            last_bpm = handle.bpm.load(Ordering::Relaxed);
            std::thread::sleep(Duration::from_millis(10));
            continue;
        }

        was_running = true;

        let bpm = handle.bpm.load(Ordering::Relaxed);
        let subs = handle.subdivisions.load(Ordering::Relaxed).max(1);
        let beat_nanos = 60_000_000_000u64 / bpm;
        let beat_interval = Duration::from_nanos(beat_nanos);
        let sub_interval = Duration::from_nanos(beat_nanos / subs as u64);

        // BPM changed — anchor next downbeat from last, reschedule subs
        if bpm != last_bpm {
            next_downbeat = last_downbeat + beat_interval;
            if subs > 1 {
                next_sub = last_downbeat + sub_interval * (sub_index as u32 + 1);
            }
            last_bpm = bpm;
        }

        // apply pending beat/sub state updates from UI
        {
            let mut bp = handle.beat_states_pending.write().unwrap();
            if let Some(new_beats) = bp.take() {
                state.beat_states = new_beats;
            }
        }
        // {
        //     let mut sp = handle.sub_states_pending.write().unwrap();
        //     if let Some(new_subs) = sp.take() {
        //         state.sub_states = new_subs;
        //         // reschedule subs from last downbeat, downbeat untouched
        //         sub_index = 0;
        //         next_sub = last_downbeat + sub_interval;
        //     }
        // }

        let now = Instant::now();

        let next_event = if subs > 1 && sub_index < subs - 1 {
            next_downbeat.min(next_sub)
        } else {
            next_downbeat
        };

        if now < next_event {
            std::thread::sleep(Duration::from_millis(8).min(next_event - now));
            continue;
        }

        // downbeat takes priority if both arrive simultaneously
        if now >= next_downbeat {
            let beat_idx = state.beat_index; // capture BEFORE advancing
            let beat = state.current_beat();
            state.advance_beat();

            last_downbeat = next_downbeat;
            next_downbeat += beat_interval;
            sub_index = 0;

            // apply pending sub changes at the clean beat boundary
            let mut sp = handle.sub_states_pending.write().unwrap();
            if let Some(new_subs) = sp.take() {
                state.sub_states = new_subs;
            }

            if subs > 1 {
                next_sub = last_downbeat + sub_interval;
            }

            on_tick(beat);
            handle.current_beat_idx.store(beat_idx, Ordering::Relaxed);
            handle
                .current_beat_type
                .store(u8::from(beat), Ordering::Relaxed);
            handle.beat_tick_count.fetch_add(1, Ordering::Relaxed);
            handle.tick_count.fetch_add(1, Ordering::Relaxed);
        } else if subs > 1 && sub_index < subs - 1 && now >= next_sub {
            let sub = state.current_sub(sub_index);
            handle.current_sub_idx.store(sub_index, Ordering::Relaxed);

            sub_index += 1;
            next_sub += sub_interval;

            on_tick(sub);
            handle
                .current_beat_type
                .store(u8::from(sub), Ordering::Relaxed);
            handle.sub_tick_count.fetch_add(1, Ordering::Relaxed);
            handle.tick_count.fetch_add(1, Ordering::Relaxed);
        }
    }
}
