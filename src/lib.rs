pub mod generator;
pub mod note;
pub mod wav;
pub mod wave;

pub use generator::play_notes;
pub use note::*;
pub use wav::{Channel, SampleType, Samples, WavAudio, Error, Result};
pub use wave::{SineWave, Wave};
