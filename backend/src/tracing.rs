use colored::Colorize;
use std::{error, ops::AddAssign, result};
use tracing::{
    dispatcher::SetGlobalDefaultError, error, info, subscriber::set_global_default, Level,
    Subscriber,
};
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{self, format, FmtContext, FormatEvent, FormatFields},
    prelude::*,
    registry::LookupSpan,
    Registry,
};

pub fn tracing_subscribe() -> Result<(), SetGlobalDefaultError> {
    let subscriber = Registry::default()
        .with(fmt::layer().event_format(EventLogFormatter))
        .with(LevelFilter::INFO);

    set_global_default(subscriber)
}

pub trait ResultTracingExt<T> {
    fn on_ok_then_log(self, message: &str) -> Self;
    fn on_err_then_log(self, message: &str) -> Self;
    fn on_err_then_log_display(self) -> Self;
    fn expect_and_log(self, message: &str) -> T;
    fn unwrap_or_log(self) -> T;
}

impl<T, E> ResultTracingExt<T> for result::Result<T, E>
where
    E: error::Error,
{
    fn on_ok_then_log(self, message: &str) -> Self {
        if self.is_ok() {
            info!("{message}");
        }

        self
    }

    fn on_err_then_log(self, message: &str) -> Self {
        if let Err(error) = &self {
            let kind = format!("{:?}", error);
            error!("{}: {}", kind.bold().red(), message);
        }

        self
    }

    fn on_err_then_log_display(self) -> Self {
        if let Err(error) = &self {
            let kind = format!("{:?}", error);
            error!("{}: {}", kind.bold().red(), error);
        }

        self
    }

    fn expect_and_log(self, message: &str) -> T {
        if let Err(error) = &self {
            let kind = format!("{:?}", error);
            error!("{}: {}", kind.bold().red(), message);

            std::process::exit(1)
        }

        self.unwrap()
    }

    fn unwrap_or_log(self) -> T {
        if let Err(error) = &self {
            let kind = format!("{:?}", error);
            error!("{}: {}", kind.bold().red(), error);

            std::process::exit(1)
        }

        self.unwrap()
    }
}

struct EventLogFormatter;

impl<S, N> FormatEvent<S, N> for EventLogFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let metadata = event.metadata();

        let mut log = String::new();

        match *metadata.level() {
            Level::ERROR => {
                log.add_assign("  error: ".bold().red().to_string().as_str());
            }
            Level::WARN => {
                log.add_assign("warning: ".bold().yellow().to_string().as_str());
            }
            Level::INFO => {
                log.add_assign("   info: ".bold().blue().to_string().as_str());
            }
            _ => {}
        }

        write!(&mut writer, "{}", log)?;
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(&mut writer)
    }
}
