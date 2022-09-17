use std::{
    fs::File,
    io::{self, Error, ErrorKind, Write},
    path::Path,
    process::{Command, Stdio},
};

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

pub struct WavAudio {
    channel: Channel,
}

impl WavAudio {
    pub fn from_channel(channel: Channel) -> WavAudio {
        if let Channel::Stereo(r, l) = &channel {
            assert_eq!(r.sample_rate(), l.sample_rate());
        }

        WavAudio { channel }
    }

    pub fn mono(samples: Samples) -> WavAudio {
        WavAudio {
            channel: Channel::Mono(samples),
        }
    }

    pub fn stereo(right: Samples, left: Samples) -> WavAudio {
        assert_eq!(right.sample_rate(), left.sample_rate());
        WavAudio {
            channel: Channel::Stereo(right, left),
        }
    }

    pub fn get_channel(&mut self) -> &mut Channel {
        &mut self.channel
    }

    pub fn play<P: AsRef<Path>>(path: P) -> io::Result<()> {
        let exit_status = Command::new("play")
            .arg(path.as_ref())
            .stderr(Stdio::null())
            .status()?;
        match exit_status.code() {
            Some(0) | None => Ok(()),
            Some(1) => Err(Error::new(
                ErrorKind::Other,
                "Error occured while running 'play'",
            )),
            Some(2 | 3 | 126) => Err(Error::new(ErrorKind::PermissionDenied, "Permission denied")),
            Some(127) => Err(Error::new(
                ErrorKind::Other,
                "'play' is not installed. Install sox to play .wav files",
            )),
            Some(e) => Err(Error::new(
                ErrorKind::Other,
                format!("Exited with error code: {}", e),
            )),
        }
    }

    fn write_metadata(&self, file: &mut File, samples: usize) -> io::Result<()> {
        let sample_rate = self.channel.sample_rate();
        let channels = self.channel.channels();
        let data_size = (samples * 2 * channels as usize) as i32;

        // Header chunk
        file.write_all(b"RIFF")?;
        file.write_all(&(data_size + 36).to_le_bytes())?;
        file.write_all(b"WAVE")?;

        // Format chunk
        file.write_all(b"fmt ")?;
        file.write_all(&16u32.to_le_bytes())?; // Size
        file.write_all(&1u16.to_le_bytes())?; // AudioFormat
        file.write_all(&channels.to_le_bytes())?; // NumChannels
        file.write_all(&sample_rate.to_le_bytes())?; // SampleRate
        file.write_all(&(sample_rate * channels as u32 * 2).to_le_bytes())?; // ByteRate = SampleRate * NumChannels * BitsPerSample/8
        file.write_all(&(channels * 2).to_le_bytes())?; // BlockAlign = NumChannels * BitsPerSample/8
        file.write_all(&16u16.to_le_bytes())?; // BitsPerSample

        // Data chunk
        file.write_all(b"data")?;
        file.write_all(&(data_size).to_le_bytes())?; // NumSamples * NumChannels * BitsPerSample/8

        Ok(())
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P, seconds: f32) -> io::Result<()> {
        let mut file = File::create(path)?;
        let sample_rate = self.channel.sample_rate() as f32;
        let samples = (sample_rate * seconds) as usize;

        self.write_metadata(&mut file, samples)?;

        for i in 0..samples {
            match &self.channel {
                Channel::Mono(samples) => {
                    let sample = samples.sample(i).unwrap();
                    file.write_all(&sample.to_le_bytes())?;
                }

                Channel::Stereo(right, left) => {
                    let right = right.sample(i).unwrap();
                    file.write_all(&right.to_le_bytes())?;

                    let left = left.sample(i).unwrap();
                    file.write_all(&left.to_le_bytes())?;
                }
            }
        }

        Ok(())
    }
}
