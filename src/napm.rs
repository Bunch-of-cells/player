use minimp3::{Decoder as Mp3Decoder, Error as Mp3Error, Frame as Mp3Frame};
use spectrum_analyzer::scaling::scale_to_zero_to_one;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use std::fs::File;
use std::time::Duration;

use crate::{Beep, Device, Note, NoteType, C0};

pub fn play_notes(
    file: &'static str,
    min_freq: f32,
    max_freq: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let (samples, sampling_rate) = read_mp3_to_mono(file);
    let batch_2pow = 13u32;

    // Hann Window code in lib
    let mut windowed_samples = Vec::with_capacity(samples.len());
    let samples_len_f32 = samples.len() as f32;
    for (i, &sample) in samples
        .iter()
        .enumerate()
        .step_by(2usize.pow(batch_2pow.saturating_sub(13)))
    {
        let two_pi_i = 2.0 * std::f32::consts::PI * i as f32;
        let idontknowthename = (two_pi_i / samples_len_f32).cos();
        let multiplier = 0.5 * (1.0 - idontknowthename);
        windowed_samples.push(multiplier * sample as f32)
    }

    let batch_size = 2usize.pow(batch_2pow);
    let batch_duration = batch_size as f32 / sampling_rate as f32;

    println!("Sample Count : {:?}", samples.len());
    println!(
        "Duration : {:?}s",
        samples.len() as f32 / sampling_rate as f32
    );
    println!("Sampling Rate : {:?}Hz", sampling_rate);
    println!("Batches : {:?}", windowed_samples.len() / batch_size);
    println!("Batch Duration : {:?}s", batch_duration);
    println!("======================================================");

    let loops = windowed_samples.len() / batch_size as usize;
    let duration = Duration::from_secs_f32(batch_duration);
    let device = Device::new()?;

    for i in 0..loops {
        let note = closest_note(
            samples_fft_to_spectrum(
                &windowed_samples[(i * batch_size)..][..batch_size],
                sampling_rate,
                FrequencyLimit::Range(min_freq, max_freq),
                Some(&scale_to_zero_to_one),
            )
            .unwrap()
            .max()
            .0
            .val(),
        );
        println!("{}", note);
        device.play_beep(Beep::from_note(note, duration))?;
    }

    Ok(())
}

#[inline]
fn closest_note(mut n: f32) -> Note {
    n /= C0.frequency();
    let mut t = (n.log2() * 12.0).floor();
    if 2f32.powf(t + 1.0) - n <= n - 2f32.powf(t) {
        t += 1.0;
    }
    let t = t as u32;
    let octave = t / 12;
    let note_type = NoteType::in_frequency_order()[(t % 12) as usize];
    Note::new(note_type, octave)
}

fn read_mp3_to_mono(file: &str) -> (Vec<i16>, u32) {
    let mut decoder = Mp3Decoder::new(File::open(file).unwrap());

    let mut sampling_rate = 0;
    let mut mono_samples = vec![];
    loop {
        match decoder.next_frame() {
            Ok(Mp3Frame {
                data: samples_of_frame,
                sample_rate,
                channels,
                ..
            }) => {
                sampling_rate = sample_rate;

                if channels == 2 {
                    for (i, sample) in samples_of_frame.iter().enumerate().step_by(2) {
                        let sample = *sample as i32;
                        let next_sample = samples_of_frame[i + 1] as i32;
                        mono_samples.push(((sample + next_sample) as f32 / 2.0) as i16);
                    }
                } else if channels == 1 {
                    mono_samples.extend_from_slice(&samples_of_frame);
                } else {
                    panic!("Unsupported number of channels={}", channels);
                }
            }
            Err(Mp3Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }

    (mono_samples, sampling_rate as u32)
}
