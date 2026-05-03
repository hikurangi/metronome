use crate::constants::BPM_DEFAULT;
use crate::sound::beat::Beat;

#[derive(Clone)]
pub struct AppContext {
    pub bpm: u64,
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
        self.sub_states = vec![Beat::Normal; n.saturating_sub(1)];
    }

    pub fn generate_pattern(&self) -> Vec<Beat> {
        self.beat_states
            .iter()
            .flat_map(|b| std::iter::once(b.clone()).chain(self.sub_states.iter().cloned()))
            .collect()
    }
}
