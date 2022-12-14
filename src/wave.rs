use std::{f32::consts::TAU, fmt::Debug};

use crate::Note;

#[derive(Clone, PartialEq)]
pub struct Wave {
    waves: Vec<SineWave>,
}

impl Wave {
    pub fn new(waves: Vec<SineWave>) -> Self {
        Self { waves }
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

    pub fn add(&mut self, other: Wave) {
        self.waves.extend(other.waves)
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

impl Debug for Wave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.waves.iter();
        if let Some(wave) = iter.next() {
            write!(f, "{:?}", wave)?;
        }
        for wave in iter {
            write!(f, " + {:?}", wave)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq)]
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

impl Debug for SineWave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.amplitude != 1.0 {
            write!(f, "{}", self.amplitude)?;
        }
        write!(f, "sin(")?;
        if self.freq_comp != 1.0 {
            write!(f, "{} ", self.freq_comp)?;
        }
        write!(f, "x")?;
        if self.offset != 0.0 {
            write!(f, "- {}", self.offset)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
