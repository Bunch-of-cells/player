use std::io::{Error, ErrorKind, Result};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};

use crate::Note;

#[derive(Copy, Clone)]
pub struct Beep {
    pub frequency: u32,
    pub delay: Duration,
    pub duration: Duration,
}

impl Beep {
    /// frequency in heartz
    pub fn new(frequency: u32, duration: Duration) -> Beep {
        Self {
            frequency,
            delay: Duration::from_secs(0),
            duration,
        }
    }

    pub fn from_note(note: Note, duration: Duration) -> Beep {
        Self::new(note.frequency() as u32, duration)
    }

    pub fn with_delay(self, delay: Duration) -> Self {
        Self { delay, ..self }
    }
}

pub struct Device {
    _inner: (),
}

impl Device {
    pub fn new() -> Result<Device> {
        Command::new("speaker-test")
            .arg("-h")
            .stdout(Stdio::null())
            .output()
            .map(|_| Device { _inner: () })
            .map_err(|e| {
                if e.kind() == ErrorKind::NotFound {
                    Error::new(ErrorKind::NotFound, "speaker-test not found")
                } else {
                    e
                }
            })
    }

    /// frequency in heartz
    pub fn beep(&self, frequency: u32, duration: Duration) -> Result<()> {
        let mut child = Command::new("speaker-test")
            .args(["-X", "-t", "sine", "-f"])
            .arg(frequency.to_string())
            .stdout(Stdio::null())
            .spawn()?;

        thread::sleep(duration);

        child.kill()?;

        Ok(())
    }

    pub fn play_beep(&self, beep: Beep) -> Result<()> {
        std::thread::sleep(beep.delay);

        let mut child = Command::new("speaker-test")
            .args(["-X", "-t", "sine", "-f"])
            .arg(beep.frequency.to_string())
            .stdout(Stdio::null())
            .spawn()?;

        thread::sleep(beep.duration);

        child.kill()?;

        Ok(())
    }

    pub fn play(&self, beeps: &[Beep]) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let flag = Arc::new(AtomicBool::new(false));
        let mut handles = Vec::new();
        for &beep in beeps {
            let tx = tx.clone();
            let flag = flag.clone();
            handles.push(thread::spawn(move || {
                thread::sleep(beep.delay);
                if flag.load(Ordering::Relaxed) {
                    return;
                }

                let mut child = match Command::new("speaker-test")
                    .args(["-X", "-t", "sine", "-f"])
                    .arg(beep.frequency.to_string())
                    .stdout(Stdio::null())
                    .spawn()
                {
                    Ok(child) => child,
                    Err(err) => {
                        tx.send(err).unwrap();
                        return;
                    }
                };

                let instant = Instant::now();

                while instant.elapsed() < beep.duration {
                    if flag.load(Ordering::Relaxed) {
                        let _ = child.kill();
                        return;
                    }
                }

                if let Err(err) = child.kill() {
                    tx.send(err).unwrap();
                }
            }));
        }

        drop(tx);

        if let Ok(err) = rx.recv() {
            flag.store(true, Ordering::Relaxed);
            return Err(err);
        }

        Ok(())
    }
}
