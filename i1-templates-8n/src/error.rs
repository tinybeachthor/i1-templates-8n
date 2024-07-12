use std::convert::Infallible;
use std::fmt;

pub type Result<I, E = Error> = ::std::result::Result<I, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("formatting error")]
    Fmt(#[from] fmt::Error),
}

impl From<Infallible> for Error {
    #[inline]
    fn from(value: Infallible) -> Self {
        match value {}
    }
}
