use chrono::prelude::*;
use std::collections::HashSet;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use web_ical::{Calendar, Events};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = std::env::args().collect();

    // checks if environement variable ICAL_CALENDAR_URL is set or not. If yes, use it else expect url
    // passed from the command line.
    let ical_url = std::env::var("ICAL_CALENDAR_URL").unwrap_or_else(|_| args[1].clone());

    loop {
        let mut is_notified = HashSet::<String>::new();
        if let Err(e) = step(&ical_url, &mut is_notified) {
            tracing::warn!("Failed step: {e}");
        }
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

fn step(url: &str, is_notified: &mut HashSet<String>) -> anyhow::Result<()> {
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
                "Event {} is far away in the future {:?}.",
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

fn get_events(url: &str) -> anyhow::Result<Vec<Events>> {
    let icals = Calendar::new(url)?;
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
        "{:?} in {} seconds.",
        event.summary,
        (event.dtstart - now).num_seconds()
    );
}
