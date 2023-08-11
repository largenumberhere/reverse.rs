use std::io::Write;
use byteorder::WriteBytesExt;

type WavEndian = byteorder::LittleEndian;

pub trait IntoBytes<TEndian>{
    fn into_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), std::io::Error>;
}

impl IntoBytes<WavEndian> for u8{
    fn into_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buffer.push(self.clone());
        Ok(())
    }
}

impl IntoBytes<WavEndian> for u32{
    fn into_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buffer.write_u32::<WavEndian>(self.clone())
    }
}

impl IntoBytes<WavEndian> for u16{
    fn into_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buffer.write_u16::<WavEndian>(self.clone())
    }
}

impl<const N: usize> IntoBytes<WavEndian> for [u8; N]{
    fn into_bytes(&self, buffer: &mut Vec<u8>) -> Result<(),std::io::Error> {
        let _ = buffer.write(self)?; //discard the bytes written count
        Ok(())
    }
}








