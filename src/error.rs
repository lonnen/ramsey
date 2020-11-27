#[derive(thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    File(#[from] std::io::Error),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;