use crate::sound::{
    beat::Beat,
    click::{ClickConfig, render},
};

pub struct SoundBank {
    accent: Vec<f32>,
    normal: Vec<f32>,
}

impl SoundBank {
    // not super convinced about this method
    // of passing a sample rate down at runtime
    pub fn new(sample_rate: u32) -> Self {
        Self {
            normal: render(ClickConfig::normal(sample_rate)),
            accent: render(ClickConfig::accent(sample_rate)),
        }
    }

    pub fn get(&self, beat: &Beat) -> Option<&[f32]> {
        match beat {
            Beat::Accent => Some(&self.accent),
            Beat::Normal => Some(&self.normal),
            Beat::Silent => None,
        }
    }
}
