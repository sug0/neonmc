use std::io::{self, Read};
use std::mem::ManuallyDrop;
use std::collections::HashMap;

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

fn read_utf<R: Read>(r: &mut R) -> io::Result<String> {
    // read the size
    let mut size = [0; 2];
    r.read_exact(&mut size[..])?;
    let size = i16::from_be_bytes([size[0], size[1]]) as usize;

    if size == 0 {
        return Ok(String::new())
    }

    // never call drop
    let mut buf = ManuallyDrop::new(vec![0; size]);
    r.read_exact(&mut buf[..])?;

    Ok(unsafe { String::from_raw_parts(buf.as_mut_ptr(), size, buf.capacity()) })
}

impl Tag {
    fn read_from<R: Read>(r: &mut R, tag_kind: u8) -> io::Result<Tag> {
        let mut buf = [0; 8];

        match tag_kind {
            0x00 => Ok(Tag::End),
            0x01 => {
                r.read_exact(&mut buf[..1])?;
                let payload = i8::from_be_bytes([buf[0]]);
                Ok(Tag::Byte(payload))
            },
            0x02 => {
                r.read_exact(&mut buf[..2])?;
                let payload = i16::from_be_bytes([buf[0], buf[1]]);
                Ok(Tag::Short(payload))
            },
            0x03 => {
                r.read_exact(&mut buf[..4])?;
                let payload = i32::from_be_bytes([
                    buf[0], buf[1], buf[2], buf[3],
                ]);
                Ok(Tag::Int(payload))
            },
            0x04 => {
                r.read_exact(&mut buf[..8])?;
                let payload = i64::from_be_bytes([
                    buf[0], buf[1], buf[2], buf[3],
                    buf[4], buf[5], buf[6], buf[7],
                ]);
                Ok(Tag::Long(payload))
            },
            0x05 => {
                r.read_exact(&mut buf[..4])?;
                let payload = f32::from_be_bytes([
                    buf[0], buf[1], buf[2], buf[3],
                ]);
                Ok(Tag::Float(payload))
            },
            0x06 => {
                r.read_exact(&mut buf[..8])?;
                let payload = f64::from_be_bytes([
                    buf[0], buf[1], buf[2], buf[3],
                    buf[4], buf[5], buf[6], buf[7],
                ]);
                Ok(Tag::Double(payload))
            },
            0x07 => {
                r.read_exact(&mut buf[..4])?;
                let size = i32::from_be_bytes([
                    buf[0], buf[1], buf[2], buf[3],
                ]) as usize;
                let mut buf = vec![0; size];
                r.read_exact(&mut buf[..])?;
                Ok(Tag::ByteArray(unsafe { std::mem::transmute(buf) }))
            },
            0x08 => Ok(Tag::String(read_utf(r)?)),
            0x09 => {
                // kind of tag
                r.read_exact(&mut buf[..1])?;
                let tag_kind = buf[0];

                // size of list
                r.read_exact(&mut buf[..4])?;
                let size = i32::from_be_bytes([
                    buf[0], buf[1], buf[2], buf[3],
                ]) as usize;

                let mut list = Vec::with_capacity(size);

                for _ in 0..size {
                    list.push(Tag::read_from(r, tag_kind)?);
                }

                Ok(Tag::List(list))
            },
            0x0a => {
                let mut m = HashMap::new();
                loop {
                    let nbt = NBT::read_from(r)?;

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
    pub fn read_from<R: Read>(r: &mut R) -> io::Result<NBT> {
        let mut buf = [0; 8];

        // read tag kind
        r.read_exact(&mut buf[..1])?;
        let tag_kind = buf[0];

        if tag_kind == 0 {
            return Ok(NBT { key: String::new(), tag: Tag::End })
        }

        // read tag key
        let key = read_utf(r)?;

        // decode tag
        let tag = Tag::read_from(r, tag_kind)?;

        Ok(NBT { key, tag })
    }
}
