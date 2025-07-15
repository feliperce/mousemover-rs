use enigo::{Enigo, MouseControllable};
use rand::Rng;
use std::time::Duration;

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
        loop {
            self.move_to_random_position();
            std::thread::sleep(Duration::from_millis(delay_ms));
        }
    }
}