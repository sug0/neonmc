mod serialize;

use std::fs::File;
use std::io::{self, BufReader};
use flate2::bufread::GzDecoder;

use serialize::DataInput;
use serialize::nbt::NBT;

fn open_nbt_file(path: &str) -> io::Result<GzDecoder<BufReader<File>>> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    Ok(GzDecoder::new(r))
}

fn main() -> io::Result<()> {
    let gz = open_nbt_file("res/player.dat")?;
    let mut input = DataInput::new(gz);

    let nbt = NBT::read_from(&mut input)?;

    println!("{:#?}", nbt);
    Ok(())
}
