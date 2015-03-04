# rust-endian
Rust I/O Endianness

At the time of this writing, there are 3 Rust libraries for endianness:

* **endian** -- this library
* **byteorder**
* **bswap**

## libendian

This library adds 4 methods to integers:

	fn read_be(&self, buf: &mut [u8]) -> Result<usize>;
	fn read_le(&self, buf: &mut [u8]) -> Result<usize>;
	fn write_be(&mut self, buf: &[u8]) -> Result<usize>;
	fn write_le(&mut self, buf: &[u8]) -> Result<usize>;

## libbyteorder

That other library adds several methods to existing readers and writers:

	fn read_u8(&mut self) -> Result<u8>;
	fn read_i8(&mut self) -> Result<i8>;
	fn read_u16<T: ByteOrder>(&mut self) -> Result<u16>;
	fn read_i16<T: ByteOrder>(&mut self) -> Result<i16>;
	fn read_u32<T: ByteOrder>(&mut self) -> Result<u32>;
	fn read_i32<T: ByteOrder>(&mut self) -> Result<i32>;
	fn read_u64<T: ByteOrder>(&mut self) -> Result<u64>;
	fn read_i64<T: ByteOrder>(&mut self) -> Result<i64>;
    fn write_u8(&mut self, n: u8) -> Result<()>;
    fn write_i8(&mut self, n: i8) -> Result<()>;
    fn write_u16<T: ByteOrder>(&mut self, n: u16) -> Result<()>;
    fn write_i16<T: ByteOrder>(&mut self, n: i16) -> Result<()>;
    fn write_u32<T: ByteOrder>(&mut self, n: u32) -> Result<()>;
    fn write_i32<T: ByteOrder>(&mut self, n: i32) -> Result<()>;
    fn write_u64<T: ByteOrder>(&mut self, n: u64) -> Result<()>;
    fn write_i64<T: ByteOrder>(&mut self, n: i64) -> Result<()>;

# Conclusion

It looks like `libbyteorder` might be the better choice, but that is
yet to be determined, until then, I think that the `libendian` might
have some valid use cases that the other does not provide for.