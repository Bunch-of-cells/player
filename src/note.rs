use crate::note;
use std::fmt::Display;

pub use NoteType::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Note {
    pub note: NoteType,
    pub octave: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NoteType {
    A = 2750,
    Bb = 2914,
    B = 3087,
    C = 1635,
    Db = 1732,
    D = 1835,
    Eb = 1945,
    E = 2060,
    F = 2183,
    Gb = 2312,
    G = 2450,
    Ab = 2596,
}

impl NoteType {
    pub const fn in_frequency_order() -> [NoteType; 12] {
        use NoteType::*;
        [C, Db, D, Eb, E, F, Gb, G, Ab, A, Bb, B]
    }

    pub const fn in_order() -> [NoteType; 12] {
        use NoteType::*;
        [A, Bb, B, C, Db, D, Eb, E, F, Gb, G, Ab]
    }

    #[inline]
    pub fn frequency(&self) -> f32 {
        *self as u32 as f32 / 100.0
    }

    #[inline]
    pub fn wave(&self) -> impl Fn(f32) -> f32 {
        let f = *self as u32 as f32 / 100.0;
        move |x| (f * std::f32::consts::TAU * x).sin()
    }
}

impl Note {
    /// Creates a new note
    /// # Panics
    /// Panics if octave is above 8
    #[track_caller]
    pub const fn new(note: NoteType, octave: u32) -> Self {
        assert!(octave <= 8, "Invalid octave");
        Self { note, octave }
    }

    #[inline]
    pub fn frequency(&self) -> f32 {
        (self.note as u32 * 2u32.pow(self.octave)) as f32 / 100f32
    }

    #[inline]
    pub const fn amplitude(&self) -> f32 {
        1.0
    }

    #[inline]
    pub fn closest_note(mut freq: f32) -> Note {
        freq /= C0.frequency();
        let mut t = (freq.log2() * 12.0).floor();
        if 2f32.powf(t + 1.0) - freq <= freq - 2f32.powf(t) {
            t += 1.0;
        }
        let t = t as u32;
        let octave = t / 12;
        let note_type = NoteType::in_frequency_order()[(t % 12) as usize];
        Note::new(note_type, octave)
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{} ({}Hz)", self.note, self.octave, self.frequency())
    }
}

note! {
    A0 -> A + 0
    Bb0 -> Bb + 0
    B0 -> B + 0
    C0 -> C + 0
    Db0 -> Db + 0
    D0 -> D + 0
    Eb0 -> Eb + 0
    E0 -> E + 0
    F0 -> F + 0
    Gb0 -> Gb + 0
    G0 -> G + 0
    Ab0 -> Ab + 0

    A1 -> A + 1
    Bb1 -> Bb + 1
    B1 -> B + 1
    C1 -> C + 1
    Db1 -> Db + 1
    D1 -> D + 1
    Eb1 -> Eb + 1
    E1 -> E + 1
    F1 -> F + 1
    Gb1 -> Gb + 1
    G1 -> G + 1
    Ab1 -> Ab + 1

    A2 -> A + 2
    Bb2 -> Bb + 2
    B2 -> B + 2
    C2 -> C + 2
    Db2 -> Db + 2
    D2 -> D + 2
    Eb2 -> Eb + 2
    E2 -> E + 2
    F2 -> F + 2
    Gb2 -> Gb + 2
    G2 -> G + 2
    Ab2 -> Ab + 2

    A3 -> A + 3
    Bb3 -> Bb + 3
    B3 -> B + 3
    C3 -> C + 3
    Db3 -> Db + 3
    D3 -> D + 3
    Eb3 -> Eb + 3
    E3 -> E + 3
    F3 -> F + 3
    Gb3 -> Gb + 3
    G3 -> G + 3
    Ab3 -> Ab + 3

    A4 -> A + 4
    Bb4 -> Bb + 4
    B4 -> B + 4
    C4 -> C + 4
    Db4 -> Db + 4
    D4 -> D + 4
    Eb4 -> Eb + 4
    E4 -> E + 4
    F4 -> F + 4
    Gb4 -> Gb + 4
    G4 -> G + 4
    Ab4 -> Ab + 4

    A5 -> A + 5
    Bb5 -> Bb + 5
    B5 -> B + 5
    C5 -> C + 5
    Db5 -> Db + 5
    D5 -> D + 5
    Eb5 -> Eb + 5
    E5 -> E + 5
    F5 -> F + 5
    Gb5 -> Gb + 5
    G5 -> G + 5
    Ab5 -> Ab + 5

    A6 -> A + 6
    Bb6 -> Bb + 6
    B6 -> B + 6
    C6 -> C + 6
    Db6 -> Db + 6
    D6 -> D + 6
    Eb6 -> Eb + 6
    E6 -> E + 6
    F6 -> F + 6
    Gb6 -> Gb + 6
    G6 -> G + 6
    Ab6 -> Ab + 6

    A7 -> A + 7
    Bb7 -> Bb + 7
    B7 -> B + 7
    C7 -> C + 7
    Db7 -> Db + 7
    D7 -> D + 7
    Eb7 -> Eb + 7
    E7 -> E + 7
    F7 -> F + 7
    Gb7 -> Gb + 7
    G7 -> G + 7
    Ab7 -> Ab + 7

    A8 -> A + 8
    Bb8 -> Bb + 8
    B8 -> B + 8
    C8 -> C + 8
    Db8 -> Db + 8
    D8 -> D + 8
    Eb8 -> Eb + 8
    E8 -> E + 8
    F8 -> F + 8
    Gb8 -> Gb + 8
    G8 -> G + 8
    Ab8 -> Ab + 8
}

#[macro_export]
macro_rules! note {
    ($($note: ident -> $n: ident + $octave: literal)*) => {
        $(
            #[allow(non_upper_case_globals)]
            pub const $note: $crate::note::Note = $crate::note::Note::new($crate::note::NoteType::$n, $octave);
        )*
    };
}
