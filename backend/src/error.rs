pub struct Error {
    pub kind: ErrorKind,
}

pub enum ErrorKind {
    Unauthenticated,
    BadRequest,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl std::error::Error for Error {}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::BadRequest => {
                write!(f, "BadRequest")
            }
            ErrorKind::Unauthenticated => {
                write!(f, "NotAuthenticated")
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::BadRequest => {
                write!(f, "BadRequest")
            }
            ErrorKind::Unauthenticated => {
                write!(f, "NotAuthenticated")
            }
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Self {
            kind: ErrorKind::BadRequest,
        }
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(_: reqwest::header::InvalidHeaderValue) -> Self {
        Self {
            kind: ErrorKind::BadRequest,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::Unauthenticated,
        }
    }
}
