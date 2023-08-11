use std::io::Write;
use byteorder::WriteBytesExt;

type WavEndian = byteorder::LittleEndian;

pub trait IntoBytes<TEndian>{
    fn into_bytes(&self, buffer: &mut Vec<u8>);
}

impl IntoBytes<WavEndian> for u8{
    fn into_bytes(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.clone())
    }
}

impl IntoBytes<WavEndian> for u32{
    fn into_bytes(&self, buffer: &mut Vec<u8>) {
        buffer.write_u32::<WavEndian>(self.clone()).unwrap()
    }
}

impl IntoBytes<WavEndian> for u16{
    fn into_bytes(&self, buffer: &mut Vec<u8>) {
        buffer.write_u16::<WavEndian>(self.clone()).unwrap()
    }
}

impl<const N: usize> IntoBytes<WavEndian> for [u8; N]{
    fn into_bytes(&self, buffer: &mut Vec<u8>) {
        buffer.write(self).unwrap();
    }
}








