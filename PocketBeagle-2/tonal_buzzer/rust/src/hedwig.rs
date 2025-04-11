//! Ported from Arduino Example:
//!
//! https://github.com/robsoncouto/arduino-songs/blob/master/harrypotter/harrypotter.ino

use beagle_helper::tone::Tone;

pub const TEMPO: f64 = 144.0;
pub const WHOLE_NOTE: f64 = (60000.0 * 4.0) / TEMPO;

// Hedwig's theme fromn the Harry Potter Movies
// Score from https://musescore.com/user/3811306/scores/4906610
pub const MELODY: &[(Option<Tone>, i32)] = &[
    // Opening
    (None, 2),
    (Some(Tone::D.octave(4)), 4),
    (Some(Tone::G.octave(4)), -4),
    (Some(Tone::A_SHARP.octave(4)), 8),
    (Some(Tone::A.octave(4)), 4),
    (Some(Tone::G.octave(4)), 2),
    (Some(Tone::D.octave(5)), 4),
    (Some(Tone::C.octave(5)), -2),
    (Some(Tone::A.octave(4)), -2),
    (Some(Tone::G.octave(4)), -4),
    (Some(Tone::A_SHARP.octave(4)), 8),
    (Some(Tone::A.octave(4)), 4),
    (Some(Tone::F.octave(4)), 2),
    (Some(Tone::G_SHARP.octave(4)), 4),
    (Some(Tone::D.octave(4)), -1),
    (Some(Tone::D.octave(4)), 4),
    // Part 2
    (Some(Tone::G.octave(4)), -4),
    (Some(Tone::A_SHARP.octave(4)), 8),
    (Some(Tone::A.octave(4)), 4),
    (Some(Tone::G.octave(4)), 2),
    (Some(Tone::D.octave(5)), 4),
    (Some(Tone::F.octave(5)), 2),
    (Some(Tone::E.octave(5)), 4),
    (Some(Tone::D_SHARP.octave(5)), 2),
    (Some(Tone::B.octave(4)), 4),
    (Some(Tone::D_SHARP.octave(5)), -4),
    (Some(Tone::D.octave(5)), 8),
    (Some(Tone::C_SHARP.octave(5)), 4),
    (Some(Tone::C_SHARP.octave(4)), 2),
    (Some(Tone::B.octave(4)), 4),
    (Some(Tone::G.octave(4)), -1),
    (Some(Tone::A_SHARP.octave(4)), 4),
    // Part 3
    (Some(Tone::D.octave(5)), 2),
    (Some(Tone::A_SHARP.octave(4)), 4),
    (Some(Tone::D.octave(5)), 2),
    (Some(Tone::A_SHARP.octave(4)), 4),
    (Some(Tone::D_SHARP.octave(5)), 2),
    (Some(Tone::D.octave(5)), 4),
    (Some(Tone::C_SHARP.octave(5)), 2),
    (Some(Tone::A.octave(4)), 4),
    (Some(Tone::A_SHARP.octave(4)), -4),
    (Some(Tone::D.octave(5)), 8),
    (Some(Tone::C_SHARP.octave(5)), 4),
    (Some(Tone::C_SHARP.octave(4)), 2),
    (Some(Tone::D.octave(4)), 4),
    (Some(Tone::D.octave(5)), -1),
    (None, 4),
    (Some(Tone::A_SHARP.octave(4)), 4),
    // Part 4
    (Some(Tone::D.octave(5)), 2),
    (Some(Tone::A_SHARP.octave(4)), 4),
    (Some(Tone::D.octave(5)), 2),
    (Some(Tone::A_SHARP.octave(4)), 4),
    (Some(Tone::F.octave(5)), 2),
    (Some(Tone::E.octave(5)), 4),
    (Some(Tone::D_SHARP.octave(5)), 2),
    (Some(Tone::B.octave(4)), 4),
    (Some(Tone::D_SHARP.octave(5)), -4),
    (Some(Tone::D.octave(5)), 8),
    (Some(Tone::C_SHARP.octave(5)), 4),
    (Some(Tone::C_SHARP.octave(4)), 2),
    (Some(Tone::A_SHARP.octave(4)), 4),
    (Some(Tone::G.octave(4)), -1),
];
