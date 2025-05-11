/* This file is based on the LZSS encoder-decoder  (c) Haruhiko Okumura */

use crate::dynamic::Lzs;
use crate::error::LzsError;
use crate::macros::{get, set};
use crate::read_write::{Read, Write};

impl Lzs {
    // Allow many single char names, this is done to copy the original code as close as possible.
    #![allow(clippy::many_single_char_names)]
    #[inline(always)]
    pub(crate) fn compress_internal<R: Read, W: Write>(
        self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<(), LzsError<R::Error, W::Error>> {
        // Initialize the text_buf with C (a character that will appear often)
        let mut text_buf = [self.c; Self::n() + Self::f() - 1];
        // Initialize trees with N
        let mut lson = [Self::n() as u16; Self::n() + 1];
        let mut rson = [Self::n() as u16; Self::n() + 257];
        let mut dad = [Self::n() as u16; Self::n() + 1];
        /* code_buf[1..16] saves eight units of code, and
         * code_buf[0] works as eight flags, "1" representing that the unit
         * is an unencoded letter (1 byte), "0" a position-and-length pair
         * (2 bytes).  Thus, eight units require at most 16 bytes of code.
         */
        let mut code_buf = [0u8; 17];
        let mut mask = 1;
        let mut code_buf_ptr = 1;
        let mut s = 0;
        let mut r = Self::n() - Self::f();
        let mut len = 0;

        while len < Self::f() {
            if let Some(c) = reader.read().map_err(LzsError::ReadError)? {
                set!(text_buf, r + len, c);
            } else {
                break;
            }

            len += 1;
        }

        if len == 0 {
            return Ok(());
        }

        for i in 1..=Self::f() {
            Self::insert_node(r - i, &mut lson, &mut rson, &mut dad, &text_buf);
        }

        let (mut match_position, mut match_length) =
            Self::insert_node(r, &mut lson, &mut rson, &mut dad, &text_buf);

        loop {
            if match_length > len {
                match_length = len;
            }

            if match_length <= Self::threshold() {
                match_length = 1;
                set!(code_buf, 0, get!(code_buf, 0) | mask);
                set!(code_buf, code_buf_ptr, get!(text_buf, r));
                code_buf_ptr += 1;
            } else {
                set!(code_buf, code_buf_ptr, match_position as u8);
                code_buf_ptr += 1;
                set!(
                    code_buf,
                    code_buf_ptr,
                    (((match_position >> 4) & 0xF0) | (match_length - (Self::threshold() + 1)))
                        as u8
                );
                code_buf_ptr += 1;
            }

            mask <<= 1;

            if mask == 0 {
                for c in code_buf.iter().take(code_buf_ptr) {
                    writer.write(*c).map_err(LzsError::WriteError)?;
                }
                set!(code_buf, 0, 0);
                mask = 1;
                code_buf_ptr = 1;
            }

            let last_match_length = match_length;
            let mut i = 0;

            while i < last_match_length {
                if let Some(c) = reader.read().map_err(LzsError::ReadError)? {
                    Self::delete_node(s, &mut lson, &mut rson, &mut dad);
                    set!(text_buf, s, c);

                    if s < Self::f() - 1 {
                        set!(text_buf, s + Self::n(), c);
                    }

                    s = (s + 1) & (Self::n() - 1);
                    r = (r + 1) & (Self::n() - 1);

                    (match_position, match_length) =
                        Self::insert_node(r, &mut lson, &mut rson, &mut dad, &text_buf);
                } else {
                    break;
                }

                i += 1;
            }

            while i < last_match_length {
                Self::delete_node(s, &mut lson, &mut rson, &mut dad);
                s = (s + 1) & (Self::n() - 1);
                r = (r + 1) & (Self::n() - 1);
                len -= 1;
                if len > 0 {
                    (match_position, match_length) =
                        Self::insert_node(r, &mut lson, &mut rson, &mut dad, &text_buf);
                }
                i += 1;
            }

            if len == 0 {
                break;
            }
        }

        if code_buf_ptr > 1 {
            // Send remaining code
            for c in code_buf.iter().take(code_buf_ptr) {
                writer.write(*c).map_err(LzsError::WriteError)?;
            }
        }

        Ok(())
    }

    /**
     * Inserts string of length F, `text_buf[r..r+F-1]`, into one of the
     * trees (`text_buf[r]`'th tree) and returns the longest-match position
     * and length.
     * If `match_length` = F, then removes the old node in favor of the new
     * one, because the old one will be deleted sooner.
     * Note r plays double role, as tree node and position in buffer.
     */
    #[inline(always)]
    fn insert_node(
        r: usize,
        lson: &mut [u16; Self::n() + 1],
        rson: &mut [u16; Self::n() + 257],
        dad: &mut [u16; Self::n() + 1],
        text_buf: &[u8; Self::n() + Self::f() - 1],
    ) -> (usize, usize) {
        let mut match_position = 0;
        let mut match_length = 0;

        let mut cmp = 1i32;
        let mut p = Self::n() + 1 + get!(text_buf, r) as usize;

        set!(lson, r, Self::n() as u16);
        set!(rson, r, Self::n() as u16);

        loop {
            if cmp >= 0 {
                if get!(rson, p) == Self::n() as u16 {
                    set!(rson, p, r as u16);
                    set!(dad, r, p as u16);
                    return (match_position, match_length);
                }
                p = get!(rson, p) as usize;
            } else if get!(lson, p) == Self::n() as u16 {
                set!(lson, p, r as u16);
                set!(dad, r, p as u16);
                return (match_position, match_length);
            } else {
                p = get!(lson, p) as usize;
            }

            let mut i = 1;
            while i < Self::f() {
                cmp = get!(text_buf, r + i) as i32 - get!(text_buf, p + i) as i32;
                if cmp != 0 {
                    break;
                }
                i += 1;
            }

            if i > match_length {
                match_position = p;
                match_length = i;

                if match_length >= Self::f() {
                    break;
                }
            }
        }

        set!(dad, r, get!(dad, p));
        set!(lson, r, get!(lson, p));
        set!(rson, r, get!(rson, p));

        let e = get!(lson, p) as usize;
        set!(dad, e, r as u16);
        let e = get!(rson, p) as usize;
        set!(dad, e, r as u16);

        let e = get!(dad, p) as usize;
        if get!(rson, e) == p as u16 {
            set!(rson, e, r as u16);
        } else {
            set!(lson, e, r as u16);
        }

        set!(dad, p, Self::n() as u16); // Remove p

        (match_position, match_length)
    }

    /**
     * deletes node p from tree
     */
    #[inline(always)]
    fn delete_node(
        p: usize,
        lson: &mut [u16; Self::n() + 1],
        rson: &mut [u16; Self::n() + 257],
        dad: &mut [u16; Self::n() + 1],
    ) {
        if get!(dad, p) == Self::n() as u16 {
            return; // Not in tree
        }

        let q = if get!(rson, p) == Self::n() as u16 {
            get!(lson, p) as usize
        } else if get!(lson, p) == Self::n() as u16 {
            get!(rson, p) as usize
        } else {
            let mut q = get!(lson, p) as usize;
            if get!(rson, q) != Self::n() as u16 {
                loop {
                    q = get!(rson, q) as usize;

                    if get!(rson, q) == Self::n() as u16 {
                        break;
                    }
                }
                let e = get!(dad, q) as usize;
                set!(rson, e, get!(lson, q));
                let e = get!(lson, q) as usize;
                set!(dad, e, get!(dad, q));
                set!(lson, q, get!(lson, p));
                let e = get!(lson, p) as usize;
                set!(dad, e, q as u16);
            }
            set!(rson, q, get!(rson, p));
            let e = get!(rson, p) as usize;
            set!(dad, e, q as u16);
            q
        };

        let e = get!(dad, p);
        set!(dad, q, e);

        let e = get!(dad, p) as usize;
        if get!(rson, e) == p as u16 {
            set!(rson, e, q as u16);
        } else {
            set!(lson, e, q as u16);
        }

        set!(dad, p, Self::n() as u16);
    }
}
