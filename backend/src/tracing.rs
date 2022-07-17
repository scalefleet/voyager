use std::{error, result};
use tracing::{dispatcher::SetGlobalDefaultError, error};
use tracing_subscriber::{filter::LevelFilter, prelude::*, Registry};

pub fn tracing_subscribe() -> Result<(), SetGlobalDefaultError> {
    let subscriber = Registry::default()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(LevelFilter::INFO);

    tracing::subscriber::set_global_default(subscriber)
}

pub trait ResultTracingExt<T> {
    fn maybe_log(self) -> Self;
    fn expect_and_log(self, message: &str) -> T;
    fn unwrap_or_log(self) -> T;
}

impl<T, E> ResultTracingExt<T> for result::Result<T, E>
where
    E: error::Error,
{
    fn maybe_log(self) -> Self {
        if let Err(error) = &self {
            let kind = format!("{:?}", error);
            error!(target: "planetscale", kind, "{}", error);
        }

        self
    }

    fn expect_and_log(self, message: &str) -> T {
        if let Err(error) = &self {
            let kind = format!("{:?}", error);
            error!(target: "planetscale", kind, "{}: {}", message, error);

            std::process::exit(1)
        }

        self.unwrap()
    }

    fn unwrap_or_log(self) -> T {
        if let Err(error) = &self {
            let kind = format!("{:?}", error);
            error!(target: "planetscale", kind, "{}", error);

            std::process::exit(1)
        }

        self.unwrap()
    }
}
