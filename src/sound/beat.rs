#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Beat {
    Accent,
    Normal,
    SubAccent,
    SubNormal,
    Silent,
}

impl Beat {
    pub fn cycle_primary(self) -> Self {
        match self {
            Beat::Silent => Beat::Normal,
            Beat::Normal => Beat::Accent,
            Beat::Accent => Beat::Silent,
            sub => sub,
        }
    }

    pub fn cycle_sub(self) -> Self {
        match self {
            Beat::Silent | Beat::Normal => Beat::SubNormal,
            Beat::SubNormal => Beat::SubAccent,
            Beat::SubAccent => Beat::Silent,
            primary => primary,
        }
    }

    pub fn flash_intensity(&self) -> f32 {
        match self {
            Beat::Accent => 1.0,
            Beat::Normal => 0.6,
            Beat::SubAccent => 0.45,
            Beat::SubNormal => 0.3,
            Beat::Silent => 0.0,
        }
    }
}

impl From<Beat> for u8 {
    fn from(b: Beat) -> u8 {
        match b {
            Beat::Accent => 4,
            Beat::Normal => 3,
            Beat::SubAccent => 2,
            Beat::SubNormal => 1,
            Beat::Silent => 0,
        }
    }
}

impl From<u8> for Beat {
    fn from(v: u8) -> Self {
        match v {
            4 => Beat::Accent,
            3 => Beat::Normal,
            2 => Beat::SubAccent,
            1 => Beat::SubNormal,
            _ => Beat::Silent,
        }
    }
}
