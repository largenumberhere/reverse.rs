use std::io::Read;
use byteorder::ReadBytesExt;
use crate::wav_bytes::Header;

type WavEndian = byteorder::LittleEndian;

pub trait ReadPrimitives<T>{
    fn from_bytes(&mut self) -> T;
}

impl<const N: usize, R: Read> ReadPrimitives<[u8;N]> for R{
    fn from_bytes(&mut self) -> [u8; N] {
        let mut bytes: [u8; N]= [0; N];
        self.read(&mut bytes).unwrap();
        
        bytes
    }
}

impl<R: Read> ReadPrimitives<u8> for R{
    fn from_bytes(&mut self) -> u8 {
        self.read_u8().unwrap()
    }
}

impl<R: Read> ReadPrimitives<u32> for R{
    fn from_bytes(&mut self) -> u32 {
        self.read_u32::<WavEndian>().unwrap()
    }
}

impl<R: Read> ReadPrimitives<u16> for R {
    fn from_bytes(&mut self) -> u16 {
        self.read_u16::<WavEndian>().unwrap()
    }
}