use enigo::{Enigo, MouseControllable};
use rand::Rng;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};

pub struct MouseMover {
    enigo: Enigo,
}

impl MouseMover {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(),
        }
    }

    pub fn move_to_random_position(&mut self) {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0..1920);
        let y = rng.gen_range(0..1080);

        self.enigo.mouse_move_to(x, y);
    }

    pub fn run(&mut self, delay_ms: u64) {
        let device_state = DeviceState::new();
        println!("Mouse mover started. Press CTRL + ALT + C to exit.");

        loop {
            self.move_to_random_position();

            let keys: Vec<Keycode> = device_state.get_keys();
            if keys.contains(&Keycode::LControl) && keys.contains(&Keycode::LAlt) && keys.contains(&Keycode::C) {
                println!("Hotkey detected: CTRL + ALT + C. Exiting...");
                break;
            }

            std::thread::sleep(Duration::from_millis(delay_ms));
        }
    }
}
