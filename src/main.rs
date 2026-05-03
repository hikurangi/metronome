use dioxus::signals::{Signal, SyncStorage};
use rodio::buffer::SamplesBuffer;
use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::{OutputStream, Sink};
use std::sync::Arc;

mod constants;
mod context;
mod engine;
mod sound;
mod ui;

use crate::constants::DEFAULT_SAMPLE_RATE;
use crate::context::AppContext;
use crate::engine::handle::EngineHandle;
use crate::engine::state::{EngineState, run};
use crate::sound::bank::SoundBank;
use crate::sound::beat::Beat;
use crate::ui::App;

fn device_sample_rate() -> u32 {
    rodio::cpal::default_host()
        .default_output_device()
        .and_then(|d| d.default_output_config().ok())
        .map(|c| c.sample_rate().0)
        .unwrap_or(DEFAULT_SAMPLE_RATE)
}

fn main() {
    let sample_rate = device_sample_rate();
    let (_stream, stream_handle) = OutputStream::try_default().expect("no audio output");

    let pattern = vec![Beat::Accent, Beat::Normal, Beat::Normal, Beat::Normal];

    let mut state = EngineState::new(pattern);
    let bank = SoundBank::new(sample_rate);

    let handle = Arc::new(EngineHandle::new());
    let handle_engine = Arc::clone(&handle);

    let sink = Arc::new(Sink::try_new(&stream_handle).unwrap());
    let sink_tick = Arc::clone(&sink);
    let sink_stop = Arc::clone(&sink);

    std::thread::spawn(move || {
        run(
            &mut state,
            handle_engine,
            |beat| {
                if let Some(buf) = bank.get(beat) {
                    sink_tick.append(SamplesBuffer::new(1, sample_rate, buf.to_vec()));
                    sink_tick.play();
                }
            },
            || sink_stop.clear(),
        );
    });

    dioxus::LaunchBuilder::new()
        .with_context(handle)
        .launch(App);
}
