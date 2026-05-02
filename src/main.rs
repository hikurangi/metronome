use std::f32::consts::TAU;
use std::time::{Duration, Instant};

use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Source};

// ── Config ────────────────────────────────────────────────────────────────────

const BPM: u64 = 120;
const SAMPLE_RATE: u32 = 44_100;
const CLICK_FREQ_HZ: f32 = 1320.0;
const CLICK_DECAY: f32 = 120.0; // higher = shorter transient
const CLICK_DURATION_MS: u32 = 60;

// ── Click synthesis ─────────────────────────────────────── future: src/audio/click.rs

struct ClickSource {
    total_samples: u32,
    current: u32,
}

impl ClickSource {
    fn new() -> Self {
        Self {
            total_samples: SAMPLE_RATE * CLICK_DURATION_MS / 1000,
            current: 0,
        }
    }
}

impl Iterator for ClickSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.current >= self.total_samples {
            return None;
        }
        let t = self.current as f32 / SAMPLE_RATE as f32;
        let sample = (TAU * CLICK_FREQ_HZ * t).sin() * (-CLICK_DECAY * t).exp();
        self.current += 1;
        Some(sample)
    }
}

impl Source for ClickSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_millis(CLICK_DURATION_MS as u64))
    }
}

// ── Engine ──────────────────────────────────────────────────── future: src/engine.rs
//
// `on_tick` will become an Output trait — audio, haptic, MIDI, WAV render, etc.

fn run(bpm: u64, on_tick: impl Fn()) {
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

// ── Pre-render click to buffer ──────────────────────────────────────────────

fn render_click() -> Vec<f32> {
    ClickSource::new().collect()
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().expect("no audio output");
    let click_buffer = render_click();

    println!("♩ {BPM} BPM");

    run(BPM, move || {
        let buf = SamplesBuffer::new(1, SAMPLE_RATE, click_buffer.clone());
        stream_handle.play_raw(buf).expect("play failed");
    });
}
