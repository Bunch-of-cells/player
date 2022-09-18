use crate::wav::Samples;

#[derive(Clone, PartialEq, Debug)]
pub enum Channel {
    Mono(Samples),
    Stereo(Samples, Samples),
}

impl Channel {
    pub fn channels(&self) -> u16 {
        match self {
            Self::Mono(..) => 1,
            Self::Stereo(..) => 2,
        }
    }

    pub fn sample_rate(&self) -> u32 {
        match self {
            Self::Mono(s) => s.sample_rate(),
            Self::Stereo(r, _) => r.sample_rate(),
        }
    }
}
