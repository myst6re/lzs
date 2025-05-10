use crate::error::LzsError;
use void::{unreachable, Void};

/// Conversion from `Result<T, LzsError<Void, Void>>` to `T`.
pub trait ResultLzsErrorVoidExt<T>: Sized {
    /// Get the value out of a wrapper.
    fn void_unwrap(self) -> T;
}

impl<T> ResultLzsErrorVoidExt<T> for Result<T, LzsError<Void, Void>> {
    /// Get the value out of an always-ok Result.
    ///
    /// Never panics, since it is statically known to be Ok.
    #[inline(always)]
    fn void_unwrap(self) -> T {
        match self {
            Ok(val) => val,
            Err(LzsError::ReadError(e)) => unreachable(e),
            Err(LzsError::WriteError(e)) => unreachable(e),
        }
    }
}

/// Conversion from `Result<T, LzsError<Void, E>>` to `Result<T, E>`.
///
/// It removes the statically known [`LzsError`] layer from the Result.
pub trait ResultLzsErrorVoidReadExt<E, T>: Sized {
    /// Remove the [`LzsError`] layer from the Result.
    fn void_read_unwrap(self) -> Result<T, E>;
}

impl<E, T> ResultLzsErrorVoidReadExt<E, T> for Result<T, LzsError<Void, E>> {
    /// Remove the [`LzsError`] layer from the Result.
    ///
    /// Never panics, since it is statically known to be Ok.
    #[inline]
    fn void_read_unwrap(self) -> Result<T, E> {
        match self {
            Ok(val) => Ok(val),
            Err(LzsError::ReadError(e)) => unreachable(e),
            Err(LzsError::WriteError(e)) => Err(e),
        }
    }
}

/// Conversion from `Result<T, LzsError<E, Void>>` to `Result<T, E>`.
///
/// It removes the statically known [`LzsError`] layer from the Result.
pub trait ResultLzsErrorVoidWriteExt<E, T>: Sized {
    /// Remove the [`LzsError`] layer from the Result.
    fn void_write_unwrap(self) -> Result<T, E>;
}

impl<E, T> ResultLzsErrorVoidWriteExt<E, T> for Result<T, LzsError<E, Void>> {
    /// Remove the [`LzsError`] layer from the Result.
    ///
    /// Never panics, since it is statically known to be Ok.
    #[inline]
    fn void_write_unwrap(self) -> Result<T, E> {
        match self {
            Ok(val) => Ok(val),
            Err(LzsError::ReadError(e)) => Err(e),
            Err(LzsError::WriteError(e)) => unreachable(e),
        }
    }
}
