use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "MouseMover-RS",
    about = "A Rust application that moves the mouse cursor to random positions on the screen with a configurable delay.",
    after_help = "Press CTRL + ALT + C at any time to stop the program."
)]
pub struct Config {
    #[clap(
        short, 
        long, 
        default_value = "1000", 
        help = "Set the delay between mouse movements in milliseconds"
    )]
    pub delay_ms: u64,
}

impl Config {
    pub fn new() -> Self {
        Config::parse()
    }
}
