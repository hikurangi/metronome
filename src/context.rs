use crate::constants::BPM_DEFAULT;
use crate::session::config::{BlockConfig, LadderConfig, Mode};
use crate::sound::beat::Beat;

#[derive(Clone)]
pub struct AppContext {
    pub bpm: u64,
    pub mode: Mode,
    pub block_config: BlockConfig,   // persisted
    pub ladder_config: LadderConfig, // persisted
    pub is_running: bool,
    pub beats_per_bar: usize,
    pub subdivisions: usize,
    pub beat_states: Vec<Beat>,
    pub sub_states: Vec<Beat>, // subdivisions - 1 entries, shared template across all beats
}

impl AppContext {
    pub fn new() -> Self {
        let beats_per_bar = 4;
        Self {
            bpm: BPM_DEFAULT,
            is_running: false,
            beats_per_bar,
            subdivisions: 1,
            beat_states: Self::default_beat_states(beats_per_bar),
            sub_states: vec![],
            mode: Mode::Infinity,
            block_config: BlockConfig::default(),
            ladder_config: LadderConfig::default(),
        }
    }

    fn default_beat_states(n: usize) -> Vec<Beat> {
        (0..n)
            .map(|i| if i == 0 { Beat::Accent } else { Beat::Normal })
            .collect()
    }

    pub fn set_beats_per_bar(&mut self, n: usize) {
        self.beats_per_bar = n;
        self.beat_states.resize_with(n, || Beat::Normal);
        if !self.beat_states.is_empty() {
            self.beat_states[0] = Beat::Accent;
        }
    }

    pub fn set_subdivisions(&mut self, n: usize) {
        self.subdivisions = n;
        let target = n.saturating_sub(1);
        match target.cmp(&self.sub_states.len()) {
            std::cmp::Ordering::Greater => {
                let extra = target - self.sub_states.len();
                self.sub_states.extend(vec![Beat::SubNormal; extra]);
            }
            std::cmp::Ordering::Less => {
                self.sub_states.truncate(target);
            }
            std::cmp::Ordering::Equal => {}
        }
    }
}
