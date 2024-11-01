use std::{thread, time::Duration};
use device_query::{DeviceQuery, DeviceState, Keycode};

pub fn center_screen() {
    let device_state = DeviceState::new();
    let mut counter = 0;

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.contains(&Keycode::RControl) {
            counter += 1;

            if counter == 2 {
                super::Screen::new().center();
                counter = 0;
            }
        } else {
            counter = 0;
        }

        thread::sleep(Duration::from_millis(200));
    }
}