use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use url::Url;

pub fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = std::env::args().collect();
    let ical_url = Url::parse(&args[1]).unwrap();
    tracing::info!("ical url: {ical_url:?}");

    let ical = reqwest::blocking::get(ical_url).unwrap().text().unwrap();
    println!("{ical}");
}
