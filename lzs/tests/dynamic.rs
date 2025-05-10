#![cfg(feature = "alloc")]

use common::{EXAMPLE_DATA, INIT_BYTE};
use lzs::{Lzs, ResultLzsErrorVoidExt, SliceReader, VecWriter};

mod common;

#[test]
#[ignore]
fn dynamic() {
    debug_assert!(false, "Disabled in debug mode");
    let lzs = Lzs::new(INIT_BYTE);
    let encoded = lzs
        .compress(
            SliceReader::new(EXAMPLE_DATA),
            VecWriter::with_capacity(EXAMPLE_DATA.len()),
        )
        .void_unwrap();
    let decoded = lzs
        .decompress(
            SliceReader::new(&encoded),
            VecWriter::with_capacity(EXAMPLE_DATA.len()),
        )
        .void_unwrap();
    assert_eq!(
        EXAMPLE_DATA,
        &decoded[..],
        "Lzs::new(0x{INIT_BYTE:02x}) Data mismatch"
    )
}
