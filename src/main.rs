use player::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // play_notes("media/instrumental.mp3", C4.frequency(), B6.frequency(), 13)?;
    let wave = Wave::new(vec![SineWave::from_note(C4)]);
    println!("{:?}", wave);
    Ok(())
}
