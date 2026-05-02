use rodio::Source;
use std::{f32::consts::TAU, time::Duration};

use crate::{CLICK_DECAY, CLICK_DURATION_MS, CLICK_FREQ_HZ, DEFAULT_SAMPLE_RATE};

pub struct ClickSourceConfig {
    pub sample_rate: u32,
    pub duration_ms: u32,
}

impl Default for ClickSource {
    fn default() -> Self {
        Self {
            current: 0,
            sample_rate: DEFAULT_SAMPLE_RATE,
            total_samples: DEFAULT_SAMPLE_RATE + CLICK_DURATION_MS / 1000,
            freq_hz: CLICK_FREQ_HZ,
            decay: CLICK_DECAY,
            duration_ms: CLICK_DURATION_MS,
        }
    }
}

pub struct ClickSource {
    freq_hz: f32,
    decay: f32,
    sample_rate: u32,
    total_samples: u32,
    current: u32,
    duration_ms: u32,
}

impl ClickSource {
    pub fn new(config: ClickSourceConfig) -> Self {
        Self {
            total_samples: config.sample_rate * config.duration_ms / 1000,
            current: 0,
            ..ClickSource::default()
        }
    }
}

impl Iterator for ClickSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.current >= self.total_samples {
            return None;
        }
        let t = self.current as f32 / self.sample_rate as f32;
        let sample = (TAU * self.freq_hz * t).sin() * (-self.decay * t).exp();
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
        self.sample_rate
    }
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_millis(self.duration_ms as u64))
    }
}
