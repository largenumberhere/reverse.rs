use byteorder::ReadBytesExt;
use std::io::Read;

type WavEndian = byteorder::LittleEndian;
// A convent wrapper for byteorder's methods that allows a
//  single line to do all the heavy lifting in the StructFromBytes derive macro.
// Type-inference is a wonderful thing!
pub trait ReadPrimitive<T, E> {
    fn read_primitive(&mut self) -> Result<T, E>;
}

impl<const N: usize, R: Read> ReadPrimitive<[u8; N], std::io::Error> for R {
    fn read_primitive(&mut self) -> Result<[u8; N], std::io::Error> {
        let mut bytes: [u8; N] = [0; N];
        let qty = self.read(&mut bytes)?;
        if qty != N{
            panic!("Bytes read '{}' is not equal to array size '{}'", qty, N);
        }
        Ok(bytes)
    }
}

impl<R: Read> ReadPrimitive<u8, std::io::Error> for R {
    fn read_primitive(&mut self) -> Result<u8, std::io::Error> {
        self.read_u8()
    }
}

impl<R: Read> ReadPrimitive<u32, std::io::Error> for R {
    fn read_primitive(&mut self) -> Result<u32, std::io::Error> {
        self.read_u32::<WavEndian>()
    }
}

impl<R: Read> ReadPrimitive<u16, std::io::Error> for R {
    fn read_primitive(&mut self) -> Result<u16, std::io::Error> {
        self.read_u16::<WavEndian>()
    }
}
