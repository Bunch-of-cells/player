use std::{
    fs::File,
    io::{self, Seek, SeekFrom, Write},
    path::Path,
};

use crate::Wave;

pub struct WavConstructor {
    wave: Wave,
    sample_rate: f32,
    sample_no: f32,
}

impl WavConstructor {
    pub fn new(wave: Wave) -> WavConstructor {
        WavConstructor {
            sample_rate: 44100.0,
            sample_no: 0f32,
            wave,
        }
    }

    pub fn with_sample_rate(self, sample_rate: u32) -> Self {
        Self {
            sample_rate: sample_rate as f32,
            ..self
        }
    }

    pub fn wave(&mut self) -> &mut Wave {
        &mut self.wave
    }

    pub fn next_point(&mut self) -> f32 {
        let x = self.sample_no / self.sample_rate;
        self.sample_no += 1.0;
        self.wave.at(x)
    }

    pub fn write_to_file<P: AsRef<Path>>(&mut self, path: P, seconds: f32) -> io::Result<()> {
        let mut file = File::create(path)?;
        let max_amplitude = 2u32.pow(15) as f32;
        let samples = (self.sample_rate * seconds) as u32;

        // Header chunk
        file.write_all(b"RIFF----WAVE")?;

        // Format chunk
        file.write_all(b"fmt ")?;
        file.write_all(&16u32.to_le_bytes())?; // Size
        file.write_all(&1u16.to_le_bytes())?; // Compression
        file.write_all(&2u16.to_le_bytes())?; // Number of channels
        file.write_all(&(self.sample_rate as u32).to_le_bytes())?; // Sample rate
        file.write_all(&(self.sample_rate as u32 * 2).to_le_bytes())?; // Byte rate
        file.write_all(&4u16.to_le_bytes())?; // Block align
        file.write_all(&16u16.to_le_bytes())?; // Bit depth

        // Data chunk
        file.write_all(b"data----")?;

        for _ in 0..samples {
            let sample = self.next_point();
            let int_sample = (sample * max_amplitude) as i16;
            file.write_all(&int_sample.to_le_bytes())?;
        }

        let data_size = samples * 2;

        file.seek(SeekFrom::Start(40))?;
        file.write_all(&data_size.to_le_bytes())?;

        file.seek(SeekFrom::Start(4))?;
        file.write_all(&(data_size + 36).to_le_bytes())?;

        Ok(())
    }
}
