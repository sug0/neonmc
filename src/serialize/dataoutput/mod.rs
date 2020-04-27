use std::io::{self, Write};

// Write Java-like encoded values.
pub struct DataOutput<W> {
    w: W,
}

impl<W> DataOutput<W> {
    pub fn new(w: W) -> Self {
        Self { w }
    }

    pub fn into_inner(self) -> W {
        self.w
    }
}

impl<W: Write> DataOutput<W> {
    pub fn write_utf<T: AsRef<str>>(&mut self, s: T) -> io::Result<()> {
        // get the str
        let s = s.as_ref();
        let size = (s.len()&0x7fff) as i16;

        // write the size to the underlying output
        self.write_short(size)?;

        if size == 0 {
            return Ok(())
        }

        // write the string bytes
        self.w.write_all(s.as_ref())
    }

    pub fn write_bytes<T: AsRef<[i8]>>(&mut self, buf: T) -> io::Result<()> {
        let buf: &[u8] = unsafe { std::mem::transmute(buf.as_ref()) };
        self.w.write_all(buf)
    }

    pub fn write_byte(&mut self, x: i8) -> io::Result<()> {
        let buf = x.to_be_bytes();
        self.w.write_all(&buf[..])
    }

    pub fn write_short(&mut self, x: i16) -> io::Result<()> {
        let buf = x.to_be_bytes();
        self.w.write_all(&buf[..])
    }

    pub fn write_int(&mut self, x: i32) -> io::Result<()> {
        let buf = x.to_be_bytes();
        self.w.write_all(&buf[..])
    }

    pub fn write_long(&mut self, x: i64) -> io::Result<()> {
        let buf = x.to_be_bytes();
        self.w.write_all(&buf[..])
    }

    pub fn write_float(&mut self, x: f32) -> io::Result<()> {
        let buf = x.to_be_bytes();
        self.w.write_all(&buf[..])
    }

    pub fn write_double(&mut self, x: f64) -> io::Result<()> {
        let buf = x.to_be_bytes();
        self.w.write_all(&buf[..])
    }
}
