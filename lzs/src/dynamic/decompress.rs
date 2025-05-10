/* This file is based on the LZSS encoder-decoder  (c) Haruhiko Okumura */

use crate::dynamic::Lzs;
use crate::error::LzsError;
use crate::macros::{get, set};
use crate::read_write::{Read, Write};

impl Lzs {
    // Allow many single char names, this is done to copy the original code as close as possible.
    #[allow(clippy::many_single_char_names)]
    #[inline(always)]
    pub(crate) fn decompress_internal<R: Read, W: Write>(
        &self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<(), LzsError<R::Error, W::Error>> {
        let mut buffer = [self.c; Self::n()];
        let mut r = Self::n() - Self::f();
        let mut flags: usize = 0;

        loop {
            flags >>= 1;

            if (flags & 256) == 0 {
                if let Some(c) = reader.read().map_err(LzsError::ReadError)? {
                    flags = c as usize | 0xFF00
                } else {
                    return Ok(());
                }
            }

            if (flags & 1) != 0 {
                if let Some(c) = reader.read().map_err(LzsError::ReadError)? {
                    writer.write(c).map_err(LzsError::WriteError)?;
                    set!(buffer, r, c);
                    r = (r + 1) & (Self::n() - 1);
                } else {
                    return Ok(());
                }
            } else {
                if let (Some(c1), Some(c2)) = (
                    reader.read().map_err(LzsError::ReadError)?,
                    reader.read().map_err(LzsError::ReadError)?,
                ) {
                    let i = c1 as usize | ((c2 as usize & 0xF0) << 4);
                    let j = (c2 as usize & 0x0F) + Self::threshold();
                    for k in 0..=j {
                        let c = get!(buffer, (i + k) & (Self::n() - 1));
                        writer.write(c).map_err(LzsError::WriteError)?;
                        set!(buffer, r, c);
                        r = (r + 1) & (Self::n() - 1);
                    }
                } else {
                    return Ok(());
                }
            }
        }
    }
}
