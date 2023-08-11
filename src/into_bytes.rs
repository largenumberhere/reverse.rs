use byteorder::WriteBytesExt;
use std::io::Write;

type WavEndian = byteorder::LittleEndian;

// Another convent wrapper for byteorder's methods that allows a
//  single line to do all the heavy lifting in the BytesToStruct derive macro.
// Type-inference is a wonderful thing!
pub trait IntoBytes<TEndian> {
    fn into_bytes(self, buffer: &mut Vec<u8>) -> Result<(), std::io::Error>;
}

impl IntoBytes<WavEndian> for u8 {
    fn into_bytes(self, buffer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buffer.push(self);
        Ok(())
    }
}

impl IntoBytes<WavEndian> for u32 {
    fn into_bytes(self, buffer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buffer.write_u32::<WavEndian>(self)
    }
}

impl IntoBytes<WavEndian> for u16 {
    fn into_bytes(self, buffer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buffer.write_u16::<WavEndian>(self)
    }
}

impl<const N: usize> IntoBytes<WavEndian> for [u8; N] {
    fn into_bytes(self, buffer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let written_count = buffer.write(&self)?; //discard the bytes written count
        if written_count!= N {
            panic!("Failed to write {N} bytes to buffer! Only wrote {written_count}");
        }
        Ok(())
    }
}
