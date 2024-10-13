use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use web_ical::{Calendar, Events};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = std::env::args().collect();

    let icals = Calendar::new(&args[1]);
    for event in &icals.events {
        println!("{:?}", event.summary);
    }
}
