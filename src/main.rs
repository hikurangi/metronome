use rodio::OutputStream;
use rodio::buffer::SamplesBuffer;
use rodio::cpal::traits::{DeviceTrait, HostTrait};

mod constants;
mod engine;
mod sound;

use crate::constants::DEFAULT_SAMPLE_RATE;
use crate::engine::{EngineState, run};
use crate::sound::bank::SoundBank;
use crate::sound::beat::Beat;

// ── Config ────────────────────────────────────────────────────────────────────

const BPM: u64 = 120;

fn device_sample_rate() -> u32 {
    rodio::cpal::default_host()
        .default_output_device()
        .and_then(|d| d.default_output_config().ok())
        .map(|c| c.sample_rate().0)
        .unwrap_or(DEFAULT_SAMPLE_RATE)
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let sample_rate = device_sample_rate();

    let (_stream, stream_handle) = OutputStream::try_default().expect("no audio output");

    println!("♩ {BPM} BPM — with click sample rate of: {sample_rate} Hz");

    let pattern = vec![
        Beat::Accent,
        Beat::Normal,
        Beat::Normal,
        Beat::Normal,
        Beat::Silent,
    ];
    let mut state = EngineState::new(BPM, pattern);

    let bank = SoundBank::new(sample_rate);

    run(&mut state, |beat| {
        if let Some(buf) = bank.get(beat) {
            stream_handle
                .play_raw(SamplesBuffer::new(1, sample_rate, buf.to_vec()))
                .unwrap();
        }
    });
}
