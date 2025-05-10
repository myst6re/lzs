use core::fmt::Display;

/// This represents either an read or write error.
#[derive(Debug, Eq, PartialEq)]
pub enum LzsError<R, W> {
    /// Contains the read error value.
    ReadError(R),
    /// Contains the write error value.
    WriteError(W),
}

impl<R: Display, W: Display> core::fmt::Display for LzsError<R, W> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LzsError::ReadError(error) => write!(f, "Read error: {error}"),
            LzsError::WriteError(error) => write!(f, "Write error: {error}"),
        }
    }
}

/// Implementation of [`Error`](std::error::Error) for [`LzsError`]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
impl<R, W> std::error::Error for LzsError<R, W>
where
    R: std::error::Error + 'static,
    W: std::error::Error + 'static,
{
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LzsError::ReadError(error) => Some(error),
            LzsError::WriteError(error) => Some(error),
        }
    }
}

impl<R, W> LzsError<R, W> {
    /// Maps a `LzsError<R, W>` to `LzsError<E, W>` by applying a function to a contained read error value, leaving an write error value untouched.
    #[inline]
    pub fn map_read_error<E, O: FnOnce(R) -> E>(self, op: O) -> LzsError<E, W> {
        match self {
            LzsError::ReadError(e) => LzsError::ReadError(op(e)),
            LzsError::WriteError(e) => LzsError::WriteError(e),
        }
    }
    /// Maps a `LzsError<R, W>` to `LzsError<R, E>` by applying a function to a contained write error value, leaving an read error value untouched.
    #[inline]
    pub fn map_write_error<E, O: FnOnce(W) -> E>(self, op: O) -> LzsError<R, E> {
        match self {
            LzsError::ReadError(e) => LzsError::ReadError(e),
            LzsError::WriteError(e) => LzsError::WriteError(op(e)),
        }
    }
}
