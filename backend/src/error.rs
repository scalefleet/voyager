pub enum Error {
    AuthorizationPending,
    Other,
    Unauthenticated,
    TooFrequentPolling,
}

impl std::error::Error for Error {}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AuthorizationPending => {
                write!(f, "AuthorizationPending")
            }
            Error::Other => {
                write!(f, "Other")
            }
            Error::Unauthenticated => {
                write!(f, "Unauthenticated")
            }
            Error::TooFrequentPolling => {
                write!(f, "TooFrequentPolling")
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AuthorizationPending => {
                write!(
                    f,
                    "waiting for the required user action to continue the authorization"
                )
            }
            Error::Other => {
                write!(f, "other kind of error")
            }
            Error::Unauthenticated => {
                write!(
                    f,
                    "you have to login or provide a service token to use this app"
                )
            }
            Error::TooFrequentPolling => {
                write!(
                    f,
                    "too frequent polling in short interval, try again with longer interval"
                )
            }
        }
    }
}
