pub mod nbt;
pub mod serialize;

use std::io::{self, BufReader};

use nbt::NBT;
use serialize::DataInput;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let _stdin_lock = stdin.lock();
    let stdin_lock = BufReader::new(_stdin_lock);

    let mut input = DataInput::new(stdin_lock);
    let nbt_tag = NBT::read_from(&mut input)?;

    println!("{:#?}", nbt_tag);
    Ok(())
}
