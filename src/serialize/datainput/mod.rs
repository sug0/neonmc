use std::io::{self, Read};
use std::mem::ManuallyDrop;

// Read Java-like encoded values.
pub struct DataInput<R> {
    r: R,
}

impl<R> DataInput<R> {
    pub fn new(r: R) -> Self {
        Self { r }
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
        let mut buf = [0; 1];
        self.r.read_exact(&mut buf[..])?;
        Ok(unsafe { std::mem::transmute(buf[0]) })
    }

    pub fn read_short(&mut self) -> io::Result<i16> {
        let mut buf = [0; 2];
        self.r.read_exact(&mut buf[..])?;
        Ok(i16::from_be_bytes([buf[0], buf[1]]))
    }

    pub fn read_int(&mut self) -> io::Result<i32> {
        let mut buf = [0; 4];
        self.r.read_exact(&mut buf[..])?;
        Ok(i32::from_be_bytes([
            buf[0], buf[1], buf[2], buf[3],
        ]))
    }

    pub fn read_long(&mut self) -> io::Result<i64> {
        let mut buf = [0; 8];
        self.r.read_exact(&mut buf[..])?;
        Ok(i64::from_be_bytes([
            buf[0], buf[1], buf[2], buf[3],
            buf[4], buf[5], buf[6], buf[7],
        ]))
    }

    pub fn read_float(&mut self) -> io::Result<f32> {
        let mut buf = [0; 4];
        self.r.read_exact(&mut buf[..])?;
        Ok(f32::from_be_bytes([
            buf[0], buf[1], buf[2], buf[3],
        ]))
    }

    pub fn read_double(&mut self) -> io::Result<f64> {
        let mut buf = [0; 8];
        self.r.read_exact(&mut buf[..])?;
        Ok(f64::from_be_bytes([
            buf[0], buf[1], buf[2], buf[3],
            buf[4], buf[5], buf[6], buf[7],
        ]))
    }
}
