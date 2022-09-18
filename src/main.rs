use player::*;

fn main() -> Result<()> {
    let wave1 = Wave::new(vec![SineWave::from_note(A4)]);
    let wave2 = Wave::new(vec![SineWave::from_note(B6)]);
    let samples1 = Samples::new(SampleType::Wave(wave1));
    let samples2 = Samples::new(SampleType::Wave(wave2));
    let wav = WavAudio::stereo(samples1, samples2);
    wav.write_to_file("media/out.wav", 2.0)?;
    WavAudio::play("media/out.wav")?;
    println!("2");
    WavAudio::load("media/out.wav")?;
    wav.write_to_file("media/out.wav", 2.0)?;
    WavAudio::play("media/out.wav")?;
    Ok(())
}
