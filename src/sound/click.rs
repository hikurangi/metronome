use rodio::Source;
use std::{f32::consts::TAU, time::Duration};

use crate::constants::SAMPLE_RATE_DEFAULT;

const CLICK_ACCENT_FREQ_HZ: f32 = 1320.0;
const CLICK_NORMAL_FREQ_HZ: f32 = 880.0;
const CLICK_DECAY: f32 = 120.0; // higher = shorter transient
const CLICK_DURATION_MS: u32 = 60;

pub struct ClickConfig {
    pub freq_hz: f32,
    pub decay: f32,
    pub sample_rate: u32,
    pub duration_ms: u32,
}

impl Default for ClickConfig {
    fn default() -> Self {
        Self {
            freq_hz: CLICK_NORMAL_FREQ_HZ,
            decay: CLICK_DECAY,
            duration_ms: CLICK_DURATION_MS,
            sample_rate: SAMPLE_RATE_DEFAULT,
        }
    }
}

impl ClickConfig {
    pub fn normal(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            ..Self::default()
        }
    }
    pub fn accent(sample_rate: u32) -> Self {
        Self {
            freq_hz: CLICK_ACCENT_FREQ_HZ,
            sample_rate,
            ..Self::default()
        }
    }
}

// Runtime state — derived from config + iteration cursor
pub struct ClickSource {
    config: ClickConfig,
    total_samples: u32,
    current: u32,
}

impl From<ClickConfig> for ClickSource {
    fn from(config: ClickConfig) -> Self {
        let total_samples = config.sample_rate * config.duration_ms / 1000;
        let current: u32 = 0;
        Self {
            config,
            total_samples,
            current,
        }
    }
}

impl Iterator for ClickSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.current >= self.total_samples {
            return None;
        }
        let t = self.current as f32 / self.config.sample_rate as f32;
        let sample = (TAU * self.config.freq_hz * t).sin() * (-self.config.decay * t).exp();
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
        self.config.sample_rate
    }
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_millis(self.config.duration_ms as u64))
    }
}

// ── Pre-render click to buffer ──────────────────────────────────────────────

pub fn render(config: ClickConfig) -> Vec<f32> {
    ClickSource::from(config).collect()
}
