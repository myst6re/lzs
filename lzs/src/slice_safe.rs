use crate::read_write::{Read, Write};
use void::Void;

/// Read from a slice.
///
/// ```rust
/// # use lzs::*;
/// let input_data = [10, 42];
/// let mut input = SliceReader::new(&input_data);
/// assert_eq!(input.read(), Ok(Some(10)));
/// assert_eq!(input.read(), Ok(Some(42)));
/// assert_eq!(input.read(), Ok(None));
/// ```
///
/// Use [`void_read_unwrap`](crate::ResultLzsErrorVoidReadExt::void_read_unwrap) to remove the Void from the result.
/// Or [`void_unwrap`](crate::ResultLzsErrorVoidExt::void_unwrap) if also the writer produces Void.
pub struct SliceReader<'a> {
    data: &'a [u8],
}
impl<'a> SliceReader<'a> {
    /// Constructs a new reader.
    #[inline(always)]
    #[must_use]
    pub fn new(data: &'a [u8]) -> SliceReader<'a> {
        Self { data }
    }
}
impl Read for SliceReader<'_> {
    /// No error can occur.
    type Error = Void;
    #[inline(always)]
    fn read(&mut self) -> Result<Option<u8>, Self::Error> {
        if let Some((&first, rest)) = self.data.split_first() {
            self.data = rest;
            Ok(Some(first))
        } else {
            Ok(None)
        }
    }
}

/// A zero-sized type, will be returned in case of an error.
#[derive(Debug, Eq, PartialEq)]
pub struct SliceWriteError;

impl core::fmt::Display for SliceWriteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Over- or underflow happened")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SliceWriteError {}

/// Write into a slice.
///
/// Returns amount of written bytes.
///
/// ```rust
/// # use lzs::*;
/// let mut buf = [0; 2];
/// // underfull
/// let mut output = SliceWriter::new(&mut buf);
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.finish(), Ok(1));
/// // full
/// let mut output = SliceWriter::new(&mut buf);
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.finish(), Ok(2));
/// // overflow
/// let mut output = SliceWriter::new(&mut buf);
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.write(1), Err(SliceWriteError));
/// ```
pub struct SliceWriter<'a> {
    count: usize,
    data: &'a mut [u8],
}

impl<'a> SliceWriter<'a> {
    /// Constructs a new writer.
    #[inline(always)]
    #[must_use]
    pub fn new(data: &'a mut [u8]) -> SliceWriter<'a> {
        Self { count: 0, data }
    }
}

impl Write for SliceWriter<'_> {
    /// Returns written bytes.
    type Output = usize;
    /// In case of an overflow this error is returned.
    type Error = SliceWriteError;
    #[inline(always)]
    fn write(&mut self, data: u8) -> Result<(), Self::Error> {
        let d = core::mem::take(&mut self.data);
        let (first, rest) = d.split_first_mut().ok_or(SliceWriteError)?;
        *first = data;
        self.data = rest;
        self.count += 1;
        Ok(())
    }
    #[inline(always)]
    fn finish(self) -> Result<Self::Output, Self::Error> {
        Ok(self.count)
    }
}

/// Write into a slice which has the exact size of the result.
///
/// ```rust
/// # use lzs::*;
/// let mut buf = [0; 2];
/// // underfull
/// let mut output = SliceWriterExact::new(&mut buf);
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.finish(), Err(SliceWriteError));
/// // full
/// let mut output = SliceWriterExact::new(&mut buf);
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.finish(), Ok(()));
/// // overflow
/// let mut output = SliceWriterExact::new(&mut buf);
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.write(1), Ok(()));
/// assert_eq!(output.write(1), Err(SliceWriteError));
/// ```
pub struct SliceWriterExact<'a> {
    data: &'a mut [u8],
}

impl<'a> SliceWriterExact<'a> {
    /// Constructs a new writer.
    #[inline(always)]
    #[must_use]
    pub fn new(data: &'a mut [u8]) -> SliceWriterExact<'a> {
        Self { data }
    }
}

impl Write for SliceWriterExact<'_> {
    /// Returns always `()`.
    type Output = ();
    /// In case of an under- or overflow this error is returned.
    type Error = SliceWriteError;
    #[inline(always)]
    fn write(&mut self, data: u8) -> Result<(), Self::Error> {
        let d = core::mem::take(&mut self.data);
        let (first, rest) = d.split_first_mut().ok_or(SliceWriteError)?;
        *first = data;
        self.data = rest;
        Ok(())
    }
    #[inline(always)]
    fn finish(self) -> Result<Self::Output, Self::Error> {
        if self.data.is_empty() {
            Ok(())
        } else {
            Err(SliceWriteError)
        }
    }
}
