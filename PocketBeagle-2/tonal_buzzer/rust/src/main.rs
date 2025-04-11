use std::fs::OpenOptions;
use std::thread::sleep;
use std::time::Duration;

use beagle_helper::chardev::{CharDev, InputEvent};

mod hedwig;

const OFF_NOTE: InputEvent = InputEvent::with_frequency(0);

fn main() {
    let mut buzzer =
        CharDev::open_input_with_name("pwm-beeper", OpenOptions::new().write(true)).unwrap();

    println!("Start Playing");
    for (t, d) in hedwig::MELODY {
        let node_dur = if *d > 0 {
            hedwig::WHOLE_NOTE / d.abs() as f64
        } else {
            (hedwig::WHOLE_NOTE / d.abs() as f64) * 1.5
        };

        // Frequency = 0 is used to stop the buzzer
        let value = t.map(|x| x.freq().round() as i32).unwrap_or(0);
        let note = InputEvent::with_frequency(value);

        buzzer.write_evt(note).unwrap();
        sleep(Duration::from_millis((node_dur * 0.9) as u64));

        buzzer.write_evt(OFF_NOTE).unwrap();
        sleep(Duration::from_millis((node_dur * 0.1) as u64));
    }
}
