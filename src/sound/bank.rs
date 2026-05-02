use crate::sound::click::{ClickConfig, render};

#[derive(Debug, Clone, PartialEq)]
pub enum Beat {
    Accent,
    Normal,
    Silent,
}

pub struct SoundBank {
    accent: Vec<f32>,
    normal: Vec<f32>,
}

impl SoundBank {
    pub fn new() -> Self {
        Self {
            normal: render(ClickConfig::normal()),
            accent: render(ClickConfig::accent()),
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
