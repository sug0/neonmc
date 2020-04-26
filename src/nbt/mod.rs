use std::io::{self, Read};
use std::collections::HashMap;

use crate::serialize::DataInput;

#[derive(Clone, Debug)]
enum Tag {
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
    Coumpound(HashMap<String, Tag>),
}

#[derive(Clone, Debug)]
pub struct NBT {
    key: String,
    tag: Tag,
}

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
                    let nbt = NBT::read_from(input)?;

                    match &nbt.tag {
                        &Tag::End => break Ok(Tag::Coumpound(m)),
                        _ => {
                            m.insert(nbt.key, nbt.tag);
                        },
                    }
                }
            },
            _ => Err(io::Error::new(io::ErrorKind::Other, "invalid tag type")),
        }
    }
}

impl NBT {
    pub fn read_from<R: Read>(input: &mut DataInput<R>) -> io::Result<NBT> {
        // read tag kind
        let tag_kind = input.read_byte()?;

        if tag_kind == 0 {
            return Ok(NBT { key: String::new(), tag: Tag::End })
        }

        // read tag key
        let key = input.read_utf()?;

        // decode tag
        let tag = Tag::read_from(input, tag_kind)?;

        Ok(NBT { key, tag })
    }
}
