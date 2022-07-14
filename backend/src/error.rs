#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::Unauthenticated,
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

pub enum ErrorKind {
    Unauthenticated,
    BadRequest,
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadRequest => {
                write!(f, "BAD REQUEST")
            }
            Self::Unauthenticated => {
                write!(f, "UNAUTHENTICATED")
            }
        }
    }
}
