use crate::error::LzsError;
use crate::read_write::{Read, Write};

mod compress;
mod decompress;

/// Dynamic parameters for de-/compression (see [Lzs](crate::Lzs) for compile-time parameters).
///
/// # Parameters
/// * `c` - The initial fill byte of the buffer, usually `0x20` (space)
///
/// # Example
/// ```rust
/// # use lzs::{Lzs, ResultLzsErrorVoidExt, SliceReader, VecWriter};
/// let my_lzs = Lzs::new(0x20);
/// let input = b"Example Data";
/// let result = my_lzs.compress(
///   SliceReader::new(input),
///   VecWriter::with_capacity(30),
/// );
/// assert_eq!(result.void_unwrap().len(), 14); // the output is 14 bytes long
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lzs {
    c: u8,
}

impl Lzs {
    /// Create new LZSS parameters.
    ///
    /// If the parameter are not valid (see above) an error is returned.
    ///
    /// For creating a const see [`Lzs::as_dyn`](crate::generic::Lzs::as_dyn).
    #[must_use]
    pub const fn new(c: u8) -> Self {
        Lzs { c }
    }

    #[inline(always)]
    #[must_use]
    pub(crate) const fn ei() -> usize {
        12
    }

    #[inline(always)]
    #[must_use]
    pub(crate) const fn ej() -> usize {
        4
    }

    #[inline(always)]
    #[must_use]
    pub(crate) const fn threshold() -> usize {
        2
    }

    #[inline(always)]
    #[must_use]
    pub(crate) const fn n() -> usize {
        1 << Self::ei()
    }

    #[inline(always)]
    #[must_use]
    pub(crate) const fn f() -> usize {
        (1 << Self::ej()) + Self::threshold()
    }

    /// Compress the input data into the output.
    ///
    /// The buffer, with `2 * (1 << EI)` bytes, is allocated on the heap.
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[cfg(feature = "alloc")]
    pub fn compress<R: Read, W: Write>(
        &self,
        mut reader: R,
        mut writer: W,
    ) -> Result<W::Output, LzsError<R::Error, W::Error>> {
        self.compress_internal(&mut reader, &mut writer)?;
        writer.finish().map_err(LzsError::WriteError)
    }

    /// Decompress the input data into the output.
    ///
    /// The buffer, with `1 << EI` bytes, is allocated on the heap.
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[cfg(feature = "alloc")]
    pub fn decompress<R: Read, W: Write>(
        &self,
        mut reader: R,
        mut writer: W,
    ) -> Result<W::Output, LzsError<R::Error, W::Error>> {
        self.decompress_internal(&mut reader, &mut writer)?;
        writer.finish().map_err(LzsError::WriteError)
    }
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use crate::dynamic::Lzs;
    use crate::slice::SliceReader;
    use crate::vec::VecWriter;
    use crate::void::ResultLzsErrorVoidExt;

    const TEST_LZS: Lzs = Lzs::new(0x20);
    const TEST_DATA: &[u8; 27] = b"Sample   Data   11221233123";
    const COMPRESSED_DATA: [u8; 27] = [
        191, 83, 97, 109, 112, 108, 101, 235, 240, 68, 247, 97, 116, 97, 235, 240, 49, 49, 50, 50,
        15, 49, 50, 51, 51, 2, 0,
    ];

    #[test]
    fn test_decompress() {
        let output = TEST_LZS
            .decompress(
                SliceReader::new(&COMPRESSED_DATA),
                VecWriter::with_capacity(TEST_DATA.len()),
            )
            .void_unwrap();
        assert_eq!(output.as_slice(), TEST_DATA);
    }

    #[test]
    fn test_compress() {
        let output = TEST_LZS
            .compress(
                SliceReader::new(TEST_DATA),
                VecWriter::with_capacity(COMPRESSED_DATA.len()),
            )
            .void_unwrap();
        assert_eq!(output.as_slice(), COMPRESSED_DATA);
    }

    #[test]
    fn test_compress_big() {
        let big_test_data = include_bytes!("mod.rs");
        // compress
        let output1 = TEST_LZS
            .compress(
                SliceReader::new(big_test_data),
                VecWriter::with_capacity(big_test_data.len()),
            )
            .void_unwrap();
        // decompress
        let output2 = TEST_LZS
            .decompress(
                SliceReader::new(&output1),
                VecWriter::with_capacity(big_test_data.len()),
            )
            .void_unwrap();
        assert_eq!(output2.as_slice(), big_test_data);
    }
}
