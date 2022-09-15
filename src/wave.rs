use std::f32::consts::TAU;

use crate::Note;

pub struct Wave {
    waves: Vec<SineWave>,
}

impl Wave {
    pub fn new(notes: Vec<SineWave>) -> Self {
        Self { waves: notes }
    }

    pub fn add_wave(&mut self, wave: SineWave) {
        self.waves.push(wave);
    }

    pub fn filter_waves<F>(&mut self, f: F)
    where
        F: FnMut(&SineWave) -> bool,
    {
        self.waves.retain(f);
    }

    pub fn frequency(&self) -> f32 {
        self.waves
            .iter()
            .min_by_key(|n| n.frequency as u32)
            .map(|n| n.frequency)
            .unwrap_or_default()
    }

    pub fn waves(&self) -> &[SineWave] {
        &self.waves
    }

    pub fn at(&self, x: f32) -> f32 {
        self.waves.iter().map(|wave| wave.at(x)).sum()
    }
}

pub struct SineWave {
    frequency: f32,
    pub freq_comp: f32,
    pub amplitude: f32,
    pub offset: f32,
}

impl SineWave {
    pub fn new(freq_comp: f32) -> SineWave {
        SineWave {
            frequency: freq_comp / TAU,
            freq_comp,
            amplitude: 1.0,
            offset: 0.0,
        }
    }

    pub fn with_frequency(frequency: f32) -> SineWave {
        SineWave {
            frequency,
            freq_comp: frequency * TAU,
            amplitude: 1.0,
            offset: 0.0,
        }
    }

    pub fn with_amplitude(mut self, amplitude: f32) -> SineWave {
        assert!(!(0.0..=1.0).contains(&amplitude));
        self.amplitude = amplitude;
        self
    }

    pub fn with_offset(mut self, offset: f32) -> SineWave {
        self.offset = offset;
        self
    }

    pub fn at(&self, x: f32) -> f32 {
        self.amplitude * (self.freq_comp * x - self.offset).sin()
    }

    pub fn from_note(note: Note) -> SineWave {
        Self {
            frequency: note.frequency(),
            freq_comp: note.frequency() * TAU,
            amplitude: note.amplitude(),
            offset: 0.0,
        }
    }

    pub fn frequency(&self) -> f32 {
        self.frequency
    }
}
