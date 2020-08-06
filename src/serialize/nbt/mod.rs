use std::collections::HashMap;
use std::io::{self, Read, Write};

use crate::serialize::{DataInput, DataOutput};

#[derive(Clone, Debug)]
pub enum Tag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Tag>),
    Compound(HashMap<String, Tag>),
}

#[derive(Clone, Debug)]
pub struct NBT {
    key: String,
    tag: Tag,
}

impl From<Tag> for NBT {
    fn from(tag: Tag) -> NBT {
        NBT { key: String::new(), tag }
    }
}

impl Tag {
    pub fn kind(&self) -> i8 {
        match self {
            Tag::End => 0x00,
            Tag::Byte(_) => 0x01,
            Tag::Short(_) => 0x02,
            Tag::Int(_) => 0x03,
            Tag::Long(_) => 0x04,
            Tag::Float(_) => 0x05,
            Tag::Double(_) => 0x06,
            Tag::ByteArray(_) => 0x07,
            Tag::String(_) => 0x08,
            Tag::List(_) => 0x09,
            Tag::Compound(_) => 0x0a,
        }
    }
}

impl NBT {
    pub fn new<T: AsRef<str>>(key: T, tag: Tag) -> NBT {
        let key = key.as_ref().into();
        NBT { key, tag }
    }

    pub fn key(&self) -> &str {
        self.key.as_ref()
    }

    pub fn tag(&self) -> &Tag {
        &self.tag
    }
}

// reading NBT
impl Tag {
    fn read_from<R: Read>(input: &mut DataInput<R>, tag_kind: i8) -> io::Result<Tag> {
        match tag_kind {
            0x00 => Ok(Tag::End),
            0x01 => Ok(Tag::Byte(input.read_byte()?)),
            0x02 => Ok(Tag::Short(input.read_short()?)),
            0x03 => Ok(Tag::Int(input.read_int()?)),
            0x04 => Ok(Tag::Long(input.read_long()?)),
            0x05 => Ok(Tag::Float(input.read_float()?)),
            0x06 => Ok(Tag::Double(input.read_double()?)),
            0x07 => {
                let size = input.read_int()?;
                Ok(Tag::ByteArray(input.read_bytes(size as usize)?))
            },
            0x08 => Ok(Tag::String(input.read_utf()?)),
            0x09 => {
                // kind of tag
                let tag_kind = input.read_byte()?;

                // read list of tags
                let size = input.read_int()? as usize;
                let mut list = Vec::with_capacity(size);

                for _ in 0..size {
                    list.push(Tag::read_from(input, tag_kind)?);
                }

                Ok(Tag::List(list))
            },
            0x0a => {
                let mut m = HashMap::new();
                loop {
                    let tag_kind = input.read_byte()?;

                    if tag_kind == 0 {
                        break Ok(Tag::Compound(m));
                    }

                    // read tag key
                    let key = input.read_utf()?;

                    // decode tag
                    let tag = Tag::read_from(input, tag_kind)?;

                    m.insert(key, tag);
                }
            },
            _ => Err(io::Error::new(io::ErrorKind::Other, "invalid tag kind")),
        }
    }
}

impl NBT {
    pub fn read_from<R: Read>(input: &mut DataInput<R>) -> io::Result<NBT> {
        let tag_kind = input.read_byte()?;

        if tag_kind != 10 {
            return Err(io::Error::new(io::ErrorKind::Other, "invalid tag kind, expected compound"))
        }

        // read tag key
        let key = input.read_utf()?;

        // decode tag
        let tag = Tag::read_from(input, 10)?;

        Ok(NBT { key, tag })
    }
}

impl Tag {
    fn write_to<W: Write>(&self, output: &mut DataOutput<W>) -> io::Result<()> {
        match self {
            Tag::End => Ok(()),
            Tag::Byte(x) => output.write_byte(*x),
            Tag::Short(x) => output.write_short(*x),
            Tag::Int(x) => output.write_int(*x),
            Tag::Long(x) => output.write_long(*x),
            Tag::Float(x) => output.write_float(*x),
            Tag::Double(x) => output.write_double(*x),
            Tag::ByteArray(xs) => {
                output.write_int((xs.len()&0x7fff_ffff) as i32)?;
                output.write_bytes(xs)
            },
            Tag::String(s) => output.write_utf(s),
            Tag::List(xs) => {
                let tag_kind = xs
                    .get(0)
                    .map(Tag::kind)
                    .unwrap_or(1);

                // write tag kind and length of the list
                output.write_byte(tag_kind)?;
                output.write_int((xs.len()&0x7fff_ffff) as i32)?;

                // write tag contents
                for tag in xs.iter() {
                    tag.write_to(output)?;
                }

                Ok(())
            },
            Tag::Compound(m) => {
                for (key, tag) in m.iter() {
                    let tag_kind = tag.kind();

                    // write the tag byte
                    output.write_byte(tag_kind)?;

                    // write the key
                    output.write_utf(key)?;

                    // write the tag itself
                    tag.write_to(output)?;

                    // end
                    output.write_byte(0)?;
                }
                Ok(())
            },
        }
    }
}

impl NBT {
    pub fn write_to<W: Write>(&self, output: &mut DataOutput<W>) -> io::Result<()> {
        // write the tag byte
        output.write_byte(10)?;

        // write the key
        output.write_utf(&self.key)?;

        // write the tag itself
        output.write_byte(self.tag.kind())?;
        self.tag.write_to(output)?;

        // end
        output.write_byte(0)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::default::Default;
    use std::collections::HashMap;
    use std::io::{self, BufReader, BufWriter};

    use flate2::bufread::GzDecoder;
    use flate2::write::GzEncoder;

    use crate::serialize::nbt::{NBT, Tag};
    use crate::serialize::{DataInput, DataOutput};

    fn open_nbt_file(path: &str) -> io::Result<GzDecoder<BufReader<File>>> {
        let f = File::open(path)?;
        let r = BufReader::new(f);
        Ok(GzDecoder::new(r))
    }

    fn create_nbt_file(path: &str) -> io::Result<GzEncoder<BufWriter<File>>> {
        let f = File::create(path)?;
        let w = BufWriter::new(f);
        Ok(GzEncoder::new(w, Default::default()))
    }

    #[test]
    fn test_nbt_read() {
        let gz = open_nbt_file("res/player.dat").unwrap();
        let mut input = DataInput::new(gz);

        let nbt = NBT::read_from(&mut input).unwrap();
        
        match nbt.tag() {
            Tag::Compound(_) => (),
            _ => panic!("not a coumpound tag"),
        }
    }

    #[test]
    fn test_nbt_write() {
        // write nbt file
        let gz = create_nbt_file("res/test.dat").unwrap();
        let mut output = DataOutput::new(gz);

        let ints = (0..=100)
            .map(|x| match x {
                _ if x%3 == 0 => ("Fizz", x),
                _ if x%5 == 0 => ("Buzz", x),
                _ => ("None", x),
            })
            .map(|(key, x)| {
                let mut m = HashMap::new();
                m.insert(key.into(), Tag::Int(x));
                Tag::Compound(m)
            })
            .collect();

        let tag = Tag::List(ints);
        let nbt = NBT::new("FizzBuzz", tag);

        nbt.write_to(&mut output).unwrap();

        // read the written file
        let gz = open_nbt_file("res/test.dat").unwrap();
        let mut input = DataInput::new(gz);

        let nbt = NBT::read_from(&mut input).unwrap();
        
        match nbt.tag() {
            Tag::List(_) => (),
            _ => panic!("not a coumpound tag"),
        }
    }
}
