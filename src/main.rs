use player::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wave = Wave::new(vec![SineWave::from_note(A4)]);
    let samples = Samples::new(SampleType::Wave(wave));
    let wav = WavAudio::mono(samples);
    wav.write_to_file("media/out.wav", 2.0)?;
    WavAudio::play("media/out.wav")?;
    Ok(())
}
