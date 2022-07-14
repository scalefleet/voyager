pub struct Error {
    kind: ErrorKind,
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
