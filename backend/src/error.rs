pub struct Error {
    pub kind: ErrorKind,
}

pub enum ErrorKind {
    Unauthenticated,
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
            ErrorKind::Unauthenticated => {
                write!(f, "Unauthenticated")
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::Unauthenticated => {
                write!(
                    f,
                    "you have to login or provide a service token to use this app"
                )
            }
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
