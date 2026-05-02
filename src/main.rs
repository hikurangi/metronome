use std::time::{Duration, Instant};

use rodio::OutputStream;
use rodio::buffer::SamplesBuffer;
use rodio::cpal::traits::{DeviceTrait, HostTrait};

use crate::sound::click::{ClickSource, ClickSourceConfig};

mod sound;
// use audio::click::ClickSource;
// ── Config ────────────────────────────────────────────────────────────────────

const BPM: u64 = 120;
const DEFAULT_SAMPLE_RATE: u32 = 44_100;
const CLICK_FREQ_HZ: f32 = 1320.0;
const CLICK_DECAY: f32 = 120.0; // higher = shorter transient
const CLICK_DURATION_MS: u32 = 60;

fn device_sample_rate() -> u32 {
    rodio::cpal::default_host()
        .default_output_device()
        .and_then(|d| d.default_output_config().ok())
        .map(|c| c.sample_rate().0)
        .unwrap_or(DEFAULT_SAMPLE_RATE)
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

fn render_click(config: ClickSourceConfig) -> Vec<f32> {
    ClickSource::new(config).collect()
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let sample_rate = device_sample_rate();
    let click_source_config = ClickSourceConfig {
        sample_rate,
        duration_ms: CLICK_DURATION_MS,
    };

    let (_stream, stream_handle) = OutputStream::try_default().expect("no audio output");
    let click_buffer = render_click(click_source_config);

    println!("♩ {BPM} BPM — with click sample rate of: {sample_rate} Hz");

    run(BPM, move || {
        let buf = SamplesBuffer::new(1, sample_rate, click_buffer.clone());
        stream_handle.play_raw(buf).expect("play failed");
    });
}
