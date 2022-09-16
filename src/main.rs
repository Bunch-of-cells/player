use player::{wav::WavConstructor, Device, SineWave, Wave, A4};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // play_notes("media/instrumental.mp3")?;

    let wave = Wave::new(vec![SineWave::from_note(A4)]);

    WavConstructor::mono(wave).write_to_file("media/out.wav", 2.0)?;
    Device::new()?.beep(440, Duration::from_secs(2))?;
    Ok(())
}
