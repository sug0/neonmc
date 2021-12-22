use std::io::{self, Write};

/// Write Java-like encoded values.
pub struct DataOutput<W> {
    w: W,
}

impl<W> DataOutput<W> {
    /// Creates a new writer for Java-like encoded values.
    pub fn new(w: W) -> Self {
        Self { w }
    }

    /// Returns the writer associated with this [`DataOutput`].
    pub fn into_inner(self) -> W {
        self.w
    }
}

impl<W: Write> DataOutput<W> {
    /// Write a [UTF encoded](https://en.wikipedia.org/wiki/UTF-8#Modified_UTF-8) string.
    pub fn write_utf<T: AsRef<str>>(&mut self, s: T) -> io::Result<()> {
        // get the str
        let s = s.as_ref();
        let slice: &[u8] = s.as_ref();
        let slice: &[u8] = &slice[..slice.len() & 0x7fff];

        // string size overflows i16
        if slice.len() < s.len() {
            return Err(io::Error::new(io::ErrorKind::Other, "string size overflows i16"));
        }

        let size = slice.len() as i16;

        // write the size to the underlying output
        self.write_short(size)?;

        if size == 0 {
            return Ok(())
        }

        // write the string bytes
        self.w.write_all(slice)
    }

    /// Write all the bytes in `buf` to the internal writer.
    pub fn write_bytes<T: AsRef<[i8]>>(&mut self, buf: T) -> io::Result<()> {
        let buf: &[u8] = unsafe { std::mem::transmute(buf.as_ref()) };
        self.w.write_all(buf)
    }

    /// Write a single byte to the internal writer.
    pub fn write_byte(&mut self, x: i8) -> io::Result<()> {
        let buf = x.to_be_bytes();
        self.w.write_all(&buf[..])
    }

    /// Write a short value to the internal writer.
    pub fn write_short(&mut self, x: i16) -> io::Result<()> {
        let buf = x.to_be_bytes();
        self.w.write_all(&buf[..])
    }

    /// Write a word to the internal writer.
    pub fn write_int(&mut self, x: i32) -> io::Result<()> {
        let buf = x.to_be_bytes();
        self.w.write_all(&buf[..])
    }

    /// Write a long value to the internal writer.
    pub fn write_long(&mut self, x: i64) -> io::Result<()> {
        let buf = x.to_be_bytes();
        self.w.write_all(&buf[..])
    }

    /// Write an [`f32`] to the internal writer.
    pub fn write_float(&mut self, x: f32) -> io::Result<()> {
        let buf = x.to_be_bytes();
        self.w.write_all(&buf[..])
    }

    /// Write an [`f64`] to the internal writer.
    pub fn write_double(&mut self, x: f64) -> io::Result<()> {
        let buf = x.to_be_bytes();
        self.w.write_all(&buf[..])
    }
}
