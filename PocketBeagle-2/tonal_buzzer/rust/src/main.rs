//! Ported from Arduino Example:
//!
//! https://github.com/robsoncouto/arduino-songs/blob/master/harrypotter/harrypotter.ino

use std::thread::sleep;
use std::time::Duration;

use beagle_helper::sysfs;
use beagle_helper::tonal_buzzer::Tone;

// PWM_3 refers to the PWM provided by PRU0 PWM firmware.
const PWMCHIP_EXPORT_PATH: &str = "/dev/pocket/pwm/3/export";
const PWMCHIP_UNEXPORT_PATH: &str = "/dev/pocket/pwm/3/unexport";

// PWM_3 channel 3 is exported on header pin P2_30.
const PWM_CHANNEL: &str = "3";
const PWM_PATH: &str = "/dev/pocket/pwm/3/pwm3";

// Hedwig's theme from the Harry Potter Movies
// Score from https://musescore.com/user/3811306/scores/4906610
// NOTE: From a readability standpoint, this could be moved out of main.rs.
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
    let buzzer = sysfs::Device::with_export(PWMCHIP_EXPORT_PATH, PWM_CHANNEL, PWM_PATH).unwrap();
    let period = buzzer.sysfs_w("period").unwrap();
    let enable = buzzer.sysfs_w("enable").unwrap();
    let duty_cycle = buzzer.sysfs_w("duty_cycle").unwrap();

    println!("Start Playing");

    for (tone, dur) in MELODY {
        let node_dur = if *dur > 0 {
            WHOLE_NOTE / dur.abs() as f64
        } else {
            (WHOLE_NOTE / dur.abs() as f64) * 1.5
        };

        period.write_string(tone.period_ns()).unwrap();
        duty_cycle.write_string(tone.duty_cycle_ns()).unwrap();
        enable.write(b"1").unwrap();
        sleep(Duration::from_millis((node_dur * 0.9) as u64));
        enable.write(b"0").unwrap();
        sleep(Duration::from_millis((node_dur * 0.1) as u64));
    }

    pwm = Sysfs::unexport(PWMCHIP_UNEXPORT_PATH, PWM_CHANNEL).unwrap();
}
