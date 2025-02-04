use linux_embedded_hal::gpio_cdev::{self, EventRequestFlags, EventType, Line, LineRequestFlags};

/// PIN name
const BTN: &str = "P1_20";

/// Find line by PIN Name.
fn find_line_by_name(name: &str) -> Line {
    let chips = gpio_cdev::chips()
        .expect("Failed to get line names")
        .filter_map(|x| x.ok());

    for chip in chips {
        for line in chip.lines() {
            match line.info() {
                Ok(info) if info.name() == Some(name) => return line,
                _ => {}
            }
        }
    }

    panic!("PIN {} not found", name);
}

fn main() {
    let line = find_line_by_name(BTN);

    // let line_handle = line.request(LineRequestFlags::INPUT, 0, "button").unwrap();
    let mut last_edge = EventType::RisingEdge;

    let events = line
        .events(
            LineRequestFlags::INPUT,
            EventRequestFlags::BOTH_EDGES,
            "button",
        )
        .unwrap()
        .filter_map(|x| x.ok());

    // Iterate over events
    for evt in events {
        if (last_edge, evt.event_type()) == (EventType::RisingEdge, EventType::FallingEdge) {
            println!("Button pressed");
        }
        last_edge = evt.event_type();
    }
}
