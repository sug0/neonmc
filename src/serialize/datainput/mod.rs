use std::io::{self, Read};
use std::mem::ManuallyDrop;

// Read Java-like encoded values.
pub struct DataInput<R> {
    buf: [u8; 8],
    r: R,
}

impl<R> DataInput<R> {
    pub fn new(r: R) -> Self {
        Self { buf: [0; 8], r }
    }

    pub fn into_inner(self) -> R {
        self.r
    }
}

impl<R: Read> DataInput<R> {
    pub fn read_utf(&mut self) -> io::Result<String> {
        // read the size
        let size = self.read_short()? as usize;

        if size == 0 {
            return Ok(String::new())
        }

        // never call drop
        let mut buf = ManuallyDrop::new(vec![0; size]);
        self.r.read_exact(&mut buf[..])?;

        Ok(unsafe { String::from_raw_parts(buf.as_mut_ptr(), size, buf.capacity()) })
    }

    pub fn read_bytes(&mut self, count: usize) -> io::Result<Vec<i8>> {
        let mut buf = vec![0; count];
        self.r.read_exact(&mut buf[..])?;
        Ok(unsafe { std::mem::transmute(buf) })
    }

    pub fn read_byte(&mut self) -> io::Result<i8> {
        self.r.read_exact(&mut self.buf[..1])?;
        Ok(unsafe { std::mem::transmute(self.buf[0]) })
    }

    pub fn read_short(&mut self) -> io::Result<i16> {
        self.r.read_exact(&mut self.buf[..2])?;
        Ok(i16::from_be_bytes([self.buf[0], self.buf[1]]))
    }

    pub fn read_int(&mut self) -> io::Result<i32> {
        self.r.read_exact(&mut self.buf[..4])?;
        Ok(i32::from_be_bytes([
            self.buf[0], self.buf[1], self.buf[2], self.buf[3],
        ]))
    }

    pub fn read_long(&mut self) -> io::Result<i64> {
        self.r.read_exact(&mut self.buf[..8])?;
        Ok(i64::from_be_bytes([
            self.buf[0], self.buf[1], self.buf[2], self.buf[3],
            self.buf[4], self.buf[5], self.buf[6], self.buf[7],
        ]))
    }

    pub fn read_float(&mut self) -> io::Result<f32> {
        self.r.read_exact(&mut self.buf[..4])?;
        Ok(f32::from_be_bytes([
            self.buf[0], self.buf[1], self.buf[2], self.buf[3],
        ]))
    }

    pub fn read_double(&mut self) -> io::Result<f64> {
        self.r.read_exact(&mut self.buf[..8])?;
        Ok(f64::from_be_bytes([
            self.buf[0], self.buf[1], self.buf[2], self.buf[3],
            self.buf[4], self.buf[5], self.buf[6], self.buf[7],
        ]))
    }
}
