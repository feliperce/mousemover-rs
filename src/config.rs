use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[clap(short, long, default_value = "1000")]
    pub delay_ms: u64,
}

impl Config {
    pub fn new() -> Self {
        Config::parse()
    }
}