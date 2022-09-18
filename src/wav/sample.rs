use crate::Wave;

#[derive(Clone, PartialEq, Debug)]
pub struct Samples {
    samples: SampleType,
    sample_rate: f32,
}

impl Samples {
    pub const MAX_AMPLITUDE: f32 = 2u32.pow(15) as f32;

    pub fn new(samples: SampleType) -> Self {
        Self {
            samples,
            sample_rate: 44100.0,
        }
    }

    pub fn with_sample_rate(mut self, sample_rate: u32) -> Self {
        self.sample_rate = sample_rate as f32;
        self
    }

    pub fn sample(&self, i: usize) -> Option<i16> {
        match &self.samples {
            SampleType::Wave(wave) => {
                Some((wave.at(i as f32 / self.sample_rate) * Self::MAX_AMPLITUDE) as i16)
            }
            SampleType::Pointsi16(points) => points.get(i).copied(),
            SampleType::Pointsf32(points) => {
                points.get(i).map(|x| (x * Self::MAX_AMPLITUDE) as i16)
            }
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate as u32
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SampleType {
    Wave(Wave),
    Pointsi16(Vec<i16>),
    Pointsf32(Vec<f32>),
}

impl SampleType {
    pub fn sample_count(&self) -> Option<usize> {
        match self {
            Self::Wave(..) => None,
            Self::Pointsi16(s, ..) => Some(s.len()),
            Self::Pointsf32(s, ..) => Some(s.len()),
        }
    }
}
