use lzs::{IOSimpleReader, IOSimpleWriter, Lzs, LzsError, Read, Write};
use std::cell::RefCell;
use std::io::{stdin, stdout};
use std::num::ParseIntError;
use std::ops::AddAssign;
use std::process::exit;
use std::rc::Rc;
use std::str::FromStr;

// This is a very rudimentary program, everybody is welcome to improve it.

#[derive(Debug)]
struct Args {
    encode: bool,
    c: u8,
}

fn parse_dec_or_hex_u8(i: &str) -> Result<u8, ParseIntError> {
    if let Some(i) = i.strip_prefix("0x") {
        u8::from_str_radix(i, 16)
    } else {
        u8::from_str(i)
    }
}

fn parse_args() -> Result<Args, &'static str> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("not exactly 2 arguments");
    }
    let encode = match args[1].as_str() {
        "e" => Ok(true),
        "d" => Ok(false),
        _ => Err("unknown command, use 'e' or 'd'"),
    }?;
    let params: Vec<_> = args[2].split(',').collect();
    if params.len() != 1 {
        return Err("not exactly 1 compression parameter found");
    }
    let c = parse_dec_or_hex_u8(params[0].trim()).map_err(|_| "can't read c")?;

    Ok(Args { encode, c })
}

struct ReadCounter<T>(T, Rc<RefCell<usize>>);

impl<T: Read> Read for ReadCounter<T> {
    type Error = T::Error;

    fn read(&mut self) -> Result<Option<u8>, Self::Error> {
        let result = self.0.read();
        if let Ok(ok_result) = result {
            if ok_result.is_some() {
                self.1.borrow_mut().add_assign(1);
            }
        }
        result
    }
}

struct WriteCounter<T>(T, usize);

impl<T: Write> Write for WriteCounter<T> {
    type Output = (T::Output, usize);
    type Error = T::Error;

    fn write(&mut self, data: u8) -> Result<(), T::Error> {
        let result = self.0.write(data);
        if result.is_ok() {
            self.1 += 1;
        }
        result
    }

    fn finish(self) -> Result<Self::Output, Self::Error> {
        Ok((self.0.finish()?, self.1))
    }
}

fn main() {
    let args = parse_args().unwrap_or_else(|err| {
        let name = std::env::args().next().unwrap();
        eprintln!("error: {err}");
        eprintln!("usage: {name} <'e'|'d'> <c>");
        eprintln!("example: {name} e 0x20");
        exit(1)
    });
    let lzs = Lzs::new(args.c);
    let mut stdin = stdin();
    let mut stdout = stdout();
    let i_cnt = Rc::new(RefCell::new(0));
    match if args.encode {
        lzs.compress(
            ReadCounter(IOSimpleReader::new(&mut stdin), i_cnt.clone()),
            WriteCounter(IOSimpleWriter::new(&mut stdout), 0),
        )
    } else {
        lzs.decompress(
            ReadCounter(IOSimpleReader::new(&mut stdin), i_cnt.clone()),
            WriteCounter(IOSimpleWriter::new(&mut stdout), 0),
        )
    } {
        Ok(((), o_cnt)) => {
            let i_cnt = *i_cnt.borrow();
            if i_cnt > 0 && o_cnt > 0 {
                let mut ratio = (o_cnt as f64) / (i_cnt as f64);
                if !args.encode {
                    ratio = 1.0 / ratio;
                }
                eprintln!("the data compression is {:.2}%", (1.0 - ratio) * 100.0)
            }
        }
        Err(LzsError::ReadError(err)) => {
            eprintln!("error while reading: {err}");
            exit(1)
        }
        Err(LzsError::WriteError(err)) => {
            eprintln!("error while writing: {err}");
            exit(1)
        }
    }
}
