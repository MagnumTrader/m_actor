use derive_more::derive::From;
use tokio::task::JoinError;

/// Errors for the [`mactor`] crate
#[derive(Debug, From)]
pub enum Error {
    JoinError(JoinError),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
