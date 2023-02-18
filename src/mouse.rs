use rdev::{simulate, Button, EventType};

fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(_) => {
            println!("We could not send {event_type:?}");
        }
    }
}

pub fn move_to(location: (f64, f64)) {
    send(&EventType::MouseMove {
        x: location.0 / 1.25,
        y: location.1 / 1.25,
    });
}

pub fn left_click() {
    send(&EventType::ButtonPress(Button::Left));
    send(&EventType::ButtonRelease(Button::Left));
}

pub fn rigth_click() {
    send(&EventType::ButtonPress(Button::Right));
    send(&EventType::ButtonRelease(Button::Right));
}
