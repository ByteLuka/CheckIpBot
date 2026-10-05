use std::env;
use std::fmt;

use chrono::Local;
use tracing::{Event, Level, Subscriber, warn};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;

const RESET: &str = "\x1b[0m";

/// Formats events as `2026-01-01 12:00:00 | INFO     | target - message`.
struct LineFormat {
    color: bool,
}

impl<S, N> FormatEvent<S, N> for LineFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let meta = event.metadata();
        let (name, color) = match *meta.level() {
            Level::TRACE => ("TRACE", "\x1b[35m"),
            Level::DEBUG => ("DEBUG", "\x1b[36m"),
            Level::INFO => ("INFO", "\x1b[32m"),
            Level::WARN => ("WARNING", "\x1b[33m"),
            Level::ERROR => ("ERROR", "\x1b[31m"),
        };
        let (color, reset) = if self.color { (color, RESET) } else { ("", "") };

        write!(
            writer,
            "{} | {color}{name:<8}{reset} | {} - ",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            meta.target(),
        )?;
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

fn parse_level(value: &str) -> Option<Level> {
    match value.to_ascii_uppercase().as_str() {
        "DEBUG" => Some(Level::DEBUG),
        "INFO" => Some(Level::INFO),
        "WARNING" | "WARN" => Some(Level::WARN),
        // There is no level above error, so CRITICAL logs errors only.
        "ERROR" | "CRITICAL" => Some(Level::ERROR),
        _ => None,
    }
}

/// Sets up logging according to the `LOG_LEVEL` environment variable (default `INFO`).
pub fn init() {
    let requested = env::var("LOG_LEVEL").ok();
    let parsed = requested.as_deref().map(parse_level);
    let level = parsed.flatten().unwrap_or(Level::INFO);

    // Dependencies never log below WARNING, the bot itself follows LOG_LEVEL.
    let filter = Targets::new()
        .with_default(level.min(Level::WARN))
        .with_target(env!("CARGO_CRATE_NAME"), level);
    let format = LineFormat {
        color: env::var_os("NO_COLOR").is_none(),
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stderr)
                .event_format(format),
        )
        .with(filter)
        .init();

    if let (Some(value), Some(None)) = (requested, parsed) {
        warn!("Unknown LOG_LEVEL {value:?}, falling back to INFO");
    }
}
