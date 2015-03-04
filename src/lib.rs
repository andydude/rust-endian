#![feature(io)]
extern crate bswap;
use std::io;

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


macro_rules! endian_impls {
    ($ty:ident, $bety:ident, $lety:ident, $nbytes:expr, $sbytes:expr) => {

        impl ReadEndian for $ty {
            fn read_be(&self, buf: &mut [u8]) -> io::Result<usize> {
                if buf.len() != $nbytes {
                    return Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            concat!("output buffer length must be exactly ", $sbytes, " bytes"),
                            None));
                }
                bswap::$bety::encode(buf, *self);
                Ok($nbytes)
            }
            fn read_le(&self, buf: &mut [u8]) -> io::Result<usize> {
                if buf.len() != $nbytes {
                    return Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            concat!("output buffer length must be exactly ", $sbytes, " bytes"),
                            None));
                }
                bswap::$lety::encode(buf, *self);
                Ok($nbytes)
            }
        }

        impl<'a> ReadEndian for &'a [$ty] {
            fn read_be(&self, buf: &mut [u8]) -> io::Result<usize> {
                if buf.len() % $nbytes != 0 {
                    return Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            concat!("output buffer length must be a multiple of ", $sbytes, " bytes"),
                            None));
                }
                if buf.len() != self.len()*$nbytes {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "input buffer must have exactly the same length as the output buffer",
                            None));
                }
                bswap::$bety::encode_slice(buf, self);
                Ok(buf.len())
            }
            fn read_le(&self, buf: &mut [u8]) -> io::Result<usize> {
                if buf.len() % $nbytes != 0 {
                    return Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            concat!("output buffer length must be a multiple of ", $sbytes, " bytes"),
                            None));
                }
                if buf.len() != self.len()*$nbytes {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "input buffer must have exactly the same length as the output buffer",
                            None));
                }
                bswap::$lety::encode_slice(buf, self);
                Ok(buf.len())
            }
        }

        impl ReadEndian for Vec<$ty> {
            fn read_be(&self, buf: &mut [u8]) -> io::Result<usize> {
                if buf.len() % $nbytes != 0 {
                    return Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            concat!("output buffer length must be a multiple of ", $sbytes, " bytes"),
                            None));
                }
                if buf.len() != self.len()*$nbytes {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "input buffer must have exactly the same length as the output buffer",
                            None));
                }
                bswap::$bety::encode_slice(buf, &self[..]);
                Ok(buf.len())
            }
            fn read_le(&self, buf: &mut [u8]) -> io::Result<usize> {
                if buf.len() % $nbytes != 0 {
                    return Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            concat!("output buffer length must be a multiple of ", $sbytes, " bytes"),
                            None));
                }
                if buf.len() != self.len()*$nbytes {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "input buffer must have exactly the same length as the output buffer",
                            None));
                }
                bswap::$lety::encode_slice(buf, &self[..]);
                Ok(buf.len())
            }
        }

        impl WriteEndian for $ty {
            fn write_be(&mut self, buf: &[u8]) -> io::Result<usize> {
                if buf.len() != $nbytes {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            concat!("input buffer length must be exactly ", $sbytes, " bytes"),
                            None));
                }
                *self = bswap::$bety::decode(buf);
                Ok($nbytes)
            }
            fn write_le(&mut self, buf: &[u8]) -> io::Result<usize> {
                if buf.len() != $nbytes {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            concat!("input buffer length must be exactly ", $sbytes, " bytes"),
                            None));
                }
                *self = bswap::$lety::decode(buf);
                Ok($nbytes)
            }
        }

        impl<'a> WriteEndian for &'a mut [$ty] {
            fn write_be(&mut self, buf: &[u8]) -> io::Result<usize> {
                if buf.len() % $nbytes != 0 {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            concat!("input buffer length must be a multiple of ", $sbytes, " bytes"),
                            None));
                }
                if buf.len() != self.len()*$nbytes {
                    return Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            "output buffer must have exactly the same length as the input buffer",
                            None));
                }
                bswap::$bety::decode_slice(self, buf);
                Ok(buf.len())
            }
            fn write_le(&mut self, buf: &[u8]) -> io::Result<usize> {
                if buf.len() % $nbytes != 0 {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            concat!("input buffer length must be a multiple of ", $sbytes, " bytes"),
                            None));
                }
                if buf.len() != self.len()*$nbytes {
                    return Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            "output buffer must have exactly the same length as the input buffer",
                            None));
                }
                bswap::$lety::decode_slice(self, buf);
                Ok(buf.len())
            }
        }

        impl WriteEndian for Vec<$ty> {
            fn write_be(&mut self, buf: &[u8]) -> io::Result<usize> {
                if buf.len() % $nbytes != 0 {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            concat!("input buffer length must be a multiple of ", $sbytes, " bytes"),
                            None));
                }
                if self.len() != 0 {
                    return Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            "output buffer must be empty",
                            None));
                }
                self.resize(buf.len()/$nbytes, 0);
                bswap::$bety::decode_slice(&mut self[..], buf);
                Ok(buf.len())
            }
            fn write_le(&mut self, buf: &[u8]) -> io::Result<usize> {
                if buf.len() % $nbytes != 0 {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            concat!("input buffer length must be a multiple of ", $sbytes, " bytes"),
                            None));
                }
                if self.len() != 0 {
                    return Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            "output buffer must be empty",
                            None));
                }
                self.resize(buf.len()/$nbytes, 0);
                bswap::$lety::decode_slice(&mut self[..], buf);
                Ok(buf.len())
            }
        }
    }
}


endian_impls!(u16, beu16, leu16, 2, "2");
endian_impls!(u32, beu32, leu32, 4, "4");
endian_impls!(u64, beu64, leu64, 8, "8");


#[cfg(test)]
mod tests {
    use super::{ReadEndian, WriteEndian};

    #[test]
    fn read_be_u32() {
        let mut to = [0u8; 4];
        let from: u32 = 0x04030201u32;

        assert_eq!(from.read_be(&mut to[..]).unwrap(), 4);
        for i in 0..4 {
            assert_eq!(to[i], (4 - i) as u8);
        }
    }

    #[test]
    fn read_le_u32() {
        let mut to = [0u8; 4];
        let from: u32 = 0x04030201u32;

        assert_eq!(from.read_le(&mut to[..]).unwrap(), 4);
        for i in 0..4 {
            assert_eq!(to[i], (i + 1) as u8);
        }
    }

    #[test]
    fn read_be_mut_u32() {
        let mut to = [0u8; 8];
        let from = [0x01020304u32,
                    0x05060708u32];

        assert_eq!((&from[..]).read_be(&mut to[..]).unwrap(), 8);

        for i in 0..4 {
            assert_eq!(to[i], (i + 1) as u8);
        }
    }

    #[test]
    fn read_le_mut_u32() {
        let mut to = [0u8; 8];
        let from = [0x04030201u32,
                    0x08070605u32];

        assert_eq!((&from[..]).read_le(&mut to[..]).unwrap(), 8);

        for i in 0..4 {
            assert_eq!(to[i], (i + 1) as u8);
        }
    }

    #[test]
    fn read_be_vec_u32() {
        let mut to = [0u8; 8];
        let from = vec![0x01020304u32,
                        0x05060708u32];

        assert_eq!(from.read_be(&mut to[..]).unwrap(), 8);

        for i in 0..4 {
            assert_eq!(to[i], (i + 1) as u8);
        }
    }

    #[test]
    fn read_le_vec_u32() {
        let mut to = [0u8; 8];
        let from = vec![0x04030201u32,
                        0x08070605u32];

        assert_eq!(from.read_le(&mut to[..]).unwrap(), 8);

        for i in 0..4 {
            assert_eq!(to[i], (i + 1) as u8);
        }
    }

    #[test]
    fn write_be_u32() {
        let mut to: u32 = 0;
        let from = [0x01u8, 0x02u8, 0x03u8, 0x04u8];

        assert_eq!(to.write_be(&from[..]).unwrap(), 4);

        assert_eq!(format!("{:08x}", to), "01020304");
    }

    #[test]
    fn write_le_u32() {
        let mut to: u32 = 0;
        let from = [0x01u8, 0x02u8, 0x03u8, 0x04u8];

        assert_eq!(to.write_le(&from[..]).unwrap(), 4);

        assert_eq!(format!("{:08x}", to), "04030201");
    }

    #[test]
    fn write_be_mut_u32() {
        let mut to = [0u32; 2];
        let from = [0x01u8, 0x02u8, 0x03u8, 0x04u8,
                    0x05u8, 0x06u8, 0x07u8, 0x08u8];

        assert_eq!((&mut to[..]).write_be(&from[..]).unwrap(), 8);

        assert_eq!(format!("{:08x}", to[0]), "01020304");
        assert_eq!(format!("{:08x}", to[1]), "05060708");
    }

    #[test]
    fn write_le_mut_u32() {
        let mut to = [0u32; 2];
        let from = [0x01u8, 0x02u8, 0x03u8, 0x04u8,
                    0x05u8, 0x06u8, 0x07u8, 0x08u8];

        assert_eq!((&mut to[..]).write_le(&from[..]).unwrap(), 8);

        assert_eq!(format!("{:08x}", to[0]), "04030201");
        assert_eq!(format!("{:08x}", to[1]), "08070605");
    }

    #[test]
    fn write_be_vec_u32() {
        let mut to: Vec<u32> = vec![];
        let from = [0x01u8, 0x02u8, 0x03u8, 0x04u8,
                    0x05u8, 0x06u8, 0x07u8, 0x08u8];

        assert_eq!(to.write_be(&from[..]).unwrap(), 8);

        assert_eq!(format!("{:08x}", to[0]), "01020304");
        assert_eq!(format!("{:08x}", to[1]), "05060708");
    }

    #[test]
    fn write_le_vec_u32() {
        let mut to: Vec<u32> = vec![];
        let from = [0x01u8, 0x02u8, 0x03u8, 0x04u8,
                    0x05u8, 0x06u8, 0x07u8, 0x08u8];

        assert_eq!(to.write_le(&from[..]).unwrap(), 8);

        assert_eq!(format!("{:08x}", to[0]), "04030201");
        assert_eq!(format!("{:08x}", to[1]), "08070605");
    }
}
