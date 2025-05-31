#[derive(Debug)]
pub enum MutfError {
    Io(std::io::Error),
    Mutf(mutf8::error::Error),
}

impl From<std::io::Error> for MutfError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<mutf8::error::Error> for MutfError {
    fn from(value: mutf8::error::Error) -> Self {
        Self::Mutf(value)
    }
}

impl std::fmt::Display for MutfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MutfError::Io(error) => write!(f, "{error}"),
            MutfError::Mutf(error) => write!(f, "MUTF-8 error: {error}"),
        }
    }
}

impl std::error::Error for MutfError {}
