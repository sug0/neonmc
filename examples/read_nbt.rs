use std::fs::File;
use flate2::bufread::GzDecoder;
use std::io::{self, Read, BufRead, BufReader, Stdin};

use neonmc::serialize::DataInput;
use neonmc::serialize::nbt::NBT;

enum In {
    Stdin(BufReader<Stdin>),
    File(BufReader<File>),
}

impl Read for In {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            In::Stdin(f) => f.read(buf),
            In::File(f) => f.read(buf),
        }
    }
}

impl BufRead for In {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self {
            In::Stdin(f) => f.fill_buf(),
            In::File(f) => f.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            In::Stdin(f) => f.consume(amt),
            In::File(f) => f.consume(amt),
        }
    }
}

fn open_nbt_file(path: &str) -> io::Result<GzDecoder<In>> {
    let r = if path == "<stdin>" {
        let stdin = BufReader::new(io::stdin());
        In::Stdin(stdin)
    } else {
        let f = BufReader::new(File::open(path)?);
        In::File(f)
    };
    Ok(GzDecoder::new(r))
}

fn main() -> io::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or(String::from("<stdin>"));

    let gz = open_nbt_file(&path)?;
    let mut input = DataInput::new(gz);

    let nbt = NBT::read_from(&mut input)?;

    println!("{:#?}", nbt);
    Ok(())
}
