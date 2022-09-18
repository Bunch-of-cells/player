use std::{
    fs::File,
    io::{ErrorKind, Read, Write},
    path::Path,
    process::{Command, Stdio},
};

pub mod channel;
pub mod error;
pub mod sample;

pub use channel::Channel;
pub use error::{Error, Result};
pub use sample::{SampleType, Samples};

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

    pub fn play<P: AsRef<Path>>(path: P) -> Result<()> {
        let exit_status = Command::new("play")
            .arg(path.as_ref())
            .stderr(Stdio::null())
            .status()?;

        match exit_status.code() {
            Some(0) | None => Ok(()),
            Some(1) => Err(Error::ExecutionError(
                1,
                "Error occured while running 'play'".into(),
            )),
            Some(e @ (2 | 3 | 126)) => Err(Error::ExecutionError(e, "Permission denied".into())),
            Some(127) => Err(Error::ExecutionError(
                127,
                "'play' is not installed. Install sox to play .wav files".into(),
            )),
            Some(e) => Err(Error::ExecutionError(e, "Unkown Error".into())),
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let size = file.metadata()?.len() as u32;
        let mut riff = [0; 12];
        file.read_exact(&mut riff)?;
        match riff {
            [b'R', b'I', b'F', b'F', s1, s2, s3, s4, b'W', b'A', b'V', b'E'] => {
                if u32::from_le_bytes([s1, s2, s3, s4]) + 8 != size {
                    todo!()
                }
            }
            _ => todo!(),
        };

        let mut fmt = [0; 24];
        file.read_exact(&mut fmt)?;
        let (channels, sample_rate) = match fmt {
            [b'f', b'm', b't', b' ', s1, s2, s3, s4, af1, af2, c1, c2, sr1, sr2, sr3, sr4, br1, br2, br3, br4, ba1, ba2, bps1, bps2] =>
            {
                if u32::from_le_bytes([s1, s2, s3, s4]) != 16 {
                    todo!()
                }
                if u16::from_le_bytes([af1, af2]) != 1 {
                    todo!()
                }
                let channels = u16::from_le_bytes([c1, c2]);
                if !(1..=2).contains(&channels) {
                    todo!()
                }
                let sample_rate = u32::from_le_bytes([sr1, sr2, sr3, sr4]);

                if u16::from_le_bytes([bps1, bps2]) != 16 {
                    todo!()
                }

                if u16::from_le_bytes([ba1, ba2]) != channels * 2 {
                    todo!()
                }

                if u32::from_le_bytes([br1, br2, br3, br4]) != sample_rate * channels as u32 * 2 {
                    todo!()
                }

                (channels, sample_rate)
            }
            _ => todo!(),
        };

        let mut data = [0; 8];
        file.read_exact(&mut data)?;
        match data {
            [b'd', b'a', b't', b'a', s1, s2, s3, s4] => {
                if u32::from_le_bytes([s1, s2, s3, s4]) + 44 != size {
                    todo!()
                }
            }
            _ => todo!(),
        }

        let mut audio_data = Vec::new();
        // file.read_to_end(&mut audio_data)?;
        loop {
            let mut buf = [0; 2];
            match file.read_exact(&mut buf) {
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
                n => n?,
            }
            audio_data.push(i16::from_le_bytes(buf));
        }

        let channel = match channels {
            1 => {
                let samples =
                    Samples::new(SampleType::Pointsi16(audio_data)).with_sample_rate(sample_rate);
                Channel::Mono(samples)
            }
            2 => {
                let right = audio_data
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &x)| if i % 2 == 1 { None } else { Some(x) })
                    .collect();
                let left = audio_data
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &x)| if i % 2 == 0 { None } else { Some(x) })
                    .collect();
                let right =
                    Samples::new(SampleType::Pointsi16(right)).with_sample_rate(sample_rate);
                let left = Samples::new(SampleType::Pointsi16(left)).with_sample_rate(sample_rate);
                Channel::Stereo(right, left)
            }
            _ => unreachable!(),
        };

        Ok(WavAudio { channel })
    }

    fn write_metadata(&self, file: &mut File, samples: usize) -> Result<()> {
        let sample_rate = self.channel.sample_rate();
        let channels = self.channel.channels();
        let data_size = (samples * 2 * channels as usize) as u32;

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

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P, seconds: f32) -> Result<()> {
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
