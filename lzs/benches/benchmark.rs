use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use lzs::{Lzs, ResultLzsErrorVoidExt, SliceReader, VecWriter};

const MY_DYN_LZS: Lzs = Lzs::new(0x20);

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("compress dyn example", |b| {
        b.iter_batched(
            || {
                (
                    SliceReader::new(EXAMPLE_DATA),
                    VecWriter::with_capacity(EXAMPLE_DATA.len()),
                )
            },
            |(r, w)| MY_DYN_LZS.compress(r, w).void_unwrap(),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("decompress dyn example", |b| {
        b.iter_batched(
            || {
                let compressed = MY_DYN_LZS
                    .compress(
                        SliceReader::new(EXAMPLE_DATA),
                        VecWriter::with_capacity(EXAMPLE_DATA.len()),
                    )
                    .void_unwrap();
                (
                    compressed,
                    VecWriter::with_capacity(EXAMPLE_DATA.len()),
                )
            },
            |(r, w)| {
                MY_DYN_LZS
                    .decompress(SliceReader::new(&r), w)
                    .void_unwrap()
            },
            BatchSize::SmallInput,
        )
    });
}

const EXAMPLE_DATA: &[u8; 781] = br#"
/* LZSS encoder-decoder (Haruhiko Okumura; public domain) */

void decode(void)
{
	int  i, j, k, r, c;
	unsigned int  flags;

	for (i = 0; i < N - F; i++) text_buf[i] = ' ';
	r = N - F;  flags = 0;
	for ( ; ; ) {
		if (((flags >>= 1) & 256) == 0) {
			if ((c = getc(infile)) == EOF) break;
			flags = c | 0xff00;		/* uses higher byte cleverly */
		}							/* to count eight */
		if (flags & 1) {
			if ((c = getc(infile)) == EOF) break;
			putc(c, outfile);  text_buf[r++] = c;  r &= (N - 1);
		} else {
			if ((i = getc(infile)) == EOF) break;
			if ((j = getc(infile)) == EOF) break;
			i |= ((j & 0xf0) << 4);  j = (j & 0x0f) + THRESHOLD;
			for (k = 0; k <= j; k++) {
				c = text_buf[(i + k) & (N - 1)];
				putc(c, outfile);  text_buf[r++] = c;  r &= (N - 1);
			}
		}
	}
}
"#;

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
