#![feature(core, io)]
use std::num::Int;
use std::io;
use std::ptr;



/// Read extension for endianness.
pub trait ReadEndian {

    // Read big-endian.
    fn read_be(&self, buf: &mut [u8]) -> io::Result<usize>;

    // Read little-endian.
    fn read_le(&self, buf: &mut [u8]) -> io::Result<usize>;
}

/// Write extension for endianness.
pub trait WriteEndian {
    
    // Write big-endian.
    fn write_be(&mut self, buf: &[u8]) -> io::Result<usize>;
    
    // Write little-endian.
    fn write_le(&mut self, buf: &[u8]) -> io::Result<usize>;
}




impl ReadEndian for u32 {
    fn read_be(&self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() != 4 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "output buffer length must be exactly 4 bytes",
                None));
        }
        let word = self.to_be();
        unsafe {
            ptr::copy_nonoverlapping_memory(
                buf.get_unchecked_mut(0),
                &word as *const _ as *const u8, 4);
        }
        Ok(4)
    }
    fn read_le(&self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() != 4 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "output buffer length must be exactly 4 bytes",
                None));
        }
        let word = self.to_le();
        unsafe {
            ptr::copy_nonoverlapping_memory(
                buf.get_unchecked_mut(0),
                &word as *const _ as *const u8, 4);
        }
        Ok(4)
    }
}

impl<'a> ReadEndian for &'a [u32] {
    fn read_be(&self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() % 4 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "output buffer length must be a multiple of 4 bytes",
                None));
        }
        if buf.len() != self.len()*4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input buffer must have exactly the same length as the output buffer",
                None));
        }
        unsafe {
            let mut bytes: *mut u8 = buf.get_unchecked_mut(0);
            let mut words: *const u32 = self.get_unchecked(0);
            for _ in 0..self.len() {
                let word = (*words).to_be();
                ptr::copy_nonoverlapping_memory(bytes, &word as *const _ as *const u8, 4);
                bytes = bytes.offset(4);
                words = words.offset(1);
            }
        }
        Ok(buf.len())
    }
    fn read_le(&self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() % 4 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "output buffer length must be a multiple of 4 bytes",
                None));
        }
        if buf.len() != self.len()*4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input buffer must have exactly the same length as the output buffer",
                None));
        }
        unsafe {
            let mut bytes: *mut u8 = buf.get_unchecked_mut(0);
            let mut words: *const u32 = self.get_unchecked(0);
            for _ in 0..self.len() {
                let word = (*words).to_le();
                ptr::copy_nonoverlapping_memory(bytes, &word as *const _ as *const u8, 4);
                bytes = bytes.offset(4);
                words = words.offset(1);
            }
        }
        Ok(buf.len())
    }
}

impl ReadEndian for Vec<u32> {
    fn read_be(&self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() % 4 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "output buffer length must be a multiple of 4 bytes",
                None));
        }
        if buf.len() != self.len()*4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input buffer must have exactly the same length as the output buffer",
                None));
        }
        for (word, mut bytes) in self.iter().zip(buf.chunks_mut(4)) {
            try!(word.to_le().read_be(bytes));
        }
        Ok(buf.len())
    }
    fn read_le(&self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() % 4 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "output buffer length must be a multiple of 4 bytes",
                None));
        }
        if buf.len() != self.len()*4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input buffer must have exactly the same length as the output buffer",
                None));
        }
        for (word, mut bytes) in self.iter().zip(buf.chunks_mut(4)) {
            try!(word.to_le().read_le(bytes));
        }
        Ok(buf.len())
    }
}

impl<'a> WriteEndian for u32 {
    fn write_be(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() != 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input buffer length must be exactly 4 bytes",
                None));
        }
        unsafe {
            ptr::copy_nonoverlapping_memory(
                self as *mut _ as *mut u8,
                buf.get_unchecked(0), 4);
        }
        *self = Int::from_be(*self);
        Ok(4)
    }
    fn write_le(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() != 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input buffer length must be exactly 4 bytes",
                None));
        }
        unsafe {
            ptr::copy_nonoverlapping_memory(
                self as *mut _ as *mut u8,
                buf.get_unchecked(0), 4);
        }
        *self = Int::from_le(*self);
        Ok(4)
    }
}

impl<'a> WriteEndian for &'a mut [u32] {
    fn write_be(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() % 4 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input buffer length must be a multiple of 4 bytes",
                None));
        }
        if buf.len() != self.len()*4 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "output buffer must have exactly the same length as the input buffer",
                None));
        }
        for (word, bytes) in self.iter_mut().zip(buf.chunks(4)) {
            try!(word.write_be(bytes));
        }
        Ok(buf.len())
    }
    fn write_le(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() % 4 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input buffer length must be a multiple of 4 bytes",
                None));
        }
        if buf.len() != self.len()*4 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "output buffer must have exactly the same length as the input buffer",
                None));
        }
        for (word, bytes) in self.iter_mut().zip(buf.chunks(4)) {
            try!(word.write_le(bytes));
        }
        Ok(buf.len())
    }
}

impl WriteEndian for Vec<u32> {
    fn write_be(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() % 4 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input buffer length must be a multiple of 4 bytes",
                None));
        }
        if self.len() != 0 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "output buffer must be empty",
                None));
        }
        for bytes in buf.chunks(4) {
            let mut word = 0u32;
            try!(word.write_be(bytes));
            self.push(word);
        }
        Ok(buf.len())
    }
    fn write_le(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() % 4 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input buffer length must be a multiple of 4 bytes",
                None));
        }
        if self.len() != 0 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "output buffer must be empty",
                None));
        }
        for bytes in buf.chunks(4) {
            let mut word = 0u32;
            try!(word.write_le(bytes));
            self.push(word);
        }
        Ok(buf.len())
    }
}




#[cfg(test)]
mod tests {
    use super::{ReadEndian, WriteEndian};

    #[test]
    fn read_be_u32() {
        let mut to = [0u8; 4];
        let from: u32 = 0x04030201;

        assert_eq!(from.read_be(&mut to[]).unwrap(), 4);
        for i in 0..4 {
            assert_eq!(to[i], (4 - i) as u8);
        }
    }
    
    #[test]
    fn read_le_u32() {
        let mut to = [0u8; 4];
        let from: u32 = 0x04030201;
        
        assert_eq!(from.read_le(&mut to[]).unwrap(), 4);
        for i in 0..4 {
            assert_eq!(to[i], (i + 1) as u8);
        }
    }

    #[test]
    fn read_be_mut_u32() {
        let mut to = [0u8; 8];
        let from = [0x01020304,
                    0x05060708];

        assert_eq!((&from[]).read_be(&mut to[]).unwrap(), 8);
        
        for i in 0..4 {
            assert_eq!(to[i], (i + 1) as u8);
        }
    }
    
    #[test]
    fn read_le_mut_u32() {
        let mut to = [0u8; 8];
        let from = [0x04030201,
                    0x08070605];
        
        assert_eq!((&from[]).read_le(&mut to[]).unwrap(), 8);
        
        for i in 0..4 {
            assert_eq!(to[i], (i + 1) as u8);
        }
    }

    #[test]
    fn read_be_vec_u32() {
        let mut to = [0u8; 8];
        let from = vec![0x01020304,
                        0x05060708];

        assert_eq!(from.read_be(&mut to[]).unwrap(), 8);
        
        for i in 0..4 {
            assert_eq!(to[i], (i + 1) as u8);
        }
    }
    
    #[test]
    fn read_le_vec_u32() {
        let mut to = [0u8; 8];
        let from = vec![0x04030201,
                        0x08070605];
        
        assert_eq!(from.read_le(&mut to[]).unwrap(), 8);
        
        for i in 0..4 {
            assert_eq!(to[i], (i + 1) as u8);
        }
    }

    #[test]
    fn write_be_u32() {
        let mut to: u32 = 0;
        let from = [0x01u8, 0x02u8, 0x03u8, 0x04u8];

        assert_eq!(to.write_be(&from[]).unwrap(), 4);
        
        assert_eq!(format!("{:08x}", to), "01020304");
    }

    #[test]
    fn write_le_u32() {
        let mut to: u32 = 0;
        let from = [0x01u8, 0x02u8, 0x03u8, 0x04u8];

        assert_eq!(to.write_le(&from[]).unwrap(), 4);
        
        assert_eq!(format!("{:08x}", to), "04030201");
    }

    #[test]
    fn write_be_mut_u32() {
        let mut to = [0u32; 2];
        let from = [0x01u8, 0x02u8, 0x03u8, 0x04u8,
                    0x05u8, 0x06u8, 0x07u8, 0x08u8];

        assert_eq!((&mut to[]).write_be(&from[]).unwrap(), 8);
        
        assert_eq!(format!("{:08x}", to[0]), "01020304");
        assert_eq!(format!("{:08x}", to[1]), "05060708");
    }

    #[test]
    fn write_le_mut_u32() {
        let mut to = [0u32; 2];
        let from = [0x01u8, 0x02u8, 0x03u8, 0x04u8,
                    0x05u8, 0x06u8, 0x07u8, 0x08u8];

        assert_eq!((&mut to[]).write_le(&from[]).unwrap(), 8);
        
        assert_eq!(format!("{:08x}", to[0]), "04030201");
        assert_eq!(format!("{:08x}", to[1]), "08070605");
    }

    #[test]
    fn write_be_vec_u32() {
        let mut to: Vec<u32> = vec![];
        let from = [0x01u8, 0x02u8, 0x03u8, 0x04u8,
                    0x05u8, 0x06u8, 0x07u8, 0x08u8];

        assert_eq!(to.write_be(&from[]).unwrap(), 8);
        
        assert_eq!(format!("{:08x}", to[0]), "01020304");
        assert_eq!(format!("{:08x}", to[1]), "05060708");
    }
    
    #[test]
    fn write_le_vec_u32() {
        let mut to: Vec<u32> = vec![];
        let from = [0x01u8, 0x02u8, 0x03u8, 0x04u8,
                    0x05u8, 0x06u8, 0x07u8, 0x08u8];

        assert_eq!(to.write_le(&from[]).unwrap(), 8);
        
        assert_eq!(format!("{:08x}", to[0]), "04030201");
        assert_eq!(format!("{:08x}", to[1]), "08070605");
    }
}
