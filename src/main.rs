mod config;
mod mouse;

fn main() {
    let config = config::Config::new();
    let mut mouse_mover = mouse::MouseMover::new();

    mouse_mover.run(config.delay_ms);
}
