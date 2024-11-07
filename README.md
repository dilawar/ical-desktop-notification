Given the ical url of a calendar, this application sends a desktop notificaiton
when an event is about to occur (in 3 minutes).

You can either set the environment variable `ICAL_CALENDAR_URL` or pass it at
command line e.g. `cargo run -- <ical_url>`.

Thats all this app does!

## How to deploy on Windows

- Build in release mode `cargo b --release`. The debug build always opens a console window.
- Move the binary to your favrite location e.g. `C:\tools`.
- Open "Task Scheduler" and create a basic task with this binary path and ical url as argument.
  Launch it on startup. 
