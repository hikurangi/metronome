use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Infinity,
    Block,
    Ladder,
}
impl Mode {
    pub fn next(self) -> Self {
        match self {
            Mode::Infinity => Mode::Block,
            Mode::Block => Mode::Ladder,
            Mode::Ladder => Mode::Infinity,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Mode::Infinity => Mode::Ladder,
            Mode::Block => Mode::Infinity,
            Mode::Ladder => Mode::Block,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Mode::Infinity => "∞",
            Mode::Block => "▭",
            Mode::Ladder => "↑",
        }
    }
}

#[derive(Clone)]
pub struct BlockConfig {
    pub bpm: u64,
    pub duration: Duration,
}
impl Default for BlockConfig {
    fn default() -> Self {
        Self {
            bpm: 120,
            duration: Duration::from_secs(60),
        }
    }
}

#[derive(Clone)]
pub struct LadderConfig {
    pub start_bpm: u64,
    pub step_duration: Duration,
    pub rest_duration: Duration,
    pub tempo_increment: i64, // signed — can go down
    pub cycle_count: usize,
}
impl Default for LadderConfig {
    fn default() -> Self {
        Self {
            start_bpm: 80,
            step_duration: Duration::from_secs(30),
            rest_duration: Duration::ZERO,
            tempo_increment: 5,
            cycle_count: 8,
        }
    }
}
