use crate::tracing::ResultTracingExt;
use crate::{Error, ErrorKind};
use std::result;
use tracing::error;

pub type Result<T> = result::Result<T, Error>;

impl<T> ResultTracingExt for Result<T> {
    fn maybe_log(self) -> Self {
        if let Err(error) = &self {
            match error.kind {
                ErrorKind::BadRequest => {
                    let kind = format!("{:?}", error.kind);
                    error!(target: "planetscale", kind, "Failed to fetch data.");
                }
                ErrorKind::Unauthenticated => {
                    let kind = format!("{:?}", error.kind);
                    error!(target: "planetscale", kind, "User is not authenticated.");
                }
            }
        }

        self
    }
}