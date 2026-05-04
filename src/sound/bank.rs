use crate::sound::{
    beat::Beat,
    click::{ClickConfig, render},
};

pub struct SoundBank {
    accent: Vec<f32>,
    normal: Vec<f32>,
    sub_accent: Vec<f32>,
    sub_normal: Vec<f32>,
}

impl SoundBank {
    // not super convinced about this method
    // of passing a sample rate down at runtime
    pub fn new(sample_rate: u32) -> Self {
        Self {
            normal: render(ClickConfig::normal(sample_rate)),
            accent: render(ClickConfig::accent(sample_rate)),
            sub_accent: render(ClickConfig::sub_accent(sample_rate)),
            sub_normal: render(ClickConfig::sub_normal(sample_rate)),
        }
    }

    pub fn get(&self, beat: &Beat) -> Option<&[f32]> {
        match beat {
            Beat::Accent => Some(&self.accent),
            Beat::Normal => Some(&self.normal),
            Beat::SubAccent => Some(&self.sub_accent),
            Beat::SubNormal => Some(&self.sub_normal),
            Beat::Silent => None,
        }
    }
}
