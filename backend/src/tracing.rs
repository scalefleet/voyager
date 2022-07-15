pub use tracing::dispatcher::SetGlobalDefaultError;
pub use tracing_subscriber::{prelude::*, Registry};

pub trait ResultTracingExt {
    fn maybe_log(self) -> Self;
}

pub fn tracing_subscribe() -> Result<(), SetGlobalDefaultError> {
    let subscriber = Registry::default().with(tracing_subscriber::fmt::layer().pretty());

    tracing::subscriber::set_global_default(subscriber)
}
