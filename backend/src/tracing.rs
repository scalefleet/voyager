use std::{error, result};
use tracing::{dispatcher::SetGlobalDefaultError, error};
use tracing_subscriber::{filter::LevelFilter, prelude::*, Registry};

pub fn tracing_subscribe() -> Result<(), SetGlobalDefaultError> {
    let subscriber = Registry::default()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(LevelFilter::INFO);

    tracing::subscriber::set_global_default(subscriber)
}

pub trait ResultTracingExt {
    fn maybe_log(self) -> Self;
}

impl<T, E> ResultTracingExt for result::Result<T, E>
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
}
