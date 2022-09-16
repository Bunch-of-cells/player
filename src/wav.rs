use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use crate::Wave;

#[derive(Clone, PartialEq, Debug)]
pub enum Channel {
    Mono(Wave),
    Stereo(Wave, Wave),
}

impl Channel {
    pub fn channels(&self) -> u16 {
        match self {
            Self::Mono(..) => 1,
            Self::Stereo(..) => 2,
        }
    }
}

pub struct WavConstructor {
    channel: Channel,
    sample_rate: f32,
    sample_no: f32,
}

impl WavConstructor {
    pub fn from_channel(channel: Channel) -> WavConstructor {
        WavConstructor {
            sample_rate: 44100.0,
            sample_no: 0f32,
            channel,
        }
    }

    pub fn mono(wave: Wave) -> WavConstructor {
        WavConstructor {
            sample_rate: 44100.0,
            sample_no: 0f32,
            channel: Channel::Mono(wave),
        }
    }

    pub fn stereo(right: Wave, left: Wave) -> WavConstructor {
        WavConstructor {
            sample_rate: 44100.0,
            sample_no: 0f32,
            channel: Channel::Stereo(right, left),
        }
    }

    pub fn with_sample_rate(self, sample_rate: u32) -> Self {
        Self {
            sample_rate: sample_rate as f32,
            ..self
        }
    }

    pub fn get_channel(&mut self) -> &mut Channel {
        &mut self.channel
    }

    pub fn write_to_file<P: AsRef<Path>>(&mut self, path: P, seconds: f32) -> io::Result<()> {
        let mut file = File::create(path)?;
        let max_amplitude = 2u32.pow(15) as f32;
        let samples = (self.sample_rate * seconds) as u32;
        let data_size = samples as i32 * 4;
        let channel = self.channel.channels();

        // Header chunk
        file.write_all(b"RIFF")?;
        file.write_all(&(data_size + 36).to_le_bytes())?;
        file.write_all(b"WAVE")?;

        // Format chunk
        file.write_all(b"fmt ")?;
        file.write_all(&16u32.to_le_bytes())?; // Size
        file.write_all(&1u16.to_le_bytes())?; // AudioFormat
        file.write_all(&channel.to_le_bytes())?; // NumChannels
        file.write_all(&(self.sample_rate as u32).to_le_bytes())?; // SampleRate
        file.write_all(&(self.sample_rate as u32 * channel as u32 * 2).to_le_bytes())?; // ByteRate = SampleRate * NumChannels * BitsPerSample/8
        file.write_all(&(channel * 2).to_le_bytes())?; // BlockAlign = NumChannels * BitsPerSample/8
        file.write_all(&16u16.to_le_bytes())?; // BitsPerSample

        // Data chunk
        file.write_all(b"data")?;
        file.write_all(&(data_size).to_le_bytes())?; // NumSamples * NumChannels * BitsPerSample/8

        for _ in 0..samples {
            let sample = self.sample_no / self.sample_rate;
            self.sample_no += 1.0;

            match &self.channel {
                Channel::Mono(wave) => {
                    let sample = (wave.at(sample) * max_amplitude) as i16;
                    file.write_all(&sample.to_le_bytes())?;
                }

                Channel::Stereo(right, left) => {
                    let right_sample = (right.at(sample) * max_amplitude) as i16;
                    file.write_all(&right_sample.to_le_bytes())?;

                    let left_sample = (left.at(sample) * max_amplitude) as i16;
                    file.write_all(&left_sample.to_le_bytes())?;
                }
            }
        }

        Ok(())
    }
}
