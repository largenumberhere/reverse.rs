use byteorder::ReadBytesExt;
use std::io::Read;

type WavEndian = byteorder::LittleEndian;

pub trait ReadPrimitives<T, E> {
    fn from_bytes(&mut self) -> Result<T, E>;
}

impl<const N: usize, R: Read> ReadPrimitives<[u8; N], std::io::Error> for R {
    fn from_bytes(&mut self) -> Result<[u8; N], std::io::Error> {
        let mut bytes: [u8; N] = [0; N];
        self.read(&mut bytes)?;

        Ok(bytes)
    }
}

impl<R: Read> ReadPrimitives<u8, std::io::Error> for R {
    fn from_bytes(&mut self) -> Result<u8, std::io::Error> {
        self.read_u8()
    }
}

impl<R: Read> ReadPrimitives<u32, std::io::Error> for R {
    fn from_bytes(&mut self) -> Result<u32, std::io::Error> {
        self.read_u32::<WavEndian>()
    }
}

impl<R: Read> ReadPrimitives<u16, std::io::Error> for R {
    fn from_bytes(&mut self) -> Result<u16, std::io::Error> {
        self.read_u16::<WavEndian>()
    }
}
