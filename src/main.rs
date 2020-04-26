pub mod nbt;

use nbt::NBT;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let _stdin_lock = stdin.lock();
    let mut stdin_lock = BufReader::new(_stdin_lock);

    let nbt_tag = NBT::read_from(&mut stdin_lock)?;
    println!("{:#?}", nbt_tag);

    Ok(())
}
