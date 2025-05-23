use crate::read_write::Write;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use void::Void;

/// Write into a vector.
///
/// In order to write into a referenced vector use [`IOSimpleWriter`](crate::IOSimpleWriter),
/// and [`SliceReader`](crate::SliceReader) to read from a vector.
///
/// Use [`void_write_unwrap`](crate::ResultLzsErrorVoidWriteExt::void_write_unwrap) to remove the Void from the result.
/// Or [`void_unwrap`](crate::ResultLzsErrorVoidExt::void_unwrap) if also the reader produces Void.
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
pub struct VecWriter(Vec<u8>);
impl VecWriter {
    /// Constructs a new, empty writer with the specified capacity.
    ///
    /// Note: The vector is not truncated to the actually used space.
    #[inline(always)]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> VecWriter {
        VecWriter(Vec::with_capacity(capacity))
    }
}

impl Write for VecWriter {
    /// Returns the generated vector.
    type Output = Vec<u8>;
    /// No error can occur.
    type Error = Void;
    #[inline(always)]
    fn write(&mut self, data: u8) -> Result<(), Self::Error> {
        self.0.push(data);
        Ok(())
    }
    #[inline(always)]
    fn finish(self) -> Result<Self::Output, Self::Error> {
        Ok(self.0)
    }
}
