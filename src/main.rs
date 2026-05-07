use rodio::buffer::SamplesBuffer;
use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::{OutputStream, Sink};
use std::sync::{Arc, atomic::Ordering};

mod constants;
mod context;
mod engine;
mod session;
mod sound;
mod ui;

use crate::constants::SAMPLE_RATE_DEFAULT;
use crate::context::AppContext;
use crate::engine::engine::{EngineState, run};
use crate::engine::handle::EngineHandle;
use crate::session::handle::SessionHandle;
use crate::sound::bank::SoundBank;
use crate::ui::App;

fn device_sample_rate() -> u32 {
    rodio::cpal::default_host()
        .default_output_device()
        .and_then(|d| d.default_output_config().ok())
        .map(|c| c.sample_rate().0)
        .unwrap_or(SAMPLE_RATE_DEFAULT)
}

fn main() {
    let sample_rate = device_sample_rate();
    let (_stream, stream_handle) = OutputStream::try_default().expect("no audio output");

    let app_ctx = AppContext::new();

    let mut state = EngineState::new(app_ctx.beat_states.clone(), app_ctx.sub_states.clone());

    let bank = SoundBank::new(sample_rate);
    let engine = Arc::new(EngineHandle::new());
    let session = Arc::new(SessionHandle::new());

    engine
        .subdivisions
        .store(app_ctx.subdivisions, Ordering::Relaxed);

    let sink = Arc::new(Sink::try_new(&stream_handle).unwrap());
    let sink_tick = Arc::clone(&sink);
    let sink_stop = Arc::clone(&sink);

    let engine_thread = Arc::clone(&engine);
    std::thread::spawn(move || {
        run(
            &mut state,
            engine_thread,
            |beat| {
                if let Some(buf) = bank.get(&beat) {
                    sink_tick.append(SamplesBuffer::new(1, sample_rate, buf.to_vec()));
                    sink_tick.play();
                }
            },
            || sink_stop.clear(),
        );
    });

    let session_controller = Arc::clone(&session);
    let engine_for_session = Arc::clone(&engine);
    std::thread::spawn(move || {
        session::controller::run(session_controller, engine_for_session);
    });

    dioxus::LaunchBuilder::new()
        .with_context(engine)
        .with_context(session)
        .launch(App);
}
