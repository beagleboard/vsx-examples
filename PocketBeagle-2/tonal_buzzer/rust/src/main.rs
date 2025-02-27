//! Ported from Arduino Example:
//!
//! https://github.com/robsoncouto/arduino-songs/blob/master/harrypotter/harrypotter.ino

use std::{thread::sleep, time::Duration};

use beagle_linux_sdk::{boards::pocketbeagle2::P2_30, tonal_buzzer::Tone, TonalBuzzer};

// Hedwig's theme fromn the Harry Potter Movies
// Score from https://musescore.com/user/3811306/scores/4906610
const MELODY: &[(Option<Tone>, i32)] = &[
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

const TEMPO: f64 = 144.0;
const WHOLE_NOTE: f64 = (60000.0 * 4.0) / TEMPO;

fn main() {
    let buzzer = TonalBuzzer::new(P2_30).unwrap();

    println!("Start Playing");

    for (t, d) in MELODY {
        let node_dur = if *d > 0 {
            WHOLE_NOTE / d.abs() as f64
        } else {
            (WHOLE_NOTE / d.abs() as f64) * 1.5
        };

        buzzer.play(*t).unwrap();
        sleep(Duration::from_millis((node_dur * 0.9) as u64));
        buzzer.stop().unwrap();
        sleep(Duration::from_millis((node_dur * 0.1) as u64));
    }

    buzzer.stop().unwrap();
}
