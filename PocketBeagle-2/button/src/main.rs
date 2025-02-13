/// PIN name
const BTN: &str = "P1_20";

/// Find line by PIN Name.
fn find_line_by_name(name: &str) -> (gpiod::Chip, u32) {
    let chips = gpiod::Chip::list_devices().expect("Failed to get line names");

    for chip_path in chips {
        let chip = gpiod::Chip::new(chip_path).unwrap();
        let num_lines = chip.num_lines();

        for line in 0..num_lines {
            match chip.line_info(line) {
                Ok(info) if info.name == name => return (chip, line),

                _ => {}
            }
        }
    }

    panic!("PIN {} not found", name);
}

fn main() {
    let (chip, offset) = find_line_by_name(BTN);

    let opts = gpiod::Options::input([offset])
        .consumer("button")
        .bias(gpiod::Bias::PullUp)
        .edge(gpiod::EdgeDetect::Both);

    let pin = chip.request_lines(opts).unwrap();
    let mut last_edge = gpiod::Edge::Rising;

    let evts = pin.filter_map(|x| x.ok());

    // Iterate over events
    for evt in evts {
        // This is required since gpiod can trigger same events multiple times. Hence we
        // want to detect event cahnges rather than events themselves.
        if (last_edge, evt.edge) == (gpiod::Edge::Rising, gpiod::Edge::Falling) {
            println!("Button pressed");
        }
        last_edge = evt.edge;
    }
}
