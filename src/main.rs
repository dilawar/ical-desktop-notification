#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::prelude::*;
use clap::{Parser, Subcommand};
use std::collections::HashSet;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use web_ical::{Calendar, Events};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Debug verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Ical urls. If `ICAL_CALENDAR_URL`, use it too.
    #[arg(value_parser, num_args = 0.., value_delimiter = ' ')]
    icals: Vec<url::Url>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build
    Build,
}

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    let log_env = match cli.verbosity {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    std::env::set_var("RUST_LOG", log_env);

    let mut calendars = cli.icals.clone();

    // checks if environement variable ICAL_CALENDAR_URL is set or not. If yes, use it else expect url
    // passed from the command line.
    if let Ok(ical_url) = std::env::var("ICAL_CALENDAR_URL") {
        calendars.push(ical_url.parse().expect("invalid url"));
    }

    loop {
        let mut is_notified = HashSet::<String>::new();
        for ical_url in calendars.iter() {
            println!("Monitoring calendar `{ical_url}`...");
            if let Err(e) = step(&ical_url, &mut is_notified) {
                tracing::warn!("Failed step: {e}");
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

fn step(url: &url::Url, is_notified: &mut HashSet<String>) -> anyhow::Result<()> {
    let events = get_events(url)?;
    // iterate over upcoming events.
    for event in events
        .iter()
        .filter(|a| (a.dtstart - Utc::now()).num_seconds() > 0)
    {
        print_event(event);

        // notify user 3 minutes before that event.
        if time_to_event_secs(event) > 3 * 60 {
            tracing::info!(
                " Event {} is far away in the future {:?}.",
                event.summary,
                event.dtstart
            );
            continue;
        }

        if is_notified.get(&event.summary).is_none() {
            if notify_user(event).is_ok() {
                is_notified.insert(event.summary.clone());
            }
        } else {
            tracing::warn!("Already notified user about it.");
        }
    }
    Ok(())
}

fn get_events(url: &url::Url) -> anyhow::Result<Vec<Events>> {
    let icals = Calendar::new(url.as_str())?;
    let mut events = icals.events;
    events.sort_by(|a, b| a.dtstart.cmp(&b.dtstart));
    Ok(events)
}

/// Notify user if the event is after interval_secs
fn notify_user(event: &Events) -> anyhow::Result<()> {
    print_event(event);
    notify_rust::Notification::new()
        .summary(&format!(
            "{} starts in {} minutes",
            event.summary,
            time_to_event_secs(event) / 60
        ))
        .timeout(notify_rust::Timeout::Never)
        .show()?;
    Ok(())
}

fn time_to_event_secs(event: &Events) -> i64 {
    (event.dtstart - Utc::now()).num_seconds()
}

fn print_event(event: &Events) {
    let now = Utc::now();
    println!(
        "Event `{}` in {} seconds.",
        event.summary,
        (event.dtstart - now).num_seconds()
    );
}
